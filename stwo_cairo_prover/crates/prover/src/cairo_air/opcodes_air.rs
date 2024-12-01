use itertools::{chain, Itertools};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo_prover::constraint_framework::TraceLocationAllocator;
use stwo_prover::core::air::ComponentProver;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::pcs::{TreeBuilder, TreeVec};
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use super::air::CairoInteractionElements;
use crate::components::{
    add_ap_opcode_is_imm_f_op1_base_fp_f, add_ap_opcode_is_imm_f_op1_base_fp_t,
    add_ap_opcode_is_imm_t_op1_base_fp_f, generic_opcode, memory_address_to_id, memory_id_to_big,
    range_check_19, range_check_9_9, ret_opcode, verify_instruction,
};
use crate::input::state_transitions::StateTransitions;

#[derive(Serialize, Deserialize)]
pub struct OpcodeClaim {
    add_ap_f_f: Vec<add_ap_opcode_is_imm_f_op1_base_fp_f::Claim>,
    add_ap_f_t: Vec<add_ap_opcode_is_imm_f_op1_base_fp_t::Claim>,
    add_ap_t_f: Vec<add_ap_opcode_is_imm_t_op1_base_fp_f::Claim>,
    ret: Vec<ret_opcode::Claim>,
    generic: Vec<generic_opcode::Claim>,
}
impl OpcodeClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.add_ap_f_f.iter().for_each(|c| c.mix_into(channel));
        self.add_ap_f_t.iter().for_each(|c| c.mix_into(channel));
        self.add_ap_t_f.iter().for_each(|c| c.mix_into(channel));
        self.ret.iter().for_each(|c| c.mix_into(channel));
        self.generic.iter().for_each(|c| c.mix_into(channel));
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols(chain!(
            self.add_ap_f_f.iter().map(|c| c.log_sizes()),
            self.add_ap_f_t.iter().map(|c| c.log_sizes()),
            self.add_ap_t_f.iter().map(|c| c.log_sizes()),
            self.generic.iter().map(|c| c.log_sizes()),
            self.ret.iter().map(|c| c.log_sizes()),
        ))
    }
}

