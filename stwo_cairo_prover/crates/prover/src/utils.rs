use cairo_air::air::CairoComponents;
use cairo_air::blake::air::Components as BlakeContextComponents;
use cairo_air::builtins_air::BuiltinComponents;
use cairo_air::opcodes_air::OpcodeComponents;
use cairo_air::pedersen::air::Components as PedersenContextComponents;
use cairo_air::poseidon::air::Components as PoseidonContextComponents;
use itertools::chain;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::ComponentProver;

pub fn cairo_provers(components: &CairoComponents) -> Vec<&dyn ComponentProver<SimdBackend>> {
    chain!(
        opcode_provers(&components.opcodes),
        [&components.verify_instruction as &dyn ComponentProver<SimdBackend>,],
        blake_context_provers(&components.blake_context.components),
        builtin_provers(&components.builtins),
        pedersen_context_provers(&components.pedersen_context.components),
        poseidon_context_provers(&components.poseidon_context.components),
        [&components.memory_address_to_id as &dyn ComponentProver<SimdBackend>,],
        components
            .memory_id_to_value
            .0
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
        [&components.memory_id_to_value.1 as &dyn ComponentProver<SimdBackend>,],
        [
            &components.range_checks.rc_6 as &dyn ComponentProver<SimdBackend>,
            &components.range_checks.rc_8 as &dyn ComponentProver<SimdBackend>,
            &components.range_checks.rc_11 as &dyn ComponentProver<SimdBackend>,
            &components.range_checks.rc_12 as &dyn ComponentProver<SimdBackend>,
            &components.range_checks.rc_18 as &dyn ComponentProver<SimdBackend>,
            &components.range_checks.rc_20 as &dyn ComponentProver<SimdBackend>,
            &components.range_checks.rc_4_3 as &dyn ComponentProver<SimdBackend>,
            &components.range_checks.rc_4_4 as &dyn ComponentProver<SimdBackend>,
            &components.range_checks.rc_9_9 as &dyn ComponentProver<SimdBackend>,
            &components.range_checks.rc_7_2_5 as &dyn ComponentProver<SimdBackend>,
            &components.range_checks.rc_3_6_6_3 as &dyn ComponentProver<SimdBackend>,
            &components.range_checks.rc_4_4_4_4 as &dyn ComponentProver<SimdBackend>,
            &components.range_checks.rc_3_3_3_3_3 as &dyn ComponentProver<SimdBackend>,
        ],
        [
            &components.verify_bitwise_xor_4 as &dyn ComponentProver<SimdBackend>,
            &components.verify_bitwise_xor_7 as &dyn ComponentProver<SimdBackend>,
            &components.verify_bitwise_xor_8 as &dyn ComponentProver<SimdBackend>,
            &components.verify_bitwise_xor_9 as &dyn ComponentProver<SimdBackend>,
        ]
    )
    .collect()
}

fn opcode_provers(components: &OpcodeComponents) -> Vec<&dyn ComponentProver<SimdBackend>> {
    let mut vec: Vec<&dyn ComponentProver<SimdBackend>> = vec![];
    vec.extend(
        components
            .add
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .add_small
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .add_ap
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .assert_eq
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .assert_eq_imm
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .assert_eq_double_deref
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .blake
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .call
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .call_rel_imm
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .generic
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .jnz
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .jnz_taken
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .jump
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .jump_double_deref
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .jump_rel
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .jump_rel_imm
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .mul
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .mul_small
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .qm31
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec.extend(
        components
            .ret
            .iter()
            .map(|component| component as &dyn ComponentProver<SimdBackend>),
    );
    vec
}

fn builtin_provers(components: &BuiltinComponents) -> Vec<&dyn ComponentProver<SimdBackend>> {
    let mut vec: Vec<&dyn ComponentProver<SimdBackend>> = vec![];
    if let Some(add_mod_builtin) = &components.add_mod_builtin {
        vec.push(add_mod_builtin as &dyn ComponentProver<SimdBackend>);
    }
    if let Some(bitwise_builtin) = &components.bitwise_builtin {
        vec.push(bitwise_builtin as &dyn ComponentProver<SimdBackend>);
    }
    if let Some(mul_mod_builtin) = &components.mul_mod_builtin {
        vec.push(mul_mod_builtin as &dyn ComponentProver<SimdBackend>);
    }
    if let Some(pedersen_builtin) = &components.pedersen_builtin {
        vec.push(pedersen_builtin as &dyn ComponentProver<SimdBackend>);
    }
    if let Some(poseidon_builtin) = &components.poseidon_builtin {
        vec.push(poseidon_builtin as &dyn ComponentProver<SimdBackend>);
    }
    if let Some(range_check_96_builtin) = &components.range_check_96_builtin {
        vec.push(range_check_96_builtin as &dyn ComponentProver<SimdBackend>);
    }
    if let Some(range_check_128_builtin) = &components.range_check_128_builtin {
        vec.push(range_check_128_builtin as &dyn ComponentProver<SimdBackend>);
    }
    vec
}

fn blake_context_provers(
    components: &Option<BlakeContextComponents>,
) -> Vec<&dyn ComponentProver<SimdBackend>> {
    if let Some(components) = components {
        return vec![
            &components.blake_round as &dyn ComponentProver<SimdBackend>,
            &components.blake_g as &dyn ComponentProver<SimdBackend>,
            &components.blake_sigma as &dyn ComponentProver<SimdBackend>,
            &components.triple_xor_32 as &dyn ComponentProver<SimdBackend>,
            &components.verify_bitwise_xor_12 as &dyn ComponentProver<SimdBackend>,
        ];
    }

    vec![]
}

fn pedersen_context_provers(
    components: &Option<PedersenContextComponents>,
) -> Vec<&dyn ComponentProver<SimdBackend>> {
    if let Some(components) = components {
        return vec![
            &components.pedersen_aggregator as &dyn ComponentProver<SimdBackend>,
            &components.partial_ec_mul as &dyn ComponentProver<SimdBackend>,
            &components.pedersen_points_table as &dyn ComponentProver<SimdBackend>,
        ];
    }

    vec![]
}

fn poseidon_context_provers(
    components: &Option<PoseidonContextComponents>,
) -> Vec<&dyn ComponentProver<SimdBackend>> {
    if let Some(components) = components {
        return vec![
            &components.poseidon_aggregator as &dyn ComponentProver<SimdBackend>,
            &components.poseidon_3_partial_rounds_chain as &dyn ComponentProver<SimdBackend>,
            &components.poseidon_full_round_chain as &dyn ComponentProver<SimdBackend>,
            &components.cube_252 as &dyn ComponentProver<SimdBackend>,
            &components.poseidon_round_keys as &dyn ComponentProver<SimdBackend>,
            &components.range_check_252_width_27 as &dyn ComponentProver<SimdBackend>,
        ];
    }

    vec![]
}
