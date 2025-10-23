// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::bitwise_xor_num_bits_8::BitwiseXorNumBits8;
use crate::components::subroutines::read_u_32::ReadU32;
use crate::components::subroutines::split_16_low_part_size_8::Split16LowPartSize8;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct CreateBlakeRoundInput {}

impl CreateBlakeRoundInput {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [create_blake_round_input_input_limb_0, create_blake_round_input_input_limb_1, create_blake_round_input_input_limb_2, create_blake_round_input_input_limb_3]: [E::F; 4],
        low_16_bits_col0: E::F,
        high_16_bits_col1: E::F,
        low_7_ms_bits_col2: E::F,
        high_14_ms_bits_col3: E::F,
        high_5_ms_bits_col4: E::F,
        state_0_id_col5: E::F,
        low_16_bits_col6: E::F,
        high_16_bits_col7: E::F,
        low_7_ms_bits_col8: E::F,
        high_14_ms_bits_col9: E::F,
        high_5_ms_bits_col10: E::F,
        state_1_id_col11: E::F,
        low_16_bits_col12: E::F,
        high_16_bits_col13: E::F,
        low_7_ms_bits_col14: E::F,
        high_14_ms_bits_col15: E::F,
        high_5_ms_bits_col16: E::F,
        state_2_id_col17: E::F,
        low_16_bits_col18: E::F,
        high_16_bits_col19: E::F,
        low_7_ms_bits_col20: E::F,
        high_14_ms_bits_col21: E::F,
        high_5_ms_bits_col22: E::F,
        state_3_id_col23: E::F,
        low_16_bits_col24: E::F,
        high_16_bits_col25: E::F,
        low_7_ms_bits_col26: E::F,
        high_14_ms_bits_col27: E::F,
        high_5_ms_bits_col28: E::F,
        state_4_id_col29: E::F,
        low_16_bits_col30: E::F,
        high_16_bits_col31: E::F,
        low_7_ms_bits_col32: E::F,
        high_14_ms_bits_col33: E::F,
        high_5_ms_bits_col34: E::F,
        state_5_id_col35: E::F,
        low_16_bits_col36: E::F,
        high_16_bits_col37: E::F,
        low_7_ms_bits_col38: E::F,
        high_14_ms_bits_col39: E::F,
        high_5_ms_bits_col40: E::F,
        state_6_id_col41: E::F,
        low_16_bits_col42: E::F,
        high_16_bits_col43: E::F,
        low_7_ms_bits_col44: E::F,
        high_14_ms_bits_col45: E::F,
        high_5_ms_bits_col46: E::F,
        state_7_id_col47: E::F,
        ms_8_bits_col48: E::F,
        ms_8_bits_col49: E::F,
        xor_col50: E::F,
        xor_col51: E::F,
        xor_col52: E::F,
        xor_col53: E::F,
        range_check_7_2_5_lookup_elements: &relations::RangeCheck_7_2_5,
        memory_address_to_id_lookup_elements: &relations::MemoryAddressToId,
        memory_id_to_big_lookup_elements: &relations::MemoryIdToBig,
        verify_bitwise_xor_8_lookup_elements: &relations::VerifyBitwiseXor_8,
        eval: &mut E,
    ) -> [E::F; 4] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_127 = E::F::from(M31::from(127));
        let M31_14 = E::F::from(M31::from(14));
        let M31_2 = E::F::from(M31::from(2));
        let M31_256 = E::F::from(M31::from(256));
        let M31_3 = E::F::from(M31::from(3));
        let M31_4 = E::F::from(M31::from(4));
        let M31_5 = E::F::from(M31::from(5));
        let M31_55723 = E::F::from(M31::from(55723));
        let M31_57468 = E::F::from(M31::from(57468));
        let M31_6 = E::F::from(M31::from(6));
        let M31_7 = E::F::from(M31::from(7));
        let M31_8067 = E::F::from(M31::from(8067));
        let M31_81 = E::F::from(M31::from(81));
        let M31_82 = E::F::from(M31::from(82));
        let M31_9812 = E::F::from(M31::from(9812));

        ReadU32::evaluate(
            [create_blake_round_input_input_limb_0.clone()],
            low_16_bits_col0.clone(),
            high_16_bits_col1.clone(),
            low_7_ms_bits_col2.clone(),
            high_14_ms_bits_col3.clone(),
            high_5_ms_bits_col4.clone(),
            state_0_id_col5.clone(),
            range_check_7_2_5_lookup_elements,
            memory_address_to_id_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        ReadU32::evaluate(
            [(create_blake_round_input_input_limb_0.clone() + M31_1.clone())],
            low_16_bits_col6.clone(),
            high_16_bits_col7.clone(),
            low_7_ms_bits_col8.clone(),
            high_14_ms_bits_col9.clone(),
            high_5_ms_bits_col10.clone(),
            state_1_id_col11.clone(),
            range_check_7_2_5_lookup_elements,
            memory_address_to_id_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        ReadU32::evaluate(
            [(create_blake_round_input_input_limb_0.clone() + M31_2.clone())],
            low_16_bits_col12.clone(),
            high_16_bits_col13.clone(),
            low_7_ms_bits_col14.clone(),
            high_14_ms_bits_col15.clone(),
            high_5_ms_bits_col16.clone(),
            state_2_id_col17.clone(),
            range_check_7_2_5_lookup_elements,
            memory_address_to_id_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        ReadU32::evaluate(
            [(create_blake_round_input_input_limb_0.clone() + M31_3.clone())],
            low_16_bits_col18.clone(),
            high_16_bits_col19.clone(),
            low_7_ms_bits_col20.clone(),
            high_14_ms_bits_col21.clone(),
            high_5_ms_bits_col22.clone(),
            state_3_id_col23.clone(),
            range_check_7_2_5_lookup_elements,
            memory_address_to_id_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        ReadU32::evaluate(
            [(create_blake_round_input_input_limb_0.clone() + M31_4.clone())],
            low_16_bits_col24.clone(),
            high_16_bits_col25.clone(),
            low_7_ms_bits_col26.clone(),
            high_14_ms_bits_col27.clone(),
            high_5_ms_bits_col28.clone(),
            state_4_id_col29.clone(),
            range_check_7_2_5_lookup_elements,
            memory_address_to_id_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        ReadU32::evaluate(
            [(create_blake_round_input_input_limb_0.clone() + M31_5.clone())],
            low_16_bits_col30.clone(),
            high_16_bits_col31.clone(),
            low_7_ms_bits_col32.clone(),
            high_14_ms_bits_col33.clone(),
            high_5_ms_bits_col34.clone(),
            state_5_id_col35.clone(),
            range_check_7_2_5_lookup_elements,
            memory_address_to_id_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        ReadU32::evaluate(
            [(create_blake_round_input_input_limb_0.clone() + M31_6.clone())],
            low_16_bits_col36.clone(),
            high_16_bits_col37.clone(),
            low_7_ms_bits_col38.clone(),
            high_14_ms_bits_col39.clone(),
            high_5_ms_bits_col40.clone(),
            state_6_id_col41.clone(),
            range_check_7_2_5_lookup_elements,
            memory_address_to_id_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        ReadU32::evaluate(
            [(create_blake_round_input_input_limb_0.clone() + M31_7.clone())],
            low_16_bits_col42.clone(),
            high_16_bits_col43.clone(),
            low_7_ms_bits_col44.clone(),
            high_14_ms_bits_col45.clone(),
            high_5_ms_bits_col46.clone(),
            state_7_id_col47.clone(),
            range_check_7_2_5_lookup_elements,
            memory_address_to_id_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        let [split_16_low_part_size_8_output_tmp_f95c3_89_limb_0] = Split16LowPartSize8::evaluate(
            [create_blake_round_input_input_limb_1.clone()],
            ms_8_bits_col48.clone(),
            eval,
        );
        let [split_16_low_part_size_8_output_tmp_f95c3_91_limb_0] = Split16LowPartSize8::evaluate(
            [create_blake_round_input_input_limb_2.clone()],
            ms_8_bits_col49.clone(),
            eval,
        );
        BitwiseXorNumBits8::evaluate(
            [
                split_16_low_part_size_8_output_tmp_f95c3_89_limb_0.clone(),
                M31_127.clone(),
            ],
            xor_col50.clone(),
            verify_bitwise_xor_8_lookup_elements,
            eval,
        );
        BitwiseXorNumBits8::evaluate(
            [ms_8_bits_col48.clone(), M31_82.clone()],
            xor_col51.clone(),
            verify_bitwise_xor_8_lookup_elements,
            eval,
        );
        BitwiseXorNumBits8::evaluate(
            [
                split_16_low_part_size_8_output_tmp_f95c3_91_limb_0.clone(),
                M31_14.clone(),
            ],
            xor_col52.clone(),
            verify_bitwise_xor_8_lookup_elements,
            eval,
        );
        BitwiseXorNumBits8::evaluate(
            [ms_8_bits_col49.clone(), M31_81.clone()],
            xor_col53.clone(),
            verify_bitwise_xor_8_lookup_elements,
            eval,
        );
        [
            (xor_col50.clone() + (xor_col51.clone() * M31_256.clone())),
            (xor_col52.clone() + (xor_col53.clone() * M31_256.clone())),
            ((create_blake_round_input_input_limb_3.clone() * M31_9812.clone())
                + ((M31_1.clone() - create_blake_round_input_input_limb_3.clone())
                    * M31_55723.clone())),
            ((create_blake_round_input_input_limb_3.clone() * M31_57468.clone())
                + ((M31_1.clone() - create_blake_round_input_input_limb_3.clone())
                    * M31_8067.clone())),
        ]
    }
}
