use itertools::Itertools;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::ComponentProver;
use stwo_constraint_framework::TraceLocationAllocator;

use super::components::display_components;
use crate::components::{
    add_ap_opcode, add_opcode, add_opcode_small, assert_eq_opcode, assert_eq_opcode_double_deref,
    assert_eq_opcode_imm, blake_compress_opcode, call_opcode_abs, call_opcode_rel_imm,
    generic_opcode, jnz_opcode_non_taken, jnz_opcode_taken, jump_opcode_abs,
    jump_opcode_double_deref, jump_opcode_rel, jump_opcode_rel_imm, mul_opcode, mul_opcode_small,
    qm_31_add_mul_opcode, ret_opcode,
};
use crate::relations::CommonLookupElements;

pub struct OpcodeComponents {
    pub add: Vec<add_opcode::Component>,
    pub add_small: Vec<add_opcode_small::Component>,
    pub add_ap: Vec<add_ap_opcode::Component>,
    pub assert_eq: Vec<assert_eq_opcode::Component>,
    pub assert_eq_imm: Vec<assert_eq_opcode_imm::Component>,
    pub assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::Component>,
    pub blake: Vec<blake_compress_opcode::Component>,
    pub call: Vec<call_opcode_abs::Component>,
    pub call_rel_imm: Vec<call_opcode_rel_imm::Component>,
    pub generic: Vec<generic_opcode::Component>,
    pub jnz: Vec<jnz_opcode_non_taken::Component>,
    pub jnz_taken: Vec<jnz_opcode_taken::Component>,
    pub jump: Vec<jump_opcode_abs::Component>,
    pub jump_double_deref: Vec<jump_opcode_double_deref::Component>,
    pub jump_rel: Vec<jump_opcode_rel::Component>,
    pub jump_rel_imm: Vec<jump_opcode_rel_imm::Component>,
    pub mul: Vec<mul_opcode::Component>,
    pub mul_small: Vec<mul_opcode_small::Component>,
    pub qm31: Vec<qm_31_add_mul_opcode::Component>,
    pub ret: Vec<ret_opcode::Component>,
}
impl OpcodeComponents {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        add_claim: &Option<add_opcode::Claim>,
        add_small_claim: &Option<add_opcode_small::Claim>,
        add_ap_claim: &Option<add_ap_opcode::Claim>,
        assert_eq_claim: &Option<assert_eq_opcode::Claim>,
        assert_eq_imm_claim: &Option<assert_eq_opcode_imm::Claim>,
        assert_eq_double_deref_claim: &Option<assert_eq_opcode_double_deref::Claim>,
        blake_claim: &Option<blake_compress_opcode::Claim>,
        call_claim: &Option<call_opcode_abs::Claim>,
        call_rel_imm_claim: &Option<call_opcode_rel_imm::Claim>,
        generic_claim: &Option<generic_opcode::Claim>,
        jnz_claim: &Option<jnz_opcode_non_taken::Claim>,
        jnz_taken_claim: &Option<jnz_opcode_taken::Claim>,
        jump_claim: &Option<jump_opcode_abs::Claim>,
        jump_double_deref_claim: &Option<jump_opcode_double_deref::Claim>,
        jump_rel_claim: &Option<jump_opcode_rel::Claim>,
        jump_rel_imm_claim: &Option<jump_opcode_rel_imm::Claim>,
        mul_claim: &Option<mul_opcode::Claim>,
        mul_small_claim: &Option<mul_opcode_small::Claim>,
        qm31_claim: &Option<qm_31_add_mul_opcode::Claim>,
        ret_claim: &Option<ret_opcode::Claim>,
        common_lookup_elements: &CommonLookupElements,
        add_interaction_claim: &Option<add_opcode::InteractionClaim>,
        add_small_interaction_claim: &Option<add_opcode_small::InteractionClaim>,
        add_ap_interaction_claim: &Option<add_ap_opcode::InteractionClaim>,
        assert_eq_interaction_claim: &Option<assert_eq_opcode::InteractionClaim>,
        assert_eq_imm_interaction_claim: &Option<assert_eq_opcode_imm::InteractionClaim>,
        assert_eq_double_deref_interaction_claim: &Option<
            assert_eq_opcode_double_deref::InteractionClaim,
        >,
        blake_interaction_claim: &Option<blake_compress_opcode::InteractionClaim>,
        call_interaction_claim: &Option<call_opcode_abs::InteractionClaim>,
        call_rel_imm_interaction_claim: &Option<call_opcode_rel_imm::InteractionClaim>,
        generic_interaction_claim: &Option<generic_opcode::InteractionClaim>,
        jnz_interaction_claim: &Option<jnz_opcode_non_taken::InteractionClaim>,
        jnz_taken_interaction_claim: &Option<jnz_opcode_taken::InteractionClaim>,
        jump_interaction_claim: &Option<jump_opcode_abs::InteractionClaim>,
        jump_double_deref_interaction_claim: &Option<jump_opcode_double_deref::InteractionClaim>,
        jump_rel_interaction_claim: &Option<jump_opcode_rel::InteractionClaim>,
        jump_rel_imm_interaction_claim: &Option<jump_opcode_rel_imm::InteractionClaim>,
        mul_interaction_claim: &Option<mul_opcode::InteractionClaim>,
        mul_small_interaction_claim: &Option<mul_opcode_small::InteractionClaim>,
        qm31_interaction_claim: &Option<qm_31_add_mul_opcode::InteractionClaim>,
        ret_interaction_claim: &Option<ret_opcode::InteractionClaim>,
    ) -> Self {
        let add_components = add_claim
            .iter()
            .zip(add_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                add_opcode::Component::new(
                    tree_span_provider,
                    add_opcode::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let add_small_components = add_small_claim
            .iter()
            .zip(add_small_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                add_opcode_small::Component::new(
                    tree_span_provider,
                    add_opcode_small::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let add_ap_components = add_ap_claim
            .iter()
            .zip(add_ap_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                add_ap_opcode::Component::new(
                    tree_span_provider,
                    add_ap_opcode::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let assert_eq_components = assert_eq_claim
            .iter()
            .zip(assert_eq_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                assert_eq_opcode::Component::new(
                    tree_span_provider,
                    assert_eq_opcode::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let assert_eq_imm_components = assert_eq_imm_claim
            .iter()
            .zip(assert_eq_imm_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                assert_eq_opcode_imm::Component::new(
                    tree_span_provider,
                    assert_eq_opcode_imm::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let assert_eq_double_deref_components = assert_eq_double_deref_claim
            .iter()
            .zip(assert_eq_double_deref_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                assert_eq_opcode_double_deref::Component::new(
                    tree_span_provider,
                    assert_eq_opcode_double_deref::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let blake_components = blake_claim
            .iter()
            .zip(blake_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                blake_compress_opcode::Component::new(
                    tree_span_provider,
                    blake_compress_opcode::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let call_components = call_claim
            .iter()
            .zip(call_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                call_opcode_abs::Component::new(
                    tree_span_provider,
                    call_opcode_abs::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let call_rel_imm_components = call_rel_imm_claim
            .iter()
            .zip(call_rel_imm_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                call_opcode_rel_imm::Component::new(
                    tree_span_provider,
                    call_opcode_rel_imm::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let generic_components = generic_claim
            .iter()
            .zip(generic_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                generic_opcode::Component::new(
                    tree_span_provider,
                    generic_opcode::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let jnz_components = jnz_claim
            .iter()
            .zip(jnz_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                jnz_opcode_non_taken::Component::new(
                    tree_span_provider,
                    jnz_opcode_non_taken::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let jnz_taken_components = jnz_taken_claim
            .iter()
            .zip(jnz_taken_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                jnz_opcode_taken::Component::new(
                    tree_span_provider,
                    jnz_opcode_taken::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let jump_components = jump_claim
            .iter()
            .zip(jump_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                jump_opcode_abs::Component::new(
                    tree_span_provider,
                    jump_opcode_abs::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let jump_double_deref_components = jump_double_deref_claim
            .iter()
            .zip(jump_double_deref_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                jump_opcode_double_deref::Component::new(
                    tree_span_provider,
                    jump_opcode_double_deref::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let jump_rel_components = jump_rel_claim
            .iter()
            .zip(jump_rel_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                jump_opcode_rel::Component::new(
                    tree_span_provider,
                    jump_opcode_rel::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let jump_rel_imm_components = jump_rel_imm_claim
            .iter()
            .zip(jump_rel_imm_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                jump_opcode_rel_imm::Component::new(
                    tree_span_provider,
                    jump_opcode_rel_imm::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let mul_components = mul_claim
            .iter()
            .zip(mul_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                mul_opcode::Component::new(
                    tree_span_provider,
                    mul_opcode::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let mul_small_components = mul_small_claim
            .iter()
            .zip(mul_small_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                mul_opcode_small::Component::new(
                    tree_span_provider,
                    mul_opcode_small::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        let qm31_components = qm31_claim
            .iter()
            .zip(qm31_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                qm_31_add_mul_opcode::Component::new(
                    tree_span_provider,
                    qm_31_add_mul_opcode::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect();
        let ret_components = ret_claim
            .iter()
            .zip(ret_interaction_claim.iter())
            .map(|(&claim, &interaction_claim)| {
                ret_opcode::Component::new(
                    tree_span_provider,
                    ret_opcode::Eval {
                        claim,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            })
            .collect_vec();
        Self {
            add: add_components,
            add_small: add_small_components,
            add_ap: add_ap_components,
            assert_eq: assert_eq_components,
            assert_eq_imm: assert_eq_imm_components,
            assert_eq_double_deref: assert_eq_double_deref_components,
            blake: blake_components,
            call: call_components,
            call_rel_imm: call_rel_imm_components,
            generic: generic_components,
            jnz: jnz_components,
            jnz_taken: jnz_taken_components,
            jump: jump_components,
            jump_double_deref: jump_double_deref_components,
            jump_rel: jump_rel_components,
            jump_rel_imm: jump_rel_imm_components,
            mul: mul_components,
            mul_small: mul_small_components,
            qm31: qm31_components,
            ret: ret_components,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        let mut vec: Vec<&dyn ComponentProver<SimdBackend>> = vec![];
        vec.extend(
            self.add
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.add_small
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.add_ap
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.assert_eq
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.assert_eq_imm
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.assert_eq_double_deref
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.blake
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.call
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.call_rel_imm
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.generic
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jnz
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jnz_taken
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jump
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jump_double_deref
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jump_rel
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.jump_rel_imm
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.mul
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.mul_small
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
        );
        vec.extend(
            self.qm31
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

impl std::fmt::Display for OpcodeComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "add:")?;
        writeln!(f, "{}", display_components(&self.add))?;
        writeln!(f, "add_small:")?;
        writeln!(f, "{}", display_components(&self.add_small))?;
        writeln!(f, "add_ap:")?;
        writeln!(f, "{}", display_components(&self.add_ap))?;
        writeln!(f, "assert_eq:")?;
        writeln!(f, "{}", display_components(&self.assert_eq))?;
        writeln!(f, "assert_eq_imm:")?;
        writeln!(f, "{}", display_components(&self.assert_eq_imm))?;
        writeln!(f, "assert_eq_double_deref:")?;
        writeln!(f, "{}", display_components(&self.assert_eq_double_deref))?;
        writeln!(f, "blake:")?;
        writeln!(f, "{}", display_components(&self.blake))?;
        writeln!(f, "call:")?;
        writeln!(f, "{}", display_components(&self.call))?;
        writeln!(f, "call_rel_imm:")?;
        writeln!(f, "{}", display_components(&self.call_rel_imm))?;
        writeln!(f, "generic:")?;
        writeln!(f, "{}", display_components(&self.generic))?;
        writeln!(f, "jnz:")?;
        writeln!(f, "{}", display_components(&self.jnz))?;
        writeln!(f, "jnz_taken:")?;
        writeln!(f, "{}", display_components(&self.jnz_taken))?;
        writeln!(f, "jump:")?;
        writeln!(f, "{}", display_components(&self.jump))?;
        writeln!(f, "jump_double_deref:")?;
        writeln!(f, "{}", display_components(&self.jump_double_deref))?;
        writeln!(f, "jump_rel:")?;
        writeln!(f, "{}", display_components(&self.jump_rel))?;
        writeln!(f, "jump_rel_imm:")?;
        writeln!(f, "{}", display_components(&self.jump_rel_imm))?;
        writeln!(f, "mul:")?;
        writeln!(f, "{}", display_components(&self.mul))?;
        writeln!(f, "mul_small:")?;
        writeln!(f, "{}", display_components(&self.mul_small))?;
        writeln!(f, "qm31:")?;
        writeln!(f, "{}", display_components(&self.qm31))?;
        writeln!(f, "ret:")?;
        writeln!(f, "{}", display_components(&self.ret))?;
        Ok(())
    }
}
