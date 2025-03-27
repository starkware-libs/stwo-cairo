use itertools::Itertools;
use stwo_cairo_adapter::ProverInput;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::fields::m31::M31;
use tracing::{span, Level};

use super::blake_context::{BlakeContextClaimGenerator, BlakeContextInteractionClaimGenerator};
use super::builtins::{BuiltinsClaimGenerator, BuiltinsInteractionClaimGenerator};
use super::opcodes::{OpcodesClaimGenerator, OpcodesInteractionClaimGenerator};
use super::range_checks::{RangeChecksClaimGenerator, RangeChecksInteractionClaimGenerator};
use crate::cairo_air::air::{
    CairoClaim, CairoInteractionClaim, CairoInteractionElements, PublicData,
};
use crate::components::memory::{memory_address_to_id, memory_id_to_big};
use crate::components::utils::TreeBuilder;
use crate::components::{
    verify_bitwise_xor_4, verify_bitwise_xor_7, verify_bitwise_xor_8, verify_bitwise_xor_9,
    verify_instruction,
};
use crate::witness::components::poseidon::{
    PoseidonContextClaimGenerator, PoseidonContextInteractionClaimGenerator,
};

/// Responsible for generating the CairoClaim and writing the trace.
/// NOTE: Order of writing the trace is important, and should be consistent with [`CairoClaim`],
/// [`CairoInteractionClaim`], [`CairoComponents`], [`CairoInteractionElements`].
pub struct CairoClaimGenerator {
    public_data: PublicData,

    opcodes: OpcodesClaimGenerator,

    // Internal components.
    verify_instruction_trace_generator: verify_instruction::ClaimGenerator,
    blake_context_trace_generator: BlakeContextClaimGenerator,
    builtins: BuiltinsClaimGenerator,
    poseidon_context_trace_generator: PoseidonContextClaimGenerator,
    memory_address_to_id_trace_generator: memory_address_to_id::ClaimGenerator,
    memory_id_to_value_trace_generator: memory_id_to_big::ClaimGenerator,
    range_checks_trace_generator: RangeChecksClaimGenerator,
    verify_bitwise_xor_4_trace_generator: verify_bitwise_xor_4::ClaimGenerator,
    verify_bitwise_xor_7_trace_generator: verify_bitwise_xor_7::ClaimGenerator,
    verify_bitwise_xor_8_trace_generator: verify_bitwise_xor_8::ClaimGenerator,
    verify_bitwise_xor_9_trace_generator: verify_bitwise_xor_9::ClaimGenerator,
    // ...
}
impl CairoClaimGenerator {
    pub fn new(input: ProverInput) -> Self {
        let initial_state = input.state_transitions.initial_state;
        let final_state = input.state_transitions.final_state;
        let opcodes = OpcodesClaimGenerator::new(input.state_transitions);
        let verify_instruction_trace_generator =
            verify_instruction::ClaimGenerator::new(input.instruction_by_pc);
        let builtins = BuiltinsClaimGenerator::new(input.builtins_segments);
        let poseidon_context_trace_generator = PoseidonContextClaimGenerator::new();
        let memory_address_to_id_trace_generator =
            memory_address_to_id::ClaimGenerator::new(&input.memory);
        let memory_id_to_value_trace_generator =
            memory_id_to_big::ClaimGenerator::new(&input.memory);
        let range_checks_trace_generator = RangeChecksClaimGenerator::new();
        let verify_bitwise_xor_4_trace_generator = verify_bitwise_xor_4::ClaimGenerator::new();
        let verify_bitwise_xor_7_trace_generator = verify_bitwise_xor_7::ClaimGenerator::new();
        let verify_bitwise_xor_8_trace_generator = verify_bitwise_xor_8::ClaimGenerator::new();
        let verify_bitwise_xor_9_trace_generator = verify_bitwise_xor_9::ClaimGenerator::new();

        // Yield public memory.
        for addr in input
            .public_memory_addresses
            .iter()
            .copied()
            .map(M31::from_u32_unchecked)
        {
            let id = memory_address_to_id_trace_generator.get_id(addr);
            memory_address_to_id_trace_generator.add_input(&addr);
            memory_id_to_value_trace_generator.add_input(&id);
        }

        // Public data.
        let public_memory = input
            .public_memory_addresses
            .iter()
            .copied()
            .map(|addr| {
                let id = input.memory.get_raw_id(addr);
                (addr, id, input.memory.get(addr).as_u256())
            })
            .collect_vec();
        let public_data = PublicData {
            public_memory,
            initial_state,
            final_state,
        };

        let blake_context_trace_generator = BlakeContextClaimGenerator::new(input.memory);

        Self {
            public_data,
            opcodes,
            verify_instruction_trace_generator,
            blake_context_trace_generator,
            builtins,
            poseidon_context_trace_generator,
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            range_checks_trace_generator,
            verify_bitwise_xor_4_trace_generator,
            verify_bitwise_xor_7_trace_generator,
            verify_bitwise_xor_8_trace_generator,
            verify_bitwise_xor_9_trace_generator,
        }
    }