pub struct OpcodesClaimGenerator {
    add_ap_f_f: Vec<add_ap_opcode_is_imm_f_op1_base_fp_f::ClaimGenerator>,
    add_ap_f_t: Vec<add_ap_opcode_is_imm_f_op1_base_fp_t::ClaimGenerator>,
    add_ap_t_f: Vec<add_ap_opcode_is_imm_t_op1_base_fp_f::ClaimGenerator>,
    generic: Vec<generic_opcode::ClaimGenerator>,
    ret: Vec<ret_opcode::ClaimGenerator>,
}
impl OpcodesClaimGenerator {
    pub fn new(input: StateTransitions) -> Self {
        // TODO(Ohad): decide split sizes for opcode traces.
        let mut add_ap_f_f = vec![];
        let mut add_ap_f_t = vec![];
        let mut add_ap_t_f = vec![];
        let mut generic = vec![];
        let mut ret = vec![];
        if !input
            .casm_states_by_opcode
            .add_ap_opcode_is_imm_f_op1_base_fp_f
            .is_empty()
        {
            add_ap_f_f.push(add_ap_opcode_is_imm_f_op1_base_fp_f::ClaimGenerator::new(
                input
                    .casm_states_by_opcode
                    .add_ap_opcode_is_imm_f_op1_base_fp_f,
            ));
        }
        if !input
            .casm_states_by_opcode
            .add_ap_opcode_is_imm_f_op1_base_fp_t
            .is_empty()
        {
            add_ap_f_t.push(add_ap_opcode_is_imm_f_op1_base_fp_t::ClaimGenerator::new(
                input
                    .casm_states_by_opcode
                    .add_ap_opcode_is_imm_f_op1_base_fp_t,
            ));
        }
        if !input
            .casm_states_by_opcode
            .add_ap_opcode_is_imm_t_op1_base_fp_f
            .is_empty()
        {
            add_ap_t_f.push(add_ap_opcode_is_imm_t_op1_base_fp_f::ClaimGenerator::new(
                input
                    .casm_states_by_opcode
                    .add_ap_opcode_is_imm_t_op1_base_fp_f,
            ));
        }
        if !input.casm_states_by_opcode.generic_opcode.is_empty() {
            generic.push(generic_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.generic_opcode,
            ));
        }
        if !input.casm_states_by_opcode.ret_opcode.is_empty() {
            ret.push(ret_opcode::ClaimGenerator::new(
                input.casm_states_by_opcode.ret_opcode,
            ));
        }
        Self {
            add_ap_f_f,
            add_ap_f_t,
            add_ap_t_f,
            ret,
            generic,
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        memory_address_to_id_trace_generator: &mut memory_address_to_id::ClaimGenerator,
        memory_id_to_value_trace_generator: &mut memory_id_to_big::ClaimGenerator,
        range_check_19_trace_generator: &mut range_check_19::ClaimGenerator,
        range_check_9_9_trace_generator: &mut range_check_9_9::ClaimGenerator,
        verify_instruction_trace_generator: &mut verify_instruction::ClaimGenerator,
    ) -> (OpcodeClaim, OpcodesInteractionClaimGenerator) {
        let (add_ap_f_f_claims, add_ap_f_f_interaction_gens) = self
            .add_ap_f_f
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (add_ap_f_t_claims, add_ap_f_t_interaction_gens) = self
            .add_ap_f_t
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (add_ap_t_f_claims, add_ap_t_f_interaction_gens) = self
            .add_ap_t_f
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (generic_opcode_claims, generic_opcode_interaction_gens) = self
            .generic
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    range_check_19_trace_generator,
                    range_check_9_9_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (ret_claims, ret_interaction_gens) = self
            .ret
            .into_iter()
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_instruction_trace_generator,
                )
            })
            .unzip();
        (
            OpcodeClaim {
                add_ap_f_f: add_ap_f_f_claims,
                add_ap_f_t: add_ap_f_t_claims,
                add_ap_t_f: add_ap_t_f_claims,
                generic: generic_opcode_claims,
                ret: ret_claims,
            },
            OpcodesInteractionClaimGenerator {
                add_ap_f_f: add_ap_f_f_interaction_gens,
                add_ap_f_t: add_ap_f_t_interaction_gens,
                add_ap_t_f: add_ap_t_f_interaction_gens,
                generic_opcode_interaction_gens,
                ret_interaction_gens,
            },
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct OpcodeInteractionClaim {
    add_ap_f_f: Vec<add_ap_opcode_is_imm_f_op1_base_fp_f::InteractionClaim>,
    add_ap_f_t: Vec<add_ap_opcode_is_imm_f_op1_base_fp_t::InteractionClaim>,
    add_ap_t_f: Vec<add_ap_opcode_is_imm_t_op1_base_fp_f::InteractionClaim>,
    generic: Vec<generic_opcode::InteractionClaim>,
    ret: Vec<ret_opcode::InteractionClaim>,
}
impl OpcodeInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.add_ap_f_f.iter().for_each(|c| c.mix_into(channel));
        self.add_ap_f_t.iter().for_each(|c| c.mix_into(channel));
        self.add_ap_t_f.iter().for_each(|c| c.mix_into(channel));
        self.generic.iter().for_each(|c| c.mix_into(channel));
        self.ret.iter().for_each(|c| c.mix_into(channel));
    }

    pub fn sum(&self) -> SecureField {
        let mut sum = QM31::zero();
        for interaction_claim in &self.add_ap_f_f {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.add_ap_f_t {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.add_ap_t_f {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.generic {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        for interaction_claim in &self.ret {
            let (total_sum, claimed_sum) = interaction_claim.logup_sums;
            sum += match claimed_sum {
                Some((claimed_sum, ..)) => claimed_sum,
                None => total_sum,
            };
        }
        sum
    }
}

pub struct OpcodesInteractionClaimGenerator {
    add_ap_f_f: Vec<add_ap_opcode_is_imm_f_op1_base_fp_f::InteractionClaimGenerator>,
    add_ap_f_t: Vec<add_ap_opcode_is_imm_f_op1_base_fp_t::InteractionClaimGenerator>,
    add_ap_t_f: Vec<add_ap_opcode_is_imm_t_op1_base_fp_f::InteractionClaimGenerator>,
    generic_opcode_interaction_gens: Vec<generic_opcode::InteractionClaimGenerator>,
    ret_interaction_gens: Vec<ret_opcode::InteractionClaimGenerator>,
}
impl OpcodesInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        interaction_elements: &CairoInteractionElements,
    ) -> OpcodeInteractionClaim {
        let add_ap_f_f_interaction_claims = self
            .add_ap_f_f
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let add_ap_f_t_interaction_claims = self
            .add_ap_f_t
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let add_ap_t_f_interaction_claims = self
            .add_ap_t_f
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let generic_opcode_interaction_claims = self
            .generic_opcode_interaction_gens
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.range_check_19,
                    &interaction_elements.range_check_9_9,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        let ret_interaction_claims = self
            .ret_interaction_gens
            .into_iter()
            .map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.opcodes,
                    &interaction_elements.verify_instruction,
                )
            })
            .collect();
        OpcodeInteractionClaim {
            add_ap_f_f: add_ap_f_f_interaction_claims,
            add_ap_f_t: add_ap_f_t_interaction_claims,
            add_ap_t_f: add_ap_t_f_interaction_claims,
            generic: generic_opcode_interaction_claims,
            ret: ret_interaction_claims,
        }
    }
}

