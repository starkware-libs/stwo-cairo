use crate::components::prelude::*;
use crate::components::subroutines::decode_instruction_64420::DecodeInstruction64420;
use crate::components::subroutines::read_blake_word::ReadBlakeWord;
use crate::components::subroutines::read_positive_num_bits_27::ReadPositiveNumBits27;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize, CompactBinary)]
pub struct DecodeBlakeOpcode {}

impl DecodeBlakeOpcode {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [decode_blake_opcode_input_pc, decode_blake_opcode_input_ap, decode_blake_opcode_input_fp]: [E::F; 3],
        offset0_col0: E::F,
        offset1_col1: E::F,
        offset2_col2: E::F,
        dst_base_fp_col3: E::F,
        op0_base_fp_col4: E::F,
        op1_base_fp_col5: E::F,
        op1_base_ap_col6: E::F,
        ap_update_add_1_col7: E::F,
        opcode_extension_col8: E::F,
        mem0_base_col9: E::F,
        op0_id_col10: E::F,
        op0_limb_0_col11: E::F,
        op0_limb_1_col12: E::F,
        op0_limb_2_col13: E::F,
        mem1_base_col14: E::F,
        op1_id_col15: E::F,
        op1_limb_0_col16: E::F,
        op1_limb_1_col17: E::F,
        op1_limb_2_col18: E::F,
        ap_id_col19: E::F,
        ap_limb_0_col20: E::F,
        ap_limb_1_col21: E::F,
        ap_limb_2_col22: E::F,
        mem_dst_base_col23: E::F,
        low_16_bits_col24: E::F,
        high_16_bits_col25: E::F,
        low_7_ms_bits_col26: E::F,
        high_14_ms_bits_col27: E::F,
        high_5_ms_bits_col28: E::F,
        dst_id_col29: E::F,
        verify_instruction_lookup_elements: &relations::VerifyInstruction,
        memory_address_to_id_lookup_elements: &relations::MemoryAddressToId,
        memory_id_to_big_lookup_elements: &relations::MemoryIdToBig,
        range_check_7_2_5_lookup_elements: &relations::RangeCheck_7_2_5,
        eval: &mut E,
    ) -> [E::F; 4] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_2 = E::F::from(M31::from(2));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_512 = E::F::from(M31::from(512));

        let [decode_instruction_64420_output_tmp_47e62_10_offset0, decode_instruction_64420_output_tmp_47e62_10_offset1, decode_instruction_64420_output_tmp_47e62_10_offset2] =
            DecodeInstruction64420::evaluate(
                [decode_blake_opcode_input_pc.clone()],
                offset0_col0.clone(),
                offset1_col1.clone(),
                offset2_col2.clone(),
                dst_base_fp_col3.clone(),
                op0_base_fp_col4.clone(),
                op1_base_fp_col5.clone(),
                op1_base_ap_col6.clone(),
                ap_update_add_1_col7.clone(),
                opcode_extension_col8.clone(),
                verify_instruction_lookup_elements,
                eval,
            );
        // Exactly one of op1_base_fp and op1_base_ap is 1.
        eval.add_constraint(
            ((op1_base_fp_col5.clone() + op1_base_ap_col6.clone()) - M31_1.clone()),
        );
        // OpcodeExtension is either Blake or BlakeFinalize.
        eval.add_constraint(
            ((opcode_extension_col8.clone() - M31_1.clone())
                * (opcode_extension_col8.clone() - M31_2.clone())),
        );
        // mem0_base.
        eval.add_constraint(
            (mem0_base_col9.clone()
                - ((op0_base_fp_col4.clone() * decode_blake_opcode_input_fp.clone())
                    + ((M31_1.clone() - op0_base_fp_col4.clone())
                        * decode_blake_opcode_input_ap.clone()))),
        );
        ReadPositiveNumBits27::evaluate(
            [(mem0_base_col9.clone()
                + decode_instruction_64420_output_tmp_47e62_10_offset1.clone())],
            op0_id_col10.clone(),
            op0_limb_0_col11.clone(),
            op0_limb_1_col12.clone(),
            op0_limb_2_col13.clone(),
            memory_address_to_id_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        // mem1_base.
        eval.add_constraint(
            (mem1_base_col14.clone()
                - ((op1_base_fp_col5.clone() * decode_blake_opcode_input_fp.clone())
                    + (op1_base_ap_col6.clone() * decode_blake_opcode_input_ap.clone()))),
        );
        ReadPositiveNumBits27::evaluate(
            [(mem1_base_col14.clone()
                + decode_instruction_64420_output_tmp_47e62_10_offset2.clone())],
            op1_id_col15.clone(),
            op1_limb_0_col16.clone(),
            op1_limb_1_col17.clone(),
            op1_limb_2_col18.clone(),
            memory_address_to_id_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        ReadPositiveNumBits27::evaluate(
            [decode_blake_opcode_input_ap.clone()],
            ap_id_col19.clone(),
            ap_limb_0_col20.clone(),
            ap_limb_1_col21.clone(),
            ap_limb_2_col22.clone(),
            memory_address_to_id_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        // mem_dst_base.
        eval.add_constraint(
            (mem_dst_base_col23.clone()
                - ((dst_base_fp_col3.clone() * decode_blake_opcode_input_fp.clone())
                    + ((M31_1.clone() - dst_base_fp_col3.clone())
                        * decode_blake_opcode_input_ap.clone()))),
        );
        ReadBlakeWord::evaluate(
            [(mem_dst_base_col23.clone()
                + decode_instruction_64420_output_tmp_47e62_10_offset0.clone())],
            low_16_bits_col24.clone(),
            high_16_bits_col25.clone(),
            low_7_ms_bits_col26.clone(),
            high_14_ms_bits_col27.clone(),
            high_5_ms_bits_col28.clone(),
            dst_id_col29.clone(),
            range_check_7_2_5_lookup_elements,
            memory_address_to_id_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        [
            ((op0_limb_0_col11.clone() + (op0_limb_1_col12.clone() * M31_512.clone()))
                + (op0_limb_2_col13.clone() * M31_262144.clone())),
            ((op1_limb_0_col16.clone() + (op1_limb_1_col17.clone() * M31_512.clone()))
                + (op1_limb_2_col18.clone() * M31_262144.clone())),
            ((ap_limb_0_col20.clone() + (ap_limb_1_col21.clone() * M31_512.clone()))
                + (ap_limb_2_col22.clone() * M31_262144.clone())),
            (opcode_extension_col8.clone() - M31_1.clone()),
        ]
    }
}