    pub fn write_trace(
        mut self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
    ) -> (CairoClaim, CairoInteractionClaimGenerator) {
        let span = span!(Level::INFO, "write opcode trace").entered();
        let (opcodes_claim, opcodes_interaction_gen) = self.opcodes.write_trace(
            tree_builder,
            &mut self.blake_context_trace_generator,
            &self.memory_address_to_id_trace_generator,
            &self.memory_id_to_value_trace_generator,
            &self.range_checks_trace_generator,
            &self.verify_instruction_trace_generator,
            &mut self.verify_bitwise_xor_8_trace_generator,
        );
        span.exit();
        let span = span!(Level::INFO, "internal component trace").entered();
        let (verify_instruction_claim, verify_instruction_interaction_gen) =
            self.verify_instruction_trace_generator.write_trace(
                tree_builder,
                &self.memory_address_to_id_trace_generator,
                &self.memory_id_to_value_trace_generator,
                &self.range_checks_trace_generator.rc_4_3_trace_generator,
                &self.range_checks_trace_generator.rc_7_2_5_trace_generator,
            );
        let (blake_context_claim, blake_context_interaction_gen) =
            self.blake_context_trace_generator.write_trace(
                tree_builder,
                &self.memory_address_to_id_trace_generator,
                &self.memory_id_to_value_trace_generator,
                &self.range_checks_trace_generator,
                &self.verify_bitwise_xor_4_trace_generator,
                &self.verify_bitwise_xor_7_trace_generator,
                &self.verify_bitwise_xor_8_trace_generator,
                &self.verify_bitwise_xor_9_trace_generator,
            );
        let (builtins_claim, builtins_interaction_gen) = self.builtins.write_trace(
            tree_builder,
            &self.memory_address_to_id_trace_generator,
            &self.memory_id_to_value_trace_generator,
            &mut self.poseidon_context_trace_generator,
            &self.range_checks_trace_generator.rc_6_trace_generator,
            &self.range_checks_trace_generator.rc_12_trace_generator,
            &self.range_checks_trace_generator.rc_18_trace_generator,
            &self.range_checks_trace_generator.rc_4_4_trace_generator,
            &self.range_checks_trace_generator.rc_3_6_6_3_trace_generator,
            &self.range_checks_trace_generator.rc_4_4_4_4_trace_generator,
            &self
                .range_checks_trace_generator
                .rc_3_3_3_3_3_trace_generator,
            &self.verify_bitwise_xor_9_trace_generator,
        );
        let (poseidon_context_claim, poseidon_context_interaction_gen) = self
            .poseidon_context_trace_generator
            .write_trace(tree_builder, &self.range_checks_trace_generator);
        let (memory_address_to_id_claim, memory_address_to_id_interaction_gen) = self
            .memory_address_to_id_trace_generator
            .write_trace(tree_builder);
        let (memory_id_to_value_claim, memory_id_to_value_interaction_gen) =
            self.memory_id_to_value_trace_generator.write_trace(
                tree_builder,
                &self.range_checks_trace_generator.rc_9_9_trace_generator,
            );
        let (range_checks_claim, range_checks_interaction_gen) =
            self.range_checks_trace_generator.write_trace(tree_builder);
        let (verify_bitwise_xor_4_claim, verify_bitwise_xor_4_interaction_gen) = self
            .verify_bitwise_xor_4_trace_generator
            .write_trace(tree_builder);
        let (verify_bitwise_xor_7_claim, verify_bitwise_xor_7_interaction_gen) = self
            .verify_bitwise_xor_7_trace_generator
            .write_trace(tree_builder);
        let (verify_bitwise_xor_8_claim, verify_bitwise_xor_8_interaction_gen) = self
            .verify_bitwise_xor_8_trace_generator
            .write_trace(tree_builder);
        let (verify_bitwise_xor_9_claim, verify_bitwise_xor_9_interaction_gen) = self
            .verify_bitwise_xor_9_trace_generator
            .write_trace(tree_builder);
        span.exit();
        (
            CairoClaim {
                public_data: self.public_data,
                opcodes: opcodes_claim,
                verify_instruction: verify_instruction_claim,
                blake_context: blake_context_claim,
                builtins: builtins_claim,
                poseidon_context: poseidon_context_claim,
                memory_address_to_id: memory_address_to_id_claim,
                memory_id_to_value: memory_id_to_value_claim,
                range_checks: range_checks_claim,
                verify_bitwise_xor_4: verify_bitwise_xor_4_claim,
                verify_bitwise_xor_7: verify_bitwise_xor_7_claim,
                verify_bitwise_xor_8: verify_bitwise_xor_8_claim,
                verify_bitwise_xor_9: verify_bitwise_xor_9_claim,
            },
            CairoInteractionClaimGenerator {
                opcodes_interaction_gen,
                verify_instruction_interaction_gen,
                blake_context_interaction_gen,
                builtins_interaction_gen,
                poseidon_context_interaction_gen,
                memory_address_to_id_interaction_gen,
                memory_id_to_value_interaction_gen,
                range_checks_interaction_gen,
                verify_bitwise_xor_4_interaction_gen,
                verify_bitwise_xor_7_interaction_gen,
                verify_bitwise_xor_8_interaction_gen,
                verify_bitwise_xor_9_interaction_gen,
            },
        )
    }
}