pub struct OpcodeComponents {
    add_ap_f_f: Vec<add_ap_opcode_is_imm_f_op1_base_fp_f::Component>,
    add_ap_f_t: Vec<add_ap_opcode_is_imm_f_op1_base_fp_t::Component>,
    add_ap_t_f: Vec<add_ap_opcode_is_imm_t_op1_base_fp_f::Component>,
    generic: Vec<generic_opcode::Component>,
    ret: Vec<ret_opcode::Component>,
}
impl OpcodeComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &OpcodeClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &OpcodeInteractionClaim,
    ) -> Self {
        let add_ap_f_f_components = claim
            .add_ap_f_f
            .iter()
            .zip(interaction_claim.add_ap_f_f.iter())
            .map(|(&claim, &interaction_claim)| {
                add_ap_opcode_is_imm_f_op1_base_fp_f::Component::new(
                    tree_span_provider,
                    add_ap_opcode_is_imm_f_op1_base_fp_f::Eval {
                        claim,
                        memoryaddresstoid_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memoryidtobig_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verifyinstruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let add_ap_f_t_components = claim
            .add_ap_f_t
            .iter()
            .zip(interaction_claim.add_ap_f_t.iter())
            .map(|(&claim, &interaction_claim)| {
                add_ap_opcode_is_imm_f_op1_base_fp_t::Component::new(
                    tree_span_provider,
                    add_ap_opcode_is_imm_f_op1_base_fp_t::Eval {
                        claim,
                        memoryaddresstoid_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memoryidtobig_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verifyinstruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let add_ap_t_f_components = claim
            .add_ap_t_f
            .iter()
            .zip(interaction_claim.add_ap_t_f.iter())
            .map(|(&claim, &interaction_claim)| {
                add_ap_opcode_is_imm_t_op1_base_fp_f::Component::new(
                    tree_span_provider,
                    add_ap_opcode_is_imm_t_op1_base_fp_f::Eval {
                        claim,
                        memoryaddresstoid_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memoryidtobig_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verifyinstruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let generic_components = claim
            .generic
            .iter()
            .zip(interaction_claim.generic.iter())
            .map(|(&claim, &interaction_claim)| {
                generic_opcode::Component::new(
                    tree_span_provider,
                    generic_opcode::Eval {
                        claim,
                        memoryaddresstoid_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memoryidtobig_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        rangecheck_19_lookup_elements: interaction_elements.range_check_19.clone(),
                        rangecheck_9_9_lookup_elements: interaction_elements
                            .range_check_9_9
                            .clone(),
                        verifyinstruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        let ret_components = claim
            .ret
            .iter()
            .zip(interaction_claim.ret.iter())
            .map(|(&claim, &interaction_claim)| {
                ret_opcode::Component::new(
                    tree_span_provider,
                    ret_opcode::Eval {
                        claim,
                        memoryaddresstoid_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memoryidtobig_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        verifyinstruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                    },
                    interaction_claim.logup_sums,
                )
            })
            .collect_vec();
        Self {
            add_ap_f_f: add_ap_f_f_components,
            add_ap_f_t: add_ap_f_t_components,
            add_ap_t_f: add_ap_t_f_components,
            generic: generic_components,
            ret: ret_components,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        let mut vec: Vec<&dyn ComponentProver<SimdBackend>> = vec![];
        vec.extend(
            self.add_ap_f_f
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.add_ap_f_t
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.add_ap_t_f
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.generic
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.ret
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec
    }
}
