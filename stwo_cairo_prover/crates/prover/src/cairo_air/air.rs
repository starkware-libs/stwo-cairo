use itertools::{chain, Itertools};
use stwo_prover::core::air::{Component, ComponentProver};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::pcs::{TreeBuilder, TreeVec};
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use crate::components::memory::{
    MemoryAndRangeElements, MemoryClaim, MemoryComponents, MemoryInteractionClaim,
    MemoryInteractionProver, MemoryProver,
};
use crate::components::opcode::{
    CpuRangeProvers, OpcodeElements, OpcodeGenContext, OpcodesClaim, OpcodesComponents,
    OpcodesInteractionClaim, OpcodesInteractionProvers, OpcodesProvers,
};
use crate::components::range_check::RangeProver;
use crate::input::instructions::VmState;
use crate::input::{CairoInput, SegmentAddrs};

pub struct CairoClaim {
    // Common claim values.
    pub public_memory: Vec<(u32, [u32; 8])>,
    pub initial_state: VmState,
    pub final_state: VmState,
    pub range_check: SegmentAddrs,

    // Opcodes.
    pub opcodes: OpcodesClaim,

    // Memory.
    pub mem: MemoryClaim,
}
impl CairoClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        // TODO(spapini): Add common values.
        self.mem.mix_into(channel);
        self.opcodes.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols([self.mem.log_sizes(), self.opcodes.log_sizes()].into_iter())
    }
}

pub struct CairoInteractionElements {
    pub opcode: OpcodeElements,
    // ...
}
impl CairoInteractionElements {
    pub fn draw(channel: &mut impl Channel) -> CairoInteractionElements {
        CairoInteractionElements {
            opcode: OpcodeElements::draw(channel),
        }
    }
}

pub struct CairoInteractionClaim {
    pub opcodes: OpcodesInteractionClaim,
    pub mem: MemoryInteractionClaim,
}
impl CairoInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.opcodes.mix_into(channel);
        self.mem.mix_into(channel);
    }
}

pub struct CairoProvers {
    pub range: CpuRangeProvers,
    pub opcodes: OpcodesProvers,
    pub mem: MemoryProver,

    // Input data,
    pub input: CairoInput,
}
impl CairoProvers {
    pub fn new(mut input: CairoInput) -> Self {
        let range = CpuRangeProvers {
            range2: RangeProver,
            range3: RangeProver,
        };
        let opcodes = OpcodesProvers::new(std::mem::take(&mut input.instructions));
        let mem = MemoryProver::new(&input.mem);
        Self {
            range,
            opcodes,
            mem,
            input,
        }
    }
    pub fn generate(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
    ) -> (CairoClaim, CairoInteractionProvers) {
        // Extract public memory.
        let public_memory = self
            .input
            .public_mem_addresses
            .iter()
            .copied()
            .map(|a| (a, self.input.mem.get(a).as_u256()))
            .collect_vec();
        let initial_state = self.input.instructions.initial_state;
        let final_state = self.input.instructions.final_state;

        // // Add public memory.
        // for addr in &input.public_mem_addresses {
        //     opcode_ctx.addr_to_id.add_inputs(*addr);
        // }

        let (opcodes_claim, opcodes_provers) = self.opcodes.generate(
            OpcodeGenContext {
                mem_builder: &mut self.mem,
                mem: &self.input.mem,
                range: &mut self.range,
            },
            tree_builder,
        );
        let (memory_claim, mem) = self.mem.generate(self.input.mem, tree_builder);

        let claim = CairoClaim {
            public_memory,
            initial_state,
            final_state,
            range_check: self.input.range_check,
            opcodes: opcodes_claim,
            mem: memory_claim,
        };
        (
            claim,
            CairoInteractionProvers {
                opcodes: opcodes_provers,
                mem,
            },
        )
    }
}

pub struct CairoInteractionProvers {
    pub opcodes: OpcodesInteractionProvers,
    pub mem: MemoryInteractionProver,
}
impl CairoInteractionProvers {
    pub fn generate(
        self,
        elements: &CairoInteractionElements,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
    ) -> CairoInteractionClaim {
        let memory_and_range_elements = MemoryAndRangeElements {
            mem: elements.opcode.mem.clone(),
            range: elements.opcode.range.clone(),
        };

        let opcodes_interaction_claim = self.opcodes.generate(&elements.opcode, tree_builder);
        let mem_interaction_claim = self.mem.generate(tree_builder, &memory_and_range_elements);

        // Commit to the interaction claim and the interaction trace.
        CairoInteractionClaim {
            opcodes: opcodes_interaction_claim,
            mem: mem_interaction_claim,
        }
    }
}

pub struct CairoComponents {
    pub opcodes: OpcodesComponents,
    pub mem: MemoryComponents,
}

impl CairoComponents {
    pub fn new(
        cairo_claim: &CairoClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &CairoInteractionClaim,
    ) -> Self {
        let opcodes = OpcodesComponents::new(
            &cairo_claim.opcodes,
            &interaction_elements.opcode,
            &interaction_claim.opcodes,
        );
        let mem_and_range_els = MemoryAndRangeElements {
            mem: interaction_elements.opcode.mem.clone(),
            range: interaction_elements.opcode.range.clone(),
        };
        let mem =
            MemoryComponents::new(&cairo_claim.mem, &mem_and_range_els, &interaction_claim.mem);
        Self { opcodes, mem }
    }

    pub fn provers(&self) -> impl Iterator<Item = &dyn ComponentProver<SimdBackend>> {
        chain![self.opcodes.provers(), self.mem.provers()]
    }

    pub fn components(&self) -> impl Iterator<Item = &dyn Component> {
        chain![self.opcodes.components(), self.mem.components()]
    }
}