pub struct CairoInteractionClaimGenerator {
    opcodes_interaction_gen: OpcodesInteractionClaimGenerator,
    verify_instruction_interaction_gen: verify_instruction::InteractionClaimGenerator,
    blake_context_interaction_gen: BlakeContextInteractionClaimGenerator,
    builtins_interaction_gen: BuiltinsInteractionClaimGenerator,
    poseidon_context_interaction_gen: PoseidonContextInteractionClaimGenerator,
    memory_address_to_id_interaction_gen: memory_address_to_id::InteractionClaimGenerator,
    memory_id_to_value_interaction_gen: memory_id_to_big::InteractionClaimGenerator,
    range_checks_interaction_gen: RangeChecksInteractionClaimGenerator,
    verify_bitwise_xor_4_interaction_gen: verify_bitwise_xor_4::InteractionClaimGenerator,
    verify_bitwise_xor_7_interaction_gen: verify_bitwise_xor_7::InteractionClaimGenerator,
    verify_bitwise_xor_8_interaction_gen: verify_bitwise_xor_8::InteractionClaimGenerator,
    verify_bitwise_xor_9_interaction_gen: verify_bitwise_xor_9::InteractionClaimGenerator,
    // ...
}
impl CairoInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        interaction_elements: &CairoInteractionElements,
    ) -> CairoInteractionClaim {
        let opcodes_interaction_claims = self
            .opcodes_interaction_gen
            .write_interaction_trace(tree_builder, interaction_elements);
        let verify_instruction_interaction_claim = self
            .verify_instruction_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.range_checks.rc_4_3,
                &interaction_elements.range_checks.rc_7_2_5,
                &interaction_elements.verify_instruction,
            );
        let blake_context_interaction_claim = self
            .blake_context_interaction_gen
            .write_interaction_trace(tree_builder, interaction_elements);
        let builtins_interaction_claims = self
            .builtins_interaction_gen
            .write_interaction_trace(tree_builder, interaction_elements);
        let poseidon_context_interaction_claim = self
            .poseidon_context_interaction_gen
            .write_interaction_trace(tree_builder, interaction_elements);
        let memory_address_to_id_interaction_claim = self
            .memory_address_to_id_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.memory_address_to_id);
        let memory_id_to_value_interaction_claim = self
            .memory_id_to_value_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.range_checks.rc_9_9,
            );

        let range_checks_interaction_claim = self
            .range_checks_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range_checks);
        let verify_bitwise_xor_4_interaction_claim = self
            .verify_bitwise_xor_4_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.verify_bitwise_xor_4);
        let verify_bitwise_xor_7_interaction_claim = self
            .verify_bitwise_xor_7_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.verify_bitwise_xor_7);
        let verify_bitwise_xor_8_interaction_claim = self
            .verify_bitwise_xor_8_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.verify_bitwise_xor_8);
        let verify_bitwise_xor_9_interaction_claim = self
            .verify_bitwise_xor_9_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.verify_bitwise_xor_9);

        CairoInteractionClaim {
            opcodes: opcodes_interaction_claims,
            verify_instruction: verify_instruction_interaction_claim,
            blake_context: blake_context_interaction_claim,
            builtins: builtins_interaction_claims,
            poseidon_context: poseidon_context_interaction_claim,
            memory_address_to_id: memory_address_to_id_interaction_claim,
            memory_id_to_value: memory_id_to_value_interaction_claim,
            range_checks: range_checks_interaction_claim,
            verify_bitwise_xor_4: verify_bitwise_xor_4_interaction_claim,
            verify_bitwise_xor_7: verify_bitwise_xor_7_interaction_claim,
            verify_bitwise_xor_8: verify_bitwise_xor_8_interaction_claim,
            verify_bitwise_xor_9: verify_bitwise_xor_9_interaction_claim,
        }
    }
}
