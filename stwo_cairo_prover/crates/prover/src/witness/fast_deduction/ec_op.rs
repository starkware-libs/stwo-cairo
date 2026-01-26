use num_traits::One;
use starknet_types_core::curve::ProjectivePoint;
use stwo::prover::backend::simd::conversion::{Pack, Unpack};
use stwo::prover::backend::simd::m31::PackedM31;
use stwo_cairo_common::prover_types::cpu::{Felt252, Felt252Width27, M31};
use stwo_cairo_common::prover_types::simd::{PackedFelt252, PackedFelt252Width27};

type PartialEcMulGenericState = (Felt252Width27, [Felt252; 2], [Felt252; 2], M31);
type PackedPartialEcMulGenericState = (
    PackedFelt252Width27,
    [PackedFelt252; 2],
    [PackedFelt252; 2],
    PackedM31,
);

#[derive(Debug)]
pub struct PartialEcMulGeneric {}
impl PartialEcMulGeneric {
    pub fn deduce_output(
        chain: M31,
        round: M31,
        (m, q, accumulator, counter): PartialEcMulGenericState,
    ) -> (M31, M31, PartialEcMulGenericState) {
        let accumulator_point =
            ProjectivePoint::from_affine(accumulator[0].into(), accumulator[1].into())
                .expect("The accumulator should contain a curve point");
        let q_point = ProjectivePoint::from_affine(q[0].into(), q[1].into())
            .expect("The accumulator should contain a curve point");

        let bit = m.limbs[0] & 1;

        let new_accumulator = if bit == 1 {
            let new_accumulator = (accumulator_point + q_point.clone())
                .to_affine()
                .expect("New accumulator is not infinity");
            [new_accumulator.x().into(), new_accumulator.y().into()]
        } else {
            accumulator
        };

        let double_q_point = q_point
            .double()
            .to_affine()
            .expect("Double q is not infinity");
        let new_q = [double_q_point.x().into(), double_q_point.y().into()];

        let (new_m, new_counter) = if counter.0 == 0 {
            (
                Felt252Width27 {
                    limbs: [
                        m.limbs[0] >> 27 | (m.limbs[1] << 37),
                        m.limbs[1] >> 27 | (m.limbs[2] << 37),
                        m.limbs[2] >> 27 | (m.limbs[3] << 37),
                        m.limbs[3] >> 27,
                    ],
                },
                M31::from_u32_unchecked(26),
            )
        } else {
            (
                Felt252Width27 {
                    limbs: [
                        ((m.limbs[0] & 0x7ff_ffff) >> 1) | (m.limbs[0] & 0xffff_ffff_f800_0000),
                        m.limbs[1],
                        m.limbs[2],
                        m.limbs[3],
                    ],
                },
                M31::from_u32_unchecked(counter.0 - 1),
            )
        };

        (
            chain,
            round + M31::one(),
            (new_m, new_q, new_accumulator, new_counter),
        )
    }
}

pub struct PackedPartialEcMulGeneric {}
impl PackedPartialEcMulGeneric {
    pub fn deduce_output(
        input: (PackedM31, PackedM31, PackedPartialEcMulGenericState),
    ) -> (PackedM31, PackedM31, PackedPartialEcMulGenericState) {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(
            unpacked_inputs.map(|(chain, round, state)| {
                PartialEcMulGeneric::deduce_output(chain, round, state)
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use starknet_curve::curve_params::{PEDERSEN_P1, PEDERSEN_P3};
    use starknet_types_core::curve::ProjectivePoint;
    use stwo_cairo_common::prover_types::cpu::{Felt252Width27, M31};

    use super::PartialEcMulGeneric;

    #[test]
    fn test_deduce_output() {
        let chain = M31::from_u32_unchecked(1234);
        let round = M31::from_u32_unchecked(0);
        let counter = M31::from_u32_unchecked(26);
        let m = Felt252Width27 {
            limbs: [
                0xfedcba98_76543211,
                0x01234567_89abcdef,
                0xfedcba98_76543210,
                0x01234567_89abcdef,
            ],
        };
        let q = [PEDERSEN_P1.x().into(), PEDERSEN_P1.y().into()];
        let accumulator = [PEDERSEN_P3.x().into(), PEDERSEN_P3.y().into()];

        let (chain2, round2, (m2, q2, accumulator2, counter2)) =
            PartialEcMulGeneric::deduce_output(chain, round, (m, q, accumulator, counter));

        assert_eq!(chain2, chain);
        assert_eq!(round2, round + M31::from_u32_unchecked(1));
        assert_eq!(counter2, M31::from_u32_unchecked(25));

        assert_eq!(
            m2,
            Felt252Width27 {
                limbs: [
                    0xfedcba98_732a1908,
                    0x01234567_89abcdef,
                    0xfedcba98_76543210,
                    0x01234567_89abcdef,
                ]
            }
        );

        let p3 = ProjectivePoint::from_affine(PEDERSEN_P3.x(), PEDERSEN_P3.y()).unwrap();
        let p1 = ProjectivePoint::from_affine(PEDERSEN_P1.x(), PEDERSEN_P1.y()).unwrap();

        let expected_q2 = p1.double().to_affine().unwrap();
        assert_eq!(q2, [expected_q2.x().into(), expected_q2.y().into()]);

        let expected_accumulator2 = (p1.clone() + p3).to_affine().unwrap();
        assert_eq!(
            accumulator2,
            [
                expected_accumulator2.x().into(),
                expected_accumulator2.y().into()
            ]
        );

        let (chain3, round3, (m3, q3, accumulator3, counter3)) =
            PartialEcMulGeneric::deduce_output(chain2, round2, (m2, q2, accumulator2, counter2));

        assert_eq!(chain3, chain2);
        assert_eq!(round3, round2 + M31::from_u32_unchecked(1));
        assert_eq!(counter3, M31::from_u32_unchecked(24));

        assert_eq!(
            m3,
            Felt252Width27 {
                limbs: [
                    0xfedcba98_71950c84,
                    0x01234567_89abcdef,
                    0xfedcba98_76543210,
                    0x01234567_89abcdef,
                ]
            }
        );

        let expected_q3 = p1.double().double().to_affine().unwrap();
        assert_eq!(q3, [expected_q3.x().into(), expected_q3.y().into()]);
        assert_eq!(accumulator3, accumulator2);

        let m_spec = Felt252Width27 {
            limbs: [
                0xfedcba98_78000001,
                0x01234567_89abcdef,
                0xfedcba98_76543210,
                0x01234567_89abcdef,
            ],
        };

        let (chain4, round4, (m_spec2, q4, accumulator4, counter4)) =
            PartialEcMulGeneric::deduce_output(
                chain,
                M31::from_u32_unchecked(26),
                (m_spec, q, accumulator, M31::from_u32_unchecked(0)),
            );

        assert_eq!(chain4, chain);
        assert_eq!(round4, M31::from_u32_unchecked(27));
        assert_eq!(counter4, M31::from_u32_unchecked(26));

        assert_eq!(
            m_spec2,
            Felt252Width27 {
                limbs: [
                    0x3579bdff_db97530f,
                    0xca864200_2468acf1,
                    0x3579bdff_db97530e,
                    0x2468acf1,
                ]
            }
        );

        assert_eq!(q4, q2);
        assert_eq!(accumulator4, accumulator2);
    }
}
