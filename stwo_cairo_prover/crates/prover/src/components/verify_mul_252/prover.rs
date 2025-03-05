#![allow(unused_parens)]
#![allow(unused_imports)]
use super::component::{Claim, InteractionClaim};
use crate::components::prelude::proving::*;
use crate::components::range_check_19;

pub type InputType = [Felt252; 3];
pub type PackedInputType = [PackedFelt252; 3];
const N_TRACE_COLUMNS: usize = 113;

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: Vec<InputType>,
}
impl ClaimGenerator {
    pub fn new() -> Self {
        Self { inputs: Vec::new() }
    }

    pub fn write_trace<MC: MerkleChannel>(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        range_check_19_state: &range_check_19::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let n_rows = self.inputs.len();
        assert_ne!(n_rows, 0);
        let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
        let log_size = size.ilog2();
        self.inputs.resize(size, *self.inputs.first().unwrap());
        let packed_inputs = pack_values(&self.inputs);

        let (trace, lookup_data) = write_trace_simd(n_rows, packed_inputs, range_check_19_state);

        tree_builder.extend_evals(trace.to_evals());

        (
            Claim { log_size },
            InteractionClaimGenerator {
                n_rows,
                lookup_data,
            },
        )
    }

    pub fn add_input(&mut self, input: &InputType) {
        self.inputs.push(*input);
    }

    pub fn add_inputs(&mut self, inputs: &[InputType]) {
        self.inputs.extend_from_slice(inputs);
    }
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    n_rows: usize,
    inputs: Vec<PackedInputType>,
    range_check_19_state: &range_check_19::ClaimGenerator,
) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
        )
    };

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_131072 = PackedM31::broadcast(M31::from(131072));
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_4194304 = PackedM31::broadcast(M31::from(4194304));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_65536 = PackedM31::broadcast(M31::from(65536));
    let M31_8 = PackedM31::broadcast(M31::from(8));
    let UInt32_262143 = PackedUInt32::broadcast(UInt32::from(262143));
    let UInt32_511 = PackedUInt32::broadcast(UInt32::from(511));
    let UInt32_65536 = PackedUInt32::broadcast(UInt32::from(65536));
    let UInt32_9 = PackedUInt32::broadcast(UInt32::from(9));
    let padding_col = Enabler::new(n_rows);

    trace
        .par_iter_mut()
        .enumerate()
        .zip(inputs.into_par_iter())
        .zip(lookup_data.par_iter_mut())
        .for_each(
            |(((row_index, mut row), verify_mul_252_input), lookup_data)| {
                let input_tmp_9a554_0 = [
                    verify_mul_252_input[0],
                    verify_mul_252_input[1],
                    verify_mul_252_input[2],
                ];
                let input_limb_0_col0 = input_tmp_9a554_0[0].get_m31(0);
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = input_tmp_9a554_0[0].get_m31(1);
                *row[1] = input_limb_1_col1;
                let input_limb_2_col2 = input_tmp_9a554_0[0].get_m31(2);
                *row[2] = input_limb_2_col2;
                let input_limb_3_col3 = input_tmp_9a554_0[0].get_m31(3);
                *row[3] = input_limb_3_col3;
                let input_limb_4_col4 = input_tmp_9a554_0[0].get_m31(4);
                *row[4] = input_limb_4_col4;
                let input_limb_5_col5 = input_tmp_9a554_0[0].get_m31(5);
                *row[5] = input_limb_5_col5;
                let input_limb_6_col6 = input_tmp_9a554_0[0].get_m31(6);
                *row[6] = input_limb_6_col6;
                let input_limb_7_col7 = input_tmp_9a554_0[0].get_m31(7);
                *row[7] = input_limb_7_col7;
                let input_limb_8_col8 = input_tmp_9a554_0[0].get_m31(8);
                *row[8] = input_limb_8_col8;
                let input_limb_9_col9 = input_tmp_9a554_0[0].get_m31(9);
                *row[9] = input_limb_9_col9;
                let input_limb_10_col10 = input_tmp_9a554_0[0].get_m31(10);
                *row[10] = input_limb_10_col10;
                let input_limb_11_col11 = input_tmp_9a554_0[0].get_m31(11);
                *row[11] = input_limb_11_col11;
                let input_limb_12_col12 = input_tmp_9a554_0[0].get_m31(12);
                *row[12] = input_limb_12_col12;
                let input_limb_13_col13 = input_tmp_9a554_0[0].get_m31(13);
                *row[13] = input_limb_13_col13;
                let input_limb_14_col14 = input_tmp_9a554_0[0].get_m31(14);
                *row[14] = input_limb_14_col14;
                let input_limb_15_col15 = input_tmp_9a554_0[0].get_m31(15);
                *row[15] = input_limb_15_col15;
                let input_limb_16_col16 = input_tmp_9a554_0[0].get_m31(16);
                *row[16] = input_limb_16_col16;
                let input_limb_17_col17 = input_tmp_9a554_0[0].get_m31(17);
                *row[17] = input_limb_17_col17;
                let input_limb_18_col18 = input_tmp_9a554_0[0].get_m31(18);
                *row[18] = input_limb_18_col18;
                let input_limb_19_col19 = input_tmp_9a554_0[0].get_m31(19);
                *row[19] = input_limb_19_col19;
                let input_limb_20_col20 = input_tmp_9a554_0[0].get_m31(20);
                *row[20] = input_limb_20_col20;
                let input_limb_21_col21 = input_tmp_9a554_0[0].get_m31(21);
                *row[21] = input_limb_21_col21;
                let input_limb_22_col22 = input_tmp_9a554_0[0].get_m31(22);
                *row[22] = input_limb_22_col22;
                let input_limb_23_col23 = input_tmp_9a554_0[0].get_m31(23);
                *row[23] = input_limb_23_col23;
                let input_limb_24_col24 = input_tmp_9a554_0[0].get_m31(24);
                *row[24] = input_limb_24_col24;
                let input_limb_25_col25 = input_tmp_9a554_0[0].get_m31(25);
                *row[25] = input_limb_25_col25;
                let input_limb_26_col26 = input_tmp_9a554_0[0].get_m31(26);
                *row[26] = input_limb_26_col26;
                let input_limb_27_col27 = input_tmp_9a554_0[0].get_m31(27);
                *row[27] = input_limb_27_col27;
                let input_limb_28_col28 = input_tmp_9a554_0[1].get_m31(0);
                *row[28] = input_limb_28_col28;
                let input_limb_29_col29 = input_tmp_9a554_0[1].get_m31(1);
                *row[29] = input_limb_29_col29;
                let input_limb_30_col30 = input_tmp_9a554_0[1].get_m31(2);
                *row[30] = input_limb_30_col30;
                let input_limb_31_col31 = input_tmp_9a554_0[1].get_m31(3);
                *row[31] = input_limb_31_col31;
                let input_limb_32_col32 = input_tmp_9a554_0[1].get_m31(4);
                *row[32] = input_limb_32_col32;
                let input_limb_33_col33 = input_tmp_9a554_0[1].get_m31(5);
                *row[33] = input_limb_33_col33;
                let input_limb_34_col34 = input_tmp_9a554_0[1].get_m31(6);
                *row[34] = input_limb_34_col34;
                let input_limb_35_col35 = input_tmp_9a554_0[1].get_m31(7);
                *row[35] = input_limb_35_col35;
                let input_limb_36_col36 = input_tmp_9a554_0[1].get_m31(8);
                *row[36] = input_limb_36_col36;
                let input_limb_37_col37 = input_tmp_9a554_0[1].get_m31(9);
                *row[37] = input_limb_37_col37;
                let input_limb_38_col38 = input_tmp_9a554_0[1].get_m31(10);
                *row[38] = input_limb_38_col38;
                let input_limb_39_col39 = input_tmp_9a554_0[1].get_m31(11);
                *row[39] = input_limb_39_col39;
                let input_limb_40_col40 = input_tmp_9a554_0[1].get_m31(12);
                *row[40] = input_limb_40_col40;
                let input_limb_41_col41 = input_tmp_9a554_0[1].get_m31(13);
                *row[41] = input_limb_41_col41;
                let input_limb_42_col42 = input_tmp_9a554_0[1].get_m31(14);
                *row[42] = input_limb_42_col42;
                let input_limb_43_col43 = input_tmp_9a554_0[1].get_m31(15);
                *row[43] = input_limb_43_col43;
                let input_limb_44_col44 = input_tmp_9a554_0[1].get_m31(16);
                *row[44] = input_limb_44_col44;
                let input_limb_45_col45 = input_tmp_9a554_0[1].get_m31(17);
                *row[45] = input_limb_45_col45;
                let input_limb_46_col46 = input_tmp_9a554_0[1].get_m31(18);
                *row[46] = input_limb_46_col46;
                let input_limb_47_col47 = input_tmp_9a554_0[1].get_m31(19);
                *row[47] = input_limb_47_col47;
                let input_limb_48_col48 = input_tmp_9a554_0[1].get_m31(20);
                *row[48] = input_limb_48_col48;
                let input_limb_49_col49 = input_tmp_9a554_0[1].get_m31(21);
                *row[49] = input_limb_49_col49;
                let input_limb_50_col50 = input_tmp_9a554_0[1].get_m31(22);
                *row[50] = input_limb_50_col50;
                let input_limb_51_col51 = input_tmp_9a554_0[1].get_m31(23);
                *row[51] = input_limb_51_col51;
                let input_limb_52_col52 = input_tmp_9a554_0[1].get_m31(24);
                *row[52] = input_limb_52_col52;
                let input_limb_53_col53 = input_tmp_9a554_0[1].get_m31(25);
                *row[53] = input_limb_53_col53;
                let input_limb_54_col54 = input_tmp_9a554_0[1].get_m31(26);
                *row[54] = input_limb_54_col54;
                let input_limb_55_col55 = input_tmp_9a554_0[1].get_m31(27);
                *row[55] = input_limb_55_col55;
                let input_limb_56_col56 = input_tmp_9a554_0[2].get_m31(0);
                *row[56] = input_limb_56_col56;
                let input_limb_57_col57 = input_tmp_9a554_0[2].get_m31(1);
                *row[57] = input_limb_57_col57;
                let input_limb_58_col58 = input_tmp_9a554_0[2].get_m31(2);
                *row[58] = input_limb_58_col58;
                let input_limb_59_col59 = input_tmp_9a554_0[2].get_m31(3);
                *row[59] = input_limb_59_col59;
                let input_limb_60_col60 = input_tmp_9a554_0[2].get_m31(4);
                *row[60] = input_limb_60_col60;
                let input_limb_61_col61 = input_tmp_9a554_0[2].get_m31(5);
                *row[61] = input_limb_61_col61;
                let input_limb_62_col62 = input_tmp_9a554_0[2].get_m31(6);
                *row[62] = input_limb_62_col62;
                let input_limb_63_col63 = input_tmp_9a554_0[2].get_m31(7);
                *row[63] = input_limb_63_col63;
                let input_limb_64_col64 = input_tmp_9a554_0[2].get_m31(8);
                *row[64] = input_limb_64_col64;
                let input_limb_65_col65 = input_tmp_9a554_0[2].get_m31(9);
                *row[65] = input_limb_65_col65;
                let input_limb_66_col66 = input_tmp_9a554_0[2].get_m31(10);
                *row[66] = input_limb_66_col66;
                let input_limb_67_col67 = input_tmp_9a554_0[2].get_m31(11);
                *row[67] = input_limb_67_col67;
                let input_limb_68_col68 = input_tmp_9a554_0[2].get_m31(12);
                *row[68] = input_limb_68_col68;
                let input_limb_69_col69 = input_tmp_9a554_0[2].get_m31(13);
                *row[69] = input_limb_69_col69;
                let input_limb_70_col70 = input_tmp_9a554_0[2].get_m31(14);
                *row[70] = input_limb_70_col70;
                let input_limb_71_col71 = input_tmp_9a554_0[2].get_m31(15);
                *row[71] = input_limb_71_col71;
                let input_limb_72_col72 = input_tmp_9a554_0[2].get_m31(16);
                *row[72] = input_limb_72_col72;
                let input_limb_73_col73 = input_tmp_9a554_0[2].get_m31(17);
                *row[73] = input_limb_73_col73;
                let input_limb_74_col74 = input_tmp_9a554_0[2].get_m31(18);
                *row[74] = input_limb_74_col74;
                let input_limb_75_col75 = input_tmp_9a554_0[2].get_m31(19);
                *row[75] = input_limb_75_col75;
                let input_limb_76_col76 = input_tmp_9a554_0[2].get_m31(20);
                *row[76] = input_limb_76_col76;
                let input_limb_77_col77 = input_tmp_9a554_0[2].get_m31(21);
                *row[77] = input_limb_77_col77;
                let input_limb_78_col78 = input_tmp_9a554_0[2].get_m31(22);
                *row[78] = input_limb_78_col78;
                let input_limb_79_col79 = input_tmp_9a554_0[2].get_m31(23);
                *row[79] = input_limb_79_col79;
                let input_limb_80_col80 = input_tmp_9a554_0[2].get_m31(24);
                *row[80] = input_limb_80_col80;
                let input_limb_81_col81 = input_tmp_9a554_0[2].get_m31(25);
                *row[81] = input_limb_81_col81;
                let input_limb_82_col82 = input_tmp_9a554_0[2].get_m31(26);
                *row[82] = input_limb_82_col82;
                let input_limb_83_col83 = input_tmp_9a554_0[2].get_m31(27);
                *row[83] = input_limb_83_col83;
                let conv_tmp_9a554_1 = (((M31_0) - (input_limb_56_col56))
                    + ((input_limb_0_col0) * (input_limb_28_col28)));
                let conv_tmp_9a554_2 = ((((M31_0) - (input_limb_57_col57))
                    + ((input_limb_0_col0) * (input_limb_29_col29)))
                    + ((input_limb_1_col1) * (input_limb_28_col28)));
                let conv_tmp_9a554_3 = (((((M31_0) - (input_limb_58_col58))
                    + ((input_limb_0_col0) * (input_limb_30_col30)))
                    + ((input_limb_1_col1) * (input_limb_29_col29)))
                    + ((input_limb_2_col2) * (input_limb_28_col28)));
                let conv_tmp_9a554_4 = ((((((M31_0) - (input_limb_59_col59))
                    + ((input_limb_0_col0) * (input_limb_31_col31)))
                    + ((input_limb_1_col1) * (input_limb_30_col30)))
                    + ((input_limb_2_col2) * (input_limb_29_col29)))
                    + ((input_limb_3_col3) * (input_limb_28_col28)));
                let conv_tmp_9a554_5 = (((((((M31_0) - (input_limb_60_col60))
                    + ((input_limb_0_col0) * (input_limb_32_col32)))
                    + ((input_limb_1_col1) * (input_limb_31_col31)))
                    + ((input_limb_2_col2) * (input_limb_30_col30)))
                    + ((input_limb_3_col3) * (input_limb_29_col29)))
                    + ((input_limb_4_col4) * (input_limb_28_col28)));
                let conv_tmp_9a554_6 = ((((((((M31_0) - (input_limb_61_col61))
                    + ((input_limb_0_col0) * (input_limb_33_col33)))
                    + ((input_limb_1_col1) * (input_limb_32_col32)))
                    + ((input_limb_2_col2) * (input_limb_31_col31)))
                    + ((input_limb_3_col3) * (input_limb_30_col30)))
                    + ((input_limb_4_col4) * (input_limb_29_col29)))
                    + ((input_limb_5_col5) * (input_limb_28_col28)));
                let conv_tmp_9a554_7 = (((((((((M31_0) - (input_limb_62_col62))
                    + ((input_limb_0_col0) * (input_limb_34_col34)))
                    + ((input_limb_1_col1) * (input_limb_33_col33)))
                    + ((input_limb_2_col2) * (input_limb_32_col32)))
                    + ((input_limb_3_col3) * (input_limb_31_col31)))
                    + ((input_limb_4_col4) * (input_limb_30_col30)))
                    + ((input_limb_5_col5) * (input_limb_29_col29)))
                    + ((input_limb_6_col6) * (input_limb_28_col28)));
                let conv_tmp_9a554_8 = ((((((((((M31_0) - (input_limb_63_col63))
                    + ((input_limb_0_col0) * (input_limb_35_col35)))
                    + ((input_limb_1_col1) * (input_limb_34_col34)))
                    + ((input_limb_2_col2) * (input_limb_33_col33)))
                    + ((input_limb_3_col3) * (input_limb_32_col32)))
                    + ((input_limb_4_col4) * (input_limb_31_col31)))
                    + ((input_limb_5_col5) * (input_limb_30_col30)))
                    + ((input_limb_6_col6) * (input_limb_29_col29)))
                    + ((input_limb_7_col7) * (input_limb_28_col28)));
                let conv_tmp_9a554_9 = (((((((((((M31_0) - (input_limb_64_col64))
                    + ((input_limb_0_col0) * (input_limb_36_col36)))
                    + ((input_limb_1_col1) * (input_limb_35_col35)))
                    + ((input_limb_2_col2) * (input_limb_34_col34)))
                    + ((input_limb_3_col3) * (input_limb_33_col33)))
                    + ((input_limb_4_col4) * (input_limb_32_col32)))
                    + ((input_limb_5_col5) * (input_limb_31_col31)))
                    + ((input_limb_6_col6) * (input_limb_30_col30)))
                    + ((input_limb_7_col7) * (input_limb_29_col29)))
                    + ((input_limb_8_col8) * (input_limb_28_col28)));
                let conv_tmp_9a554_10 = ((((((((((((M31_0) - (input_limb_65_col65))
                    + ((input_limb_0_col0) * (input_limb_37_col37)))
                    + ((input_limb_1_col1) * (input_limb_36_col36)))
                    + ((input_limb_2_col2) * (input_limb_35_col35)))
                    + ((input_limb_3_col3) * (input_limb_34_col34)))
                    + ((input_limb_4_col4) * (input_limb_33_col33)))
                    + ((input_limb_5_col5) * (input_limb_32_col32)))
                    + ((input_limb_6_col6) * (input_limb_31_col31)))
                    + ((input_limb_7_col7) * (input_limb_30_col30)))
                    + ((input_limb_8_col8) * (input_limb_29_col29)))
                    + ((input_limb_9_col9) * (input_limb_28_col28)));
                let conv_tmp_9a554_11 = (((((((((((((M31_0) - (input_limb_66_col66))
                    + ((input_limb_0_col0) * (input_limb_38_col38)))
                    + ((input_limb_1_col1) * (input_limb_37_col37)))
                    + ((input_limb_2_col2) * (input_limb_36_col36)))
                    + ((input_limb_3_col3) * (input_limb_35_col35)))
                    + ((input_limb_4_col4) * (input_limb_34_col34)))
                    + ((input_limb_5_col5) * (input_limb_33_col33)))
                    + ((input_limb_6_col6) * (input_limb_32_col32)))
                    + ((input_limb_7_col7) * (input_limb_31_col31)))
                    + ((input_limb_8_col8) * (input_limb_30_col30)))
                    + ((input_limb_9_col9) * (input_limb_29_col29)))
                    + ((input_limb_10_col10) * (input_limb_28_col28)));
                let conv_tmp_9a554_12 = ((((((((((((((M31_0) - (input_limb_67_col67))
                    + ((input_limb_0_col0) * (input_limb_39_col39)))
                    + ((input_limb_1_col1) * (input_limb_38_col38)))
                    + ((input_limb_2_col2) * (input_limb_37_col37)))
                    + ((input_limb_3_col3) * (input_limb_36_col36)))
                    + ((input_limb_4_col4) * (input_limb_35_col35)))
                    + ((input_limb_5_col5) * (input_limb_34_col34)))
                    + ((input_limb_6_col6) * (input_limb_33_col33)))
                    + ((input_limb_7_col7) * (input_limb_32_col32)))
                    + ((input_limb_8_col8) * (input_limb_31_col31)))
                    + ((input_limb_9_col9) * (input_limb_30_col30)))
                    + ((input_limb_10_col10) * (input_limb_29_col29)))
                    + ((input_limb_11_col11) * (input_limb_28_col28)));
                let conv_tmp_9a554_13 = (((((((((((((((M31_0) - (input_limb_68_col68))
                    + ((input_limb_0_col0) * (input_limb_40_col40)))
                    + ((input_limb_1_col1) * (input_limb_39_col39)))
                    + ((input_limb_2_col2) * (input_limb_38_col38)))
                    + ((input_limb_3_col3) * (input_limb_37_col37)))
                    + ((input_limb_4_col4) * (input_limb_36_col36)))
                    + ((input_limb_5_col5) * (input_limb_35_col35)))
                    + ((input_limb_6_col6) * (input_limb_34_col34)))
                    + ((input_limb_7_col7) * (input_limb_33_col33)))
                    + ((input_limb_8_col8) * (input_limb_32_col32)))
                    + ((input_limb_9_col9) * (input_limb_31_col31)))
                    + ((input_limb_10_col10) * (input_limb_30_col30)))
                    + ((input_limb_11_col11) * (input_limb_29_col29)))
                    + ((input_limb_12_col12) * (input_limb_28_col28)));
                let conv_tmp_9a554_14 = ((((((((((((((((M31_0)
                    - (input_limb_69_col69))
                    + ((input_limb_0_col0) * (input_limb_41_col41)))
                    + ((input_limb_1_col1) * (input_limb_40_col40)))
                    + ((input_limb_2_col2) * (input_limb_39_col39)))
                    + ((input_limb_3_col3) * (input_limb_38_col38)))
                    + ((input_limb_4_col4) * (input_limb_37_col37)))
                    + ((input_limb_5_col5) * (input_limb_36_col36)))
                    + ((input_limb_6_col6) * (input_limb_35_col35)))
                    + ((input_limb_7_col7) * (input_limb_34_col34)))
                    + ((input_limb_8_col8) * (input_limb_33_col33)))
                    + ((input_limb_9_col9) * (input_limb_32_col32)))
                    + ((input_limb_10_col10) * (input_limb_31_col31)))
                    + ((input_limb_11_col11) * (input_limb_30_col30)))
                    + ((input_limb_12_col12) * (input_limb_29_col29)))
                    + ((input_limb_13_col13) * (input_limb_28_col28)));
                let conv_tmp_9a554_15 = (((((((((((((((((M31_0)
                    - (input_limb_70_col70))
                    + ((input_limb_0_col0) * (input_limb_42_col42)))
                    + ((input_limb_1_col1) * (input_limb_41_col41)))
                    + ((input_limb_2_col2) * (input_limb_40_col40)))
                    + ((input_limb_3_col3) * (input_limb_39_col39)))
                    + ((input_limb_4_col4) * (input_limb_38_col38)))
                    + ((input_limb_5_col5) * (input_limb_37_col37)))
                    + ((input_limb_6_col6) * (input_limb_36_col36)))
                    + ((input_limb_7_col7) * (input_limb_35_col35)))
                    + ((input_limb_8_col8) * (input_limb_34_col34)))
                    + ((input_limb_9_col9) * (input_limb_33_col33)))
                    + ((input_limb_10_col10) * (input_limb_32_col32)))
                    + ((input_limb_11_col11) * (input_limb_31_col31)))
                    + ((input_limb_12_col12) * (input_limb_30_col30)))
                    + ((input_limb_13_col13) * (input_limb_29_col29)))
                    + ((input_limb_14_col14) * (input_limb_28_col28)));
                let conv_tmp_9a554_16 = ((((((((((((((((((M31_0)
                    - (input_limb_71_col71))
                    + ((input_limb_0_col0) * (input_limb_43_col43)))
                    + ((input_limb_1_col1) * (input_limb_42_col42)))
                    + ((input_limb_2_col2) * (input_limb_41_col41)))
                    + ((input_limb_3_col3) * (input_limb_40_col40)))
                    + ((input_limb_4_col4) * (input_limb_39_col39)))
                    + ((input_limb_5_col5) * (input_limb_38_col38)))
                    + ((input_limb_6_col6) * (input_limb_37_col37)))
                    + ((input_limb_7_col7) * (input_limb_36_col36)))
                    + ((input_limb_8_col8) * (input_limb_35_col35)))
                    + ((input_limb_9_col9) * (input_limb_34_col34)))
                    + ((input_limb_10_col10) * (input_limb_33_col33)))
                    + ((input_limb_11_col11) * (input_limb_32_col32)))
                    + ((input_limb_12_col12) * (input_limb_31_col31)))
                    + ((input_limb_13_col13) * (input_limb_30_col30)))
                    + ((input_limb_14_col14) * (input_limb_29_col29)))
                    + ((input_limb_15_col15) * (input_limb_28_col28)));
                let conv_tmp_9a554_17 = (((((((((((((((((((M31_0)
                    - (input_limb_72_col72))
                    + ((input_limb_0_col0) * (input_limb_44_col44)))
                    + ((input_limb_1_col1) * (input_limb_43_col43)))
                    + ((input_limb_2_col2) * (input_limb_42_col42)))
                    + ((input_limb_3_col3) * (input_limb_41_col41)))
                    + ((input_limb_4_col4) * (input_limb_40_col40)))
                    + ((input_limb_5_col5) * (input_limb_39_col39)))
                    + ((input_limb_6_col6) * (input_limb_38_col38)))
                    + ((input_limb_7_col7) * (input_limb_37_col37)))
                    + ((input_limb_8_col8) * (input_limb_36_col36)))
                    + ((input_limb_9_col9) * (input_limb_35_col35)))
                    + ((input_limb_10_col10) * (input_limb_34_col34)))
                    + ((input_limb_11_col11) * (input_limb_33_col33)))
                    + ((input_limb_12_col12) * (input_limb_32_col32)))
                    + ((input_limb_13_col13) * (input_limb_31_col31)))
                    + ((input_limb_14_col14) * (input_limb_30_col30)))
                    + ((input_limb_15_col15) * (input_limb_29_col29)))
                    + ((input_limb_16_col16) * (input_limb_28_col28)));
                let conv_tmp_9a554_18 = ((((((((((((((((((((M31_0)
                    - (input_limb_73_col73))
                    + ((input_limb_0_col0) * (input_limb_45_col45)))
                    + ((input_limb_1_col1) * (input_limb_44_col44)))
                    + ((input_limb_2_col2) * (input_limb_43_col43)))
                    + ((input_limb_3_col3) * (input_limb_42_col42)))
                    + ((input_limb_4_col4) * (input_limb_41_col41)))
                    + ((input_limb_5_col5) * (input_limb_40_col40)))
                    + ((input_limb_6_col6) * (input_limb_39_col39)))
                    + ((input_limb_7_col7) * (input_limb_38_col38)))
                    + ((input_limb_8_col8) * (input_limb_37_col37)))
                    + ((input_limb_9_col9) * (input_limb_36_col36)))
                    + ((input_limb_10_col10) * (input_limb_35_col35)))
                    + ((input_limb_11_col11) * (input_limb_34_col34)))
                    + ((input_limb_12_col12) * (input_limb_33_col33)))
                    + ((input_limb_13_col13) * (input_limb_32_col32)))
                    + ((input_limb_14_col14) * (input_limb_31_col31)))
                    + ((input_limb_15_col15) * (input_limb_30_col30)))
                    + ((input_limb_16_col16) * (input_limb_29_col29)))
                    + ((input_limb_17_col17) * (input_limb_28_col28)));
                let conv_tmp_9a554_19 = (((((((((((((((((((((M31_0)
                    - (input_limb_74_col74))
                    + ((input_limb_0_col0) * (input_limb_46_col46)))
                    + ((input_limb_1_col1) * (input_limb_45_col45)))
                    + ((input_limb_2_col2) * (input_limb_44_col44)))
                    + ((input_limb_3_col3) * (input_limb_43_col43)))
                    + ((input_limb_4_col4) * (input_limb_42_col42)))
                    + ((input_limb_5_col5) * (input_limb_41_col41)))
                    + ((input_limb_6_col6) * (input_limb_40_col40)))
                    + ((input_limb_7_col7) * (input_limb_39_col39)))
                    + ((input_limb_8_col8) * (input_limb_38_col38)))
                    + ((input_limb_9_col9) * (input_limb_37_col37)))
                    + ((input_limb_10_col10) * (input_limb_36_col36)))
                    + ((input_limb_11_col11) * (input_limb_35_col35)))
                    + ((input_limb_12_col12) * (input_limb_34_col34)))
                    + ((input_limb_13_col13) * (input_limb_33_col33)))
                    + ((input_limb_14_col14) * (input_limb_32_col32)))
                    + ((input_limb_15_col15) * (input_limb_31_col31)))
                    + ((input_limb_16_col16) * (input_limb_30_col30)))
                    + ((input_limb_17_col17) * (input_limb_29_col29)))
                    + ((input_limb_18_col18) * (input_limb_28_col28)));
                let conv_tmp_9a554_20 = ((((((((((((((((((((((M31_0)
                    - (input_limb_75_col75))
                    + ((input_limb_0_col0) * (input_limb_47_col47)))
                    + ((input_limb_1_col1) * (input_limb_46_col46)))
                    + ((input_limb_2_col2) * (input_limb_45_col45)))
                    + ((input_limb_3_col3) * (input_limb_44_col44)))
                    + ((input_limb_4_col4) * (input_limb_43_col43)))
                    + ((input_limb_5_col5) * (input_limb_42_col42)))
                    + ((input_limb_6_col6) * (input_limb_41_col41)))
                    + ((input_limb_7_col7) * (input_limb_40_col40)))
                    + ((input_limb_8_col8) * (input_limb_39_col39)))
                    + ((input_limb_9_col9) * (input_limb_38_col38)))
                    + ((input_limb_10_col10) * (input_limb_37_col37)))
                    + ((input_limb_11_col11) * (input_limb_36_col36)))
                    + ((input_limb_12_col12) * (input_limb_35_col35)))
                    + ((input_limb_13_col13) * (input_limb_34_col34)))
                    + ((input_limb_14_col14) * (input_limb_33_col33)))
                    + ((input_limb_15_col15) * (input_limb_32_col32)))
                    + ((input_limb_16_col16) * (input_limb_31_col31)))
                    + ((input_limb_17_col17) * (input_limb_30_col30)))
                    + ((input_limb_18_col18) * (input_limb_29_col29)))
                    + ((input_limb_19_col19) * (input_limb_28_col28)));
                let conv_tmp_9a554_21 = (((((((((((((((((((((((M31_0)
                    - (input_limb_76_col76))
                    + ((input_limb_0_col0) * (input_limb_48_col48)))
                    + ((input_limb_1_col1) * (input_limb_47_col47)))
                    + ((input_limb_2_col2) * (input_limb_46_col46)))
                    + ((input_limb_3_col3) * (input_limb_45_col45)))
                    + ((input_limb_4_col4) * (input_limb_44_col44)))
                    + ((input_limb_5_col5) * (input_limb_43_col43)))
                    + ((input_limb_6_col6) * (input_limb_42_col42)))
                    + ((input_limb_7_col7) * (input_limb_41_col41)))
                    + ((input_limb_8_col8) * (input_limb_40_col40)))
                    + ((input_limb_9_col9) * (input_limb_39_col39)))
                    + ((input_limb_10_col10) * (input_limb_38_col38)))
                    + ((input_limb_11_col11) * (input_limb_37_col37)))
                    + ((input_limb_12_col12) * (input_limb_36_col36)))
                    + ((input_limb_13_col13) * (input_limb_35_col35)))
                    + ((input_limb_14_col14) * (input_limb_34_col34)))
                    + ((input_limb_15_col15) * (input_limb_33_col33)))
                    + ((input_limb_16_col16) * (input_limb_32_col32)))
                    + ((input_limb_17_col17) * (input_limb_31_col31)))
                    + ((input_limb_18_col18) * (input_limb_30_col30)))
                    + ((input_limb_19_col19) * (input_limb_29_col29)))
                    + ((input_limb_20_col20) * (input_limb_28_col28)));
                let conv_tmp_9a554_22 = ((((((((((((((((((((((((M31_0)
                    - (input_limb_77_col77))
                    + ((input_limb_0_col0) * (input_limb_49_col49)))
                    + ((input_limb_1_col1) * (input_limb_48_col48)))
                    + ((input_limb_2_col2) * (input_limb_47_col47)))
                    + ((input_limb_3_col3) * (input_limb_46_col46)))
                    + ((input_limb_4_col4) * (input_limb_45_col45)))
                    + ((input_limb_5_col5) * (input_limb_44_col44)))
                    + ((input_limb_6_col6) * (input_limb_43_col43)))
                    + ((input_limb_7_col7) * (input_limb_42_col42)))
                    + ((input_limb_8_col8) * (input_limb_41_col41)))
                    + ((input_limb_9_col9) * (input_limb_40_col40)))
                    + ((input_limb_10_col10) * (input_limb_39_col39)))
                    + ((input_limb_11_col11) * (input_limb_38_col38)))
                    + ((input_limb_12_col12) * (input_limb_37_col37)))
                    + ((input_limb_13_col13) * (input_limb_36_col36)))
                    + ((input_limb_14_col14) * (input_limb_35_col35)))
                    + ((input_limb_15_col15) * (input_limb_34_col34)))
                    + ((input_limb_16_col16) * (input_limb_33_col33)))
                    + ((input_limb_17_col17) * (input_limb_32_col32)))
                    + ((input_limb_18_col18) * (input_limb_31_col31)))
                    + ((input_limb_19_col19) * (input_limb_30_col30)))
                    + ((input_limb_20_col20) * (input_limb_29_col29)))
                    + ((input_limb_21_col21) * (input_limb_28_col28)));
                let conv_tmp_9a554_23 = (((((((((((((((((((((((((M31_0)
                    - (input_limb_78_col78))
                    + ((input_limb_0_col0) * (input_limb_50_col50)))
                    + ((input_limb_1_col1) * (input_limb_49_col49)))
                    + ((input_limb_2_col2) * (input_limb_48_col48)))
                    + ((input_limb_3_col3) * (input_limb_47_col47)))
                    + ((input_limb_4_col4) * (input_limb_46_col46)))
                    + ((input_limb_5_col5) * (input_limb_45_col45)))
                    + ((input_limb_6_col6) * (input_limb_44_col44)))
                    + ((input_limb_7_col7) * (input_limb_43_col43)))
                    + ((input_limb_8_col8) * (input_limb_42_col42)))
                    + ((input_limb_9_col9) * (input_limb_41_col41)))
                    + ((input_limb_10_col10) * (input_limb_40_col40)))
                    + ((input_limb_11_col11) * (input_limb_39_col39)))
                    + ((input_limb_12_col12) * (input_limb_38_col38)))
                    + ((input_limb_13_col13) * (input_limb_37_col37)))
                    + ((input_limb_14_col14) * (input_limb_36_col36)))
                    + ((input_limb_15_col15) * (input_limb_35_col35)))
                    + ((input_limb_16_col16) * (input_limb_34_col34)))
                    + ((input_limb_17_col17) * (input_limb_33_col33)))
                    + ((input_limb_18_col18) * (input_limb_32_col32)))
                    + ((input_limb_19_col19) * (input_limb_31_col31)))
                    + ((input_limb_20_col20) * (input_limb_30_col30)))
                    + ((input_limb_21_col21) * (input_limb_29_col29)))
                    + ((input_limb_22_col22) * (input_limb_28_col28)));
                let conv_tmp_9a554_24 = ((((((((((((((((((((((((((M31_0)
                    - (input_limb_79_col79))
                    + ((input_limb_0_col0) * (input_limb_51_col51)))
                    + ((input_limb_1_col1) * (input_limb_50_col50)))
                    + ((input_limb_2_col2) * (input_limb_49_col49)))
                    + ((input_limb_3_col3) * (input_limb_48_col48)))
                    + ((input_limb_4_col4) * (input_limb_47_col47)))
                    + ((input_limb_5_col5) * (input_limb_46_col46)))
                    + ((input_limb_6_col6) * (input_limb_45_col45)))
                    + ((input_limb_7_col7) * (input_limb_44_col44)))
                    + ((input_limb_8_col8) * (input_limb_43_col43)))
                    + ((input_limb_9_col9) * (input_limb_42_col42)))
                    + ((input_limb_10_col10) * (input_limb_41_col41)))
                    + ((input_limb_11_col11) * (input_limb_40_col40)))
                    + ((input_limb_12_col12) * (input_limb_39_col39)))
                    + ((input_limb_13_col13) * (input_limb_38_col38)))
                    + ((input_limb_14_col14) * (input_limb_37_col37)))
                    + ((input_limb_15_col15) * (input_limb_36_col36)))
                    + ((input_limb_16_col16) * (input_limb_35_col35)))
                    + ((input_limb_17_col17) * (input_limb_34_col34)))
                    + ((input_limb_18_col18) * (input_limb_33_col33)))
                    + ((input_limb_19_col19) * (input_limb_32_col32)))
                    + ((input_limb_20_col20) * (input_limb_31_col31)))
                    + ((input_limb_21_col21) * (input_limb_30_col30)))
                    + ((input_limb_22_col22) * (input_limb_29_col29)))
                    + ((input_limb_23_col23) * (input_limb_28_col28)));
                let conv_tmp_9a554_25 = (((((((((((((((((((((((((((M31_0)
                    - (input_limb_80_col80))
                    + ((input_limb_0_col0) * (input_limb_52_col52)))
                    + ((input_limb_1_col1) * (input_limb_51_col51)))
                    + ((input_limb_2_col2) * (input_limb_50_col50)))
                    + ((input_limb_3_col3) * (input_limb_49_col49)))
                    + ((input_limb_4_col4) * (input_limb_48_col48)))
                    + ((input_limb_5_col5) * (input_limb_47_col47)))
                    + ((input_limb_6_col6) * (input_limb_46_col46)))
                    + ((input_limb_7_col7) * (input_limb_45_col45)))
                    + ((input_limb_8_col8) * (input_limb_44_col44)))
                    + ((input_limb_9_col9) * (input_limb_43_col43)))
                    + ((input_limb_10_col10) * (input_limb_42_col42)))
                    + ((input_limb_11_col11) * (input_limb_41_col41)))
                    + ((input_limb_12_col12) * (input_limb_40_col40)))
                    + ((input_limb_13_col13) * (input_limb_39_col39)))
                    + ((input_limb_14_col14) * (input_limb_38_col38)))
                    + ((input_limb_15_col15) * (input_limb_37_col37)))
                    + ((input_limb_16_col16) * (input_limb_36_col36)))
                    + ((input_limb_17_col17) * (input_limb_35_col35)))
                    + ((input_limb_18_col18) * (input_limb_34_col34)))
                    + ((input_limb_19_col19) * (input_limb_33_col33)))
                    + ((input_limb_20_col20) * (input_limb_32_col32)))
                    + ((input_limb_21_col21) * (input_limb_31_col31)))
                    + ((input_limb_22_col22) * (input_limb_30_col30)))
                    + ((input_limb_23_col23) * (input_limb_29_col29)))
                    + ((input_limb_24_col24) * (input_limb_28_col28)));
                let conv_tmp_9a554_26 = ((((((((((((((((((((((((((((M31_0)
                    - (input_limb_81_col81))
                    + ((input_limb_0_col0) * (input_limb_53_col53)))
                    + ((input_limb_1_col1) * (input_limb_52_col52)))
                    + ((input_limb_2_col2) * (input_limb_51_col51)))
                    + ((input_limb_3_col3) * (input_limb_50_col50)))
                    + ((input_limb_4_col4) * (input_limb_49_col49)))
                    + ((input_limb_5_col5) * (input_limb_48_col48)))
                    + ((input_limb_6_col6) * (input_limb_47_col47)))
                    + ((input_limb_7_col7) * (input_limb_46_col46)))
                    + ((input_limb_8_col8) * (input_limb_45_col45)))
                    + ((input_limb_9_col9) * (input_limb_44_col44)))
                    + ((input_limb_10_col10) * (input_limb_43_col43)))
                    + ((input_limb_11_col11) * (input_limb_42_col42)))
                    + ((input_limb_12_col12) * (input_limb_41_col41)))
                    + ((input_limb_13_col13) * (input_limb_40_col40)))
                    + ((input_limb_14_col14) * (input_limb_39_col39)))
                    + ((input_limb_15_col15) * (input_limb_38_col38)))
                    + ((input_limb_16_col16) * (input_limb_37_col37)))
                    + ((input_limb_17_col17) * (input_limb_36_col36)))
                    + ((input_limb_18_col18) * (input_limb_35_col35)))
                    + ((input_limb_19_col19) * (input_limb_34_col34)))
                    + ((input_limb_20_col20) * (input_limb_33_col33)))
                    + ((input_limb_21_col21) * (input_limb_32_col32)))
                    + ((input_limb_22_col22) * (input_limb_31_col31)))
                    + ((input_limb_23_col23) * (input_limb_30_col30)))
                    + ((input_limb_24_col24) * (input_limb_29_col29)))
                    + ((input_limb_25_col25) * (input_limb_28_col28)));
                let conv_tmp_9a554_27 = (((((((((((((((((((((((((((((M31_0)
                    - (input_limb_82_col82))
                    + ((input_limb_0_col0) * (input_limb_54_col54)))
                    + ((input_limb_1_col1) * (input_limb_53_col53)))
                    + ((input_limb_2_col2) * (input_limb_52_col52)))
                    + ((input_limb_3_col3) * (input_limb_51_col51)))
                    + ((input_limb_4_col4) * (input_limb_50_col50)))
                    + ((input_limb_5_col5) * (input_limb_49_col49)))
                    + ((input_limb_6_col6) * (input_limb_48_col48)))
                    + ((input_limb_7_col7) * (input_limb_47_col47)))
                    + ((input_limb_8_col8) * (input_limb_46_col46)))
                    + ((input_limb_9_col9) * (input_limb_45_col45)))
                    + ((input_limb_10_col10) * (input_limb_44_col44)))
                    + ((input_limb_11_col11) * (input_limb_43_col43)))
                    + ((input_limb_12_col12) * (input_limb_42_col42)))
                    + ((input_limb_13_col13) * (input_limb_41_col41)))
                    + ((input_limb_14_col14) * (input_limb_40_col40)))
                    + ((input_limb_15_col15) * (input_limb_39_col39)))
                    + ((input_limb_16_col16) * (input_limb_38_col38)))
                    + ((input_limb_17_col17) * (input_limb_37_col37)))
                    + ((input_limb_18_col18) * (input_limb_36_col36)))
                    + ((input_limb_19_col19) * (input_limb_35_col35)))
                    + ((input_limb_20_col20) * (input_limb_34_col34)))
                    + ((input_limb_21_col21) * (input_limb_33_col33)))
                    + ((input_limb_22_col22) * (input_limb_32_col32)))
                    + ((input_limb_23_col23) * (input_limb_31_col31)))
                    + ((input_limb_24_col24) * (input_limb_30_col30)))
                    + ((input_limb_25_col25) * (input_limb_29_col29)))
                    + ((input_limb_26_col26) * (input_limb_28_col28)));
                let conv_tmp_9a554_28 = ((((((((((((((((((((((((((((((M31_0)
                    - (input_limb_83_col83))
                    + ((input_limb_0_col0) * (input_limb_55_col55)))
                    + ((input_limb_1_col1) * (input_limb_54_col54)))
                    + ((input_limb_2_col2) * (input_limb_53_col53)))
                    + ((input_limb_3_col3) * (input_limb_52_col52)))
                    + ((input_limb_4_col4) * (input_limb_51_col51)))
                    + ((input_limb_5_col5) * (input_limb_50_col50)))
                    + ((input_limb_6_col6) * (input_limb_49_col49)))
                    + ((input_limb_7_col7) * (input_limb_48_col48)))
                    + ((input_limb_8_col8) * (input_limb_47_col47)))
                    + ((input_limb_9_col9) * (input_limb_46_col46)))
                    + ((input_limb_10_col10) * (input_limb_45_col45)))
                    + ((input_limb_11_col11) * (input_limb_44_col44)))
                    + ((input_limb_12_col12) * (input_limb_43_col43)))
                    + ((input_limb_13_col13) * (input_limb_42_col42)))
                    + ((input_limb_14_col14) * (input_limb_41_col41)))
                    + ((input_limb_15_col15) * (input_limb_40_col40)))
                    + ((input_limb_16_col16) * (input_limb_39_col39)))
                    + ((input_limb_17_col17) * (input_limb_38_col38)))
                    + ((input_limb_18_col18) * (input_limb_37_col37)))
                    + ((input_limb_19_col19) * (input_limb_36_col36)))
                    + ((input_limb_20_col20) * (input_limb_35_col35)))
                    + ((input_limb_21_col21) * (input_limb_34_col34)))
                    + ((input_limb_22_col22) * (input_limb_33_col33)))
                    + ((input_limb_23_col23) * (input_limb_32_col32)))
                    + ((input_limb_24_col24) * (input_limb_31_col31)))
                    + ((input_limb_25_col25) * (input_limb_30_col30)))
                    + ((input_limb_26_col26) * (input_limb_29_col29)))
                    + ((input_limb_27_col27) * (input_limb_28_col28)));
                let conv_tmp_9a554_29 = ((((((((((((((((((((((((((((M31_0)
                    + ((input_limb_1_col1) * (input_limb_55_col55)))
                    + ((input_limb_2_col2) * (input_limb_54_col54)))
                    + ((input_limb_3_col3) * (input_limb_53_col53)))
                    + ((input_limb_4_col4) * (input_limb_52_col52)))
                    + ((input_limb_5_col5) * (input_limb_51_col51)))
                    + ((input_limb_6_col6) * (input_limb_50_col50)))
                    + ((input_limb_7_col7) * (input_limb_49_col49)))
                    + ((input_limb_8_col8) * (input_limb_48_col48)))
                    + ((input_limb_9_col9) * (input_limb_47_col47)))
                    + ((input_limb_10_col10) * (input_limb_46_col46)))
                    + ((input_limb_11_col11) * (input_limb_45_col45)))
                    + ((input_limb_12_col12) * (input_limb_44_col44)))
                    + ((input_limb_13_col13) * (input_limb_43_col43)))
                    + ((input_limb_14_col14) * (input_limb_42_col42)))
                    + ((input_limb_15_col15) * (input_limb_41_col41)))
                    + ((input_limb_16_col16) * (input_limb_40_col40)))
                    + ((input_limb_17_col17) * (input_limb_39_col39)))
                    + ((input_limb_18_col18) * (input_limb_38_col38)))
                    + ((input_limb_19_col19) * (input_limb_37_col37)))
                    + ((input_limb_20_col20) * (input_limb_36_col36)))
                    + ((input_limb_21_col21) * (input_limb_35_col35)))
                    + ((input_limb_22_col22) * (input_limb_34_col34)))
                    + ((input_limb_23_col23) * (input_limb_33_col33)))
                    + ((input_limb_24_col24) * (input_limb_32_col32)))
                    + ((input_limb_25_col25) * (input_limb_31_col31)))
                    + ((input_limb_26_col26) * (input_limb_30_col30)))
                    + ((input_limb_27_col27) * (input_limb_29_col29)));
                let conv_tmp_9a554_30 = (((((((((((((((((((((((((((M31_0)
                    + ((input_limb_2_col2) * (input_limb_55_col55)))
                    + ((input_limb_3_col3) * (input_limb_54_col54)))
                    + ((input_limb_4_col4) * (input_limb_53_col53)))
                    + ((input_limb_5_col5) * (input_limb_52_col52)))
                    + ((input_limb_6_col6) * (input_limb_51_col51)))
                    + ((input_limb_7_col7) * (input_limb_50_col50)))
                    + ((input_limb_8_col8) * (input_limb_49_col49)))
                    + ((input_limb_9_col9) * (input_limb_48_col48)))
                    + ((input_limb_10_col10) * (input_limb_47_col47)))
                    + ((input_limb_11_col11) * (input_limb_46_col46)))
                    + ((input_limb_12_col12) * (input_limb_45_col45)))
                    + ((input_limb_13_col13) * (input_limb_44_col44)))
                    + ((input_limb_14_col14) * (input_limb_43_col43)))
                    + ((input_limb_15_col15) * (input_limb_42_col42)))
                    + ((input_limb_16_col16) * (input_limb_41_col41)))
                    + ((input_limb_17_col17) * (input_limb_40_col40)))
                    + ((input_limb_18_col18) * (input_limb_39_col39)))
                    + ((input_limb_19_col19) * (input_limb_38_col38)))
                    + ((input_limb_20_col20) * (input_limb_37_col37)))
                    + ((input_limb_21_col21) * (input_limb_36_col36)))
                    + ((input_limb_22_col22) * (input_limb_35_col35)))
                    + ((input_limb_23_col23) * (input_limb_34_col34)))
                    + ((input_limb_24_col24) * (input_limb_33_col33)))
                    + ((input_limb_25_col25) * (input_limb_32_col32)))
                    + ((input_limb_26_col26) * (input_limb_31_col31)))
                    + ((input_limb_27_col27) * (input_limb_30_col30)));
                let conv_tmp_9a554_31 = ((((((((((((((((((((((((((M31_0)
                    + ((input_limb_3_col3) * (input_limb_55_col55)))
                    + ((input_limb_4_col4) * (input_limb_54_col54)))
                    + ((input_limb_5_col5) * (input_limb_53_col53)))
                    + ((input_limb_6_col6) * (input_limb_52_col52)))
                    + ((input_limb_7_col7) * (input_limb_51_col51)))
                    + ((input_limb_8_col8) * (input_limb_50_col50)))
                    + ((input_limb_9_col9) * (input_limb_49_col49)))
                    + ((input_limb_10_col10) * (input_limb_48_col48)))
                    + ((input_limb_11_col11) * (input_limb_47_col47)))
                    + ((input_limb_12_col12) * (input_limb_46_col46)))
                    + ((input_limb_13_col13) * (input_limb_45_col45)))
                    + ((input_limb_14_col14) * (input_limb_44_col44)))
                    + ((input_limb_15_col15) * (input_limb_43_col43)))
                    + ((input_limb_16_col16) * (input_limb_42_col42)))
                    + ((input_limb_17_col17) * (input_limb_41_col41)))
                    + ((input_limb_18_col18) * (input_limb_40_col40)))
                    + ((input_limb_19_col19) * (input_limb_39_col39)))
                    + ((input_limb_20_col20) * (input_limb_38_col38)))
                    + ((input_limb_21_col21) * (input_limb_37_col37)))
                    + ((input_limb_22_col22) * (input_limb_36_col36)))
                    + ((input_limb_23_col23) * (input_limb_35_col35)))
                    + ((input_limb_24_col24) * (input_limb_34_col34)))
                    + ((input_limb_25_col25) * (input_limb_33_col33)))
                    + ((input_limb_26_col26) * (input_limb_32_col32)))
                    + ((input_limb_27_col27) * (input_limb_31_col31)));
                let conv_tmp_9a554_32 = (((((((((((((((((((((((((M31_0)
                    + ((input_limb_4_col4) * (input_limb_55_col55)))
                    + ((input_limb_5_col5) * (input_limb_54_col54)))
                    + ((input_limb_6_col6) * (input_limb_53_col53)))
                    + ((input_limb_7_col7) * (input_limb_52_col52)))
                    + ((input_limb_8_col8) * (input_limb_51_col51)))
                    + ((input_limb_9_col9) * (input_limb_50_col50)))
                    + ((input_limb_10_col10) * (input_limb_49_col49)))
                    + ((input_limb_11_col11) * (input_limb_48_col48)))
                    + ((input_limb_12_col12) * (input_limb_47_col47)))
                    + ((input_limb_13_col13) * (input_limb_46_col46)))
                    + ((input_limb_14_col14) * (input_limb_45_col45)))
                    + ((input_limb_15_col15) * (input_limb_44_col44)))
                    + ((input_limb_16_col16) * (input_limb_43_col43)))
                    + ((input_limb_17_col17) * (input_limb_42_col42)))
                    + ((input_limb_18_col18) * (input_limb_41_col41)))
                    + ((input_limb_19_col19) * (input_limb_40_col40)))
                    + ((input_limb_20_col20) * (input_limb_39_col39)))
                    + ((input_limb_21_col21) * (input_limb_38_col38)))
                    + ((input_limb_22_col22) * (input_limb_37_col37)))
                    + ((input_limb_23_col23) * (input_limb_36_col36)))
                    + ((input_limb_24_col24) * (input_limb_35_col35)))
                    + ((input_limb_25_col25) * (input_limb_34_col34)))
                    + ((input_limb_26_col26) * (input_limb_33_col33)))
                    + ((input_limb_27_col27) * (input_limb_32_col32)));
                let conv_tmp_9a554_33 = ((((((((((((((((((((((((M31_0)
                    + ((input_limb_5_col5) * (input_limb_55_col55)))
                    + ((input_limb_6_col6) * (input_limb_54_col54)))
                    + ((input_limb_7_col7) * (input_limb_53_col53)))
                    + ((input_limb_8_col8) * (input_limb_52_col52)))
                    + ((input_limb_9_col9) * (input_limb_51_col51)))
                    + ((input_limb_10_col10) * (input_limb_50_col50)))
                    + ((input_limb_11_col11) * (input_limb_49_col49)))
                    + ((input_limb_12_col12) * (input_limb_48_col48)))
                    + ((input_limb_13_col13) * (input_limb_47_col47)))
                    + ((input_limb_14_col14) * (input_limb_46_col46)))
                    + ((input_limb_15_col15) * (input_limb_45_col45)))
                    + ((input_limb_16_col16) * (input_limb_44_col44)))
                    + ((input_limb_17_col17) * (input_limb_43_col43)))
                    + ((input_limb_18_col18) * (input_limb_42_col42)))
                    + ((input_limb_19_col19) * (input_limb_41_col41)))
                    + ((input_limb_20_col20) * (input_limb_40_col40)))
                    + ((input_limb_21_col21) * (input_limb_39_col39)))
                    + ((input_limb_22_col22) * (input_limb_38_col38)))
                    + ((input_limb_23_col23) * (input_limb_37_col37)))
                    + ((input_limb_24_col24) * (input_limb_36_col36)))
                    + ((input_limb_25_col25) * (input_limb_35_col35)))
                    + ((input_limb_26_col26) * (input_limb_34_col34)))
                    + ((input_limb_27_col27) * (input_limb_33_col33)));
                let conv_tmp_9a554_34 = (((((((((((((((((((((((M31_0)
                    + ((input_limb_6_col6) * (input_limb_55_col55)))
                    + ((input_limb_7_col7) * (input_limb_54_col54)))
                    + ((input_limb_8_col8) * (input_limb_53_col53)))
                    + ((input_limb_9_col9) * (input_limb_52_col52)))
                    + ((input_limb_10_col10) * (input_limb_51_col51)))
                    + ((input_limb_11_col11) * (input_limb_50_col50)))
                    + ((input_limb_12_col12) * (input_limb_49_col49)))
                    + ((input_limb_13_col13) * (input_limb_48_col48)))
                    + ((input_limb_14_col14) * (input_limb_47_col47)))
                    + ((input_limb_15_col15) * (input_limb_46_col46)))
                    + ((input_limb_16_col16) * (input_limb_45_col45)))
                    + ((input_limb_17_col17) * (input_limb_44_col44)))
                    + ((input_limb_18_col18) * (input_limb_43_col43)))
                    + ((input_limb_19_col19) * (input_limb_42_col42)))
                    + ((input_limb_20_col20) * (input_limb_41_col41)))
                    + ((input_limb_21_col21) * (input_limb_40_col40)))
                    + ((input_limb_22_col22) * (input_limb_39_col39)))
                    + ((input_limb_23_col23) * (input_limb_38_col38)))
                    + ((input_limb_24_col24) * (input_limb_37_col37)))
                    + ((input_limb_25_col25) * (input_limb_36_col36)))
                    + ((input_limb_26_col26) * (input_limb_35_col35)))
                    + ((input_limb_27_col27) * (input_limb_34_col34)));
                let conv_tmp_9a554_35 = ((((((((((((((((((((((M31_0)
                    + ((input_limb_7_col7) * (input_limb_55_col55)))
                    + ((input_limb_8_col8) * (input_limb_54_col54)))
                    + ((input_limb_9_col9) * (input_limb_53_col53)))
                    + ((input_limb_10_col10) * (input_limb_52_col52)))
                    + ((input_limb_11_col11) * (input_limb_51_col51)))
                    + ((input_limb_12_col12) * (input_limb_50_col50)))
                    + ((input_limb_13_col13) * (input_limb_49_col49)))
                    + ((input_limb_14_col14) * (input_limb_48_col48)))
                    + ((input_limb_15_col15) * (input_limb_47_col47)))
                    + ((input_limb_16_col16) * (input_limb_46_col46)))
                    + ((input_limb_17_col17) * (input_limb_45_col45)))
                    + ((input_limb_18_col18) * (input_limb_44_col44)))
                    + ((input_limb_19_col19) * (input_limb_43_col43)))
                    + ((input_limb_20_col20) * (input_limb_42_col42)))
                    + ((input_limb_21_col21) * (input_limb_41_col41)))
                    + ((input_limb_22_col22) * (input_limb_40_col40)))
                    + ((input_limb_23_col23) * (input_limb_39_col39)))
                    + ((input_limb_24_col24) * (input_limb_38_col38)))
                    + ((input_limb_25_col25) * (input_limb_37_col37)))
                    + ((input_limb_26_col26) * (input_limb_36_col36)))
                    + ((input_limb_27_col27) * (input_limb_35_col35)));
                let conv_tmp_9a554_36 = (((((((((((((((((((((M31_0)
                    + ((input_limb_8_col8) * (input_limb_55_col55)))
                    + ((input_limb_9_col9) * (input_limb_54_col54)))
                    + ((input_limb_10_col10) * (input_limb_53_col53)))
                    + ((input_limb_11_col11) * (input_limb_52_col52)))
                    + ((input_limb_12_col12) * (input_limb_51_col51)))
                    + ((input_limb_13_col13) * (input_limb_50_col50)))
                    + ((input_limb_14_col14) * (input_limb_49_col49)))
                    + ((input_limb_15_col15) * (input_limb_48_col48)))
                    + ((input_limb_16_col16) * (input_limb_47_col47)))
                    + ((input_limb_17_col17) * (input_limb_46_col46)))
                    + ((input_limb_18_col18) * (input_limb_45_col45)))
                    + ((input_limb_19_col19) * (input_limb_44_col44)))
                    + ((input_limb_20_col20) * (input_limb_43_col43)))
                    + ((input_limb_21_col21) * (input_limb_42_col42)))
                    + ((input_limb_22_col22) * (input_limb_41_col41)))
                    + ((input_limb_23_col23) * (input_limb_40_col40)))
                    + ((input_limb_24_col24) * (input_limb_39_col39)))
                    + ((input_limb_25_col25) * (input_limb_38_col38)))
                    + ((input_limb_26_col26) * (input_limb_37_col37)))
                    + ((input_limb_27_col27) * (input_limb_36_col36)));
                let conv_tmp_9a554_37 = ((((((((((((((((((((M31_0)
                    + ((input_limb_9_col9) * (input_limb_55_col55)))
                    + ((input_limb_10_col10) * (input_limb_54_col54)))
                    + ((input_limb_11_col11) * (input_limb_53_col53)))
                    + ((input_limb_12_col12) * (input_limb_52_col52)))
                    + ((input_limb_13_col13) * (input_limb_51_col51)))
                    + ((input_limb_14_col14) * (input_limb_50_col50)))
                    + ((input_limb_15_col15) * (input_limb_49_col49)))
                    + ((input_limb_16_col16) * (input_limb_48_col48)))
                    + ((input_limb_17_col17) * (input_limb_47_col47)))
                    + ((input_limb_18_col18) * (input_limb_46_col46)))
                    + ((input_limb_19_col19) * (input_limb_45_col45)))
                    + ((input_limb_20_col20) * (input_limb_44_col44)))
                    + ((input_limb_21_col21) * (input_limb_43_col43)))
                    + ((input_limb_22_col22) * (input_limb_42_col42)))
                    + ((input_limb_23_col23) * (input_limb_41_col41)))
                    + ((input_limb_24_col24) * (input_limb_40_col40)))
                    + ((input_limb_25_col25) * (input_limb_39_col39)))
                    + ((input_limb_26_col26) * (input_limb_38_col38)))
                    + ((input_limb_27_col27) * (input_limb_37_col37)));
                let conv_tmp_9a554_38 = (((((((((((((((((((M31_0)
                    + ((input_limb_10_col10) * (input_limb_55_col55)))
                    + ((input_limb_11_col11) * (input_limb_54_col54)))
                    + ((input_limb_12_col12) * (input_limb_53_col53)))
                    + ((input_limb_13_col13) * (input_limb_52_col52)))
                    + ((input_limb_14_col14) * (input_limb_51_col51)))
                    + ((input_limb_15_col15) * (input_limb_50_col50)))
                    + ((input_limb_16_col16) * (input_limb_49_col49)))
                    + ((input_limb_17_col17) * (input_limb_48_col48)))
                    + ((input_limb_18_col18) * (input_limb_47_col47)))
                    + ((input_limb_19_col19) * (input_limb_46_col46)))
                    + ((input_limb_20_col20) * (input_limb_45_col45)))
                    + ((input_limb_21_col21) * (input_limb_44_col44)))
                    + ((input_limb_22_col22) * (input_limb_43_col43)))
                    + ((input_limb_23_col23) * (input_limb_42_col42)))
                    + ((input_limb_24_col24) * (input_limb_41_col41)))
                    + ((input_limb_25_col25) * (input_limb_40_col40)))
                    + ((input_limb_26_col26) * (input_limb_39_col39)))
                    + ((input_limb_27_col27) * (input_limb_38_col38)));
                let conv_tmp_9a554_39 = ((((((((((((((((((M31_0)
                    + ((input_limb_11_col11) * (input_limb_55_col55)))
                    + ((input_limb_12_col12) * (input_limb_54_col54)))
                    + ((input_limb_13_col13) * (input_limb_53_col53)))
                    + ((input_limb_14_col14) * (input_limb_52_col52)))
                    + ((input_limb_15_col15) * (input_limb_51_col51)))
                    + ((input_limb_16_col16) * (input_limb_50_col50)))
                    + ((input_limb_17_col17) * (input_limb_49_col49)))
                    + ((input_limb_18_col18) * (input_limb_48_col48)))
                    + ((input_limb_19_col19) * (input_limb_47_col47)))
                    + ((input_limb_20_col20) * (input_limb_46_col46)))
                    + ((input_limb_21_col21) * (input_limb_45_col45)))
                    + ((input_limb_22_col22) * (input_limb_44_col44)))
                    + ((input_limb_23_col23) * (input_limb_43_col43)))
                    + ((input_limb_24_col24) * (input_limb_42_col42)))
                    + ((input_limb_25_col25) * (input_limb_41_col41)))
                    + ((input_limb_26_col26) * (input_limb_40_col40)))
                    + ((input_limb_27_col27) * (input_limb_39_col39)));
                let conv_tmp_9a554_40 = (((((((((((((((((M31_0)
                    + ((input_limb_12_col12) * (input_limb_55_col55)))
                    + ((input_limb_13_col13) * (input_limb_54_col54)))
                    + ((input_limb_14_col14) * (input_limb_53_col53)))
                    + ((input_limb_15_col15) * (input_limb_52_col52)))
                    + ((input_limb_16_col16) * (input_limb_51_col51)))
                    + ((input_limb_17_col17) * (input_limb_50_col50)))
                    + ((input_limb_18_col18) * (input_limb_49_col49)))
                    + ((input_limb_19_col19) * (input_limb_48_col48)))
                    + ((input_limb_20_col20) * (input_limb_47_col47)))
                    + ((input_limb_21_col21) * (input_limb_46_col46)))
                    + ((input_limb_22_col22) * (input_limb_45_col45)))
                    + ((input_limb_23_col23) * (input_limb_44_col44)))
                    + ((input_limb_24_col24) * (input_limb_43_col43)))
                    + ((input_limb_25_col25) * (input_limb_42_col42)))
                    + ((input_limb_26_col26) * (input_limb_41_col41)))
                    + ((input_limb_27_col27) * (input_limb_40_col40)));
                let conv_tmp_9a554_41 = ((((((((((((((((M31_0)
                    + ((input_limb_13_col13) * (input_limb_55_col55)))
                    + ((input_limb_14_col14) * (input_limb_54_col54)))
                    + ((input_limb_15_col15) * (input_limb_53_col53)))
                    + ((input_limb_16_col16) * (input_limb_52_col52)))
                    + ((input_limb_17_col17) * (input_limb_51_col51)))
                    + ((input_limb_18_col18) * (input_limb_50_col50)))
                    + ((input_limb_19_col19) * (input_limb_49_col49)))
                    + ((input_limb_20_col20) * (input_limb_48_col48)))
                    + ((input_limb_21_col21) * (input_limb_47_col47)))
                    + ((input_limb_22_col22) * (input_limb_46_col46)))
                    + ((input_limb_23_col23) * (input_limb_45_col45)))
                    + ((input_limb_24_col24) * (input_limb_44_col44)))
                    + ((input_limb_25_col25) * (input_limb_43_col43)))
                    + ((input_limb_26_col26) * (input_limb_42_col42)))
                    + ((input_limb_27_col27) * (input_limb_41_col41)));
                let conv_tmp_9a554_42 = (((((((((((((((M31_0)
                    + ((input_limb_14_col14) * (input_limb_55_col55)))
                    + ((input_limb_15_col15) * (input_limb_54_col54)))
                    + ((input_limb_16_col16) * (input_limb_53_col53)))
                    + ((input_limb_17_col17) * (input_limb_52_col52)))
                    + ((input_limb_18_col18) * (input_limb_51_col51)))
                    + ((input_limb_19_col19) * (input_limb_50_col50)))
                    + ((input_limb_20_col20) * (input_limb_49_col49)))
                    + ((input_limb_21_col21) * (input_limb_48_col48)))
                    + ((input_limb_22_col22) * (input_limb_47_col47)))
                    + ((input_limb_23_col23) * (input_limb_46_col46)))
                    + ((input_limb_24_col24) * (input_limb_45_col45)))
                    + ((input_limb_25_col25) * (input_limb_44_col44)))
                    + ((input_limb_26_col26) * (input_limb_43_col43)))
                    + ((input_limb_27_col27) * (input_limb_42_col42)));
                let conv_tmp_9a554_43 = ((((((((((((((M31_0)
                    + ((input_limb_15_col15) * (input_limb_55_col55)))
                    + ((input_limb_16_col16) * (input_limb_54_col54)))
                    + ((input_limb_17_col17) * (input_limb_53_col53)))
                    + ((input_limb_18_col18) * (input_limb_52_col52)))
                    + ((input_limb_19_col19) * (input_limb_51_col51)))
                    + ((input_limb_20_col20) * (input_limb_50_col50)))
                    + ((input_limb_21_col21) * (input_limb_49_col49)))
                    + ((input_limb_22_col22) * (input_limb_48_col48)))
                    + ((input_limb_23_col23) * (input_limb_47_col47)))
                    + ((input_limb_24_col24) * (input_limb_46_col46)))
                    + ((input_limb_25_col25) * (input_limb_45_col45)))
                    + ((input_limb_26_col26) * (input_limb_44_col44)))
                    + ((input_limb_27_col27) * (input_limb_43_col43)));
                let conv_tmp_9a554_44 = (((((((((((((M31_0)
                    + ((input_limb_16_col16) * (input_limb_55_col55)))
                    + ((input_limb_17_col17) * (input_limb_54_col54)))
                    + ((input_limb_18_col18) * (input_limb_53_col53)))
                    + ((input_limb_19_col19) * (input_limb_52_col52)))
                    + ((input_limb_20_col20) * (input_limb_51_col51)))
                    + ((input_limb_21_col21) * (input_limb_50_col50)))
                    + ((input_limb_22_col22) * (input_limb_49_col49)))
                    + ((input_limb_23_col23) * (input_limb_48_col48)))
                    + ((input_limb_24_col24) * (input_limb_47_col47)))
                    + ((input_limb_25_col25) * (input_limb_46_col46)))
                    + ((input_limb_26_col26) * (input_limb_45_col45)))
                    + ((input_limb_27_col27) * (input_limb_44_col44)));
                let conv_tmp_9a554_45 = ((((((((((((M31_0)
                    + ((input_limb_17_col17) * (input_limb_55_col55)))
                    + ((input_limb_18_col18) * (input_limb_54_col54)))
                    + ((input_limb_19_col19) * (input_limb_53_col53)))
                    + ((input_limb_20_col20) * (input_limb_52_col52)))
                    + ((input_limb_21_col21) * (input_limb_51_col51)))
                    + ((input_limb_22_col22) * (input_limb_50_col50)))
                    + ((input_limb_23_col23) * (input_limb_49_col49)))
                    + ((input_limb_24_col24) * (input_limb_48_col48)))
                    + ((input_limb_25_col25) * (input_limb_47_col47)))
                    + ((input_limb_26_col26) * (input_limb_46_col46)))
                    + ((input_limb_27_col27) * (input_limb_45_col45)));
                let conv_tmp_9a554_46 = (((((((((((M31_0)
                    + ((input_limb_18_col18) * (input_limb_55_col55)))
                    + ((input_limb_19_col19) * (input_limb_54_col54)))
                    + ((input_limb_20_col20) * (input_limb_53_col53)))
                    + ((input_limb_21_col21) * (input_limb_52_col52)))
                    + ((input_limb_22_col22) * (input_limb_51_col51)))
                    + ((input_limb_23_col23) * (input_limb_50_col50)))
                    + ((input_limb_24_col24) * (input_limb_49_col49)))
                    + ((input_limb_25_col25) * (input_limb_48_col48)))
                    + ((input_limb_26_col26) * (input_limb_47_col47)))
                    + ((input_limb_27_col27) * (input_limb_46_col46)));
                let conv_tmp_9a554_47 = ((((((((((M31_0)
                    + ((input_limb_19_col19) * (input_limb_55_col55)))
                    + ((input_limb_20_col20) * (input_limb_54_col54)))
                    + ((input_limb_21_col21) * (input_limb_53_col53)))
                    + ((input_limb_22_col22) * (input_limb_52_col52)))
                    + ((input_limb_23_col23) * (input_limb_51_col51)))
                    + ((input_limb_24_col24) * (input_limb_50_col50)))
                    + ((input_limb_25_col25) * (input_limb_49_col49)))
                    + ((input_limb_26_col26) * (input_limb_48_col48)))
                    + ((input_limb_27_col27) * (input_limb_47_col47)));
                let conv_tmp_9a554_48 = (((((((((M31_0)
                    + ((input_limb_20_col20) * (input_limb_55_col55)))
                    + ((input_limb_21_col21) * (input_limb_54_col54)))
                    + ((input_limb_22_col22) * (input_limb_53_col53)))
                    + ((input_limb_23_col23) * (input_limb_52_col52)))
                    + ((input_limb_24_col24) * (input_limb_51_col51)))
                    + ((input_limb_25_col25) * (input_limb_50_col50)))
                    + ((input_limb_26_col26) * (input_limb_49_col49)))
                    + ((input_limb_27_col27) * (input_limb_48_col48)));
                let conv_tmp_9a554_49 = ((((((((M31_0)
                    + ((input_limb_21_col21) * (input_limb_55_col55)))
                    + ((input_limb_22_col22) * (input_limb_54_col54)))
                    + ((input_limb_23_col23) * (input_limb_53_col53)))
                    + ((input_limb_24_col24) * (input_limb_52_col52)))
                    + ((input_limb_25_col25) * (input_limb_51_col51)))
                    + ((input_limb_26_col26) * (input_limb_50_col50)))
                    + ((input_limb_27_col27) * (input_limb_49_col49)));
                let conv_tmp_9a554_50 = (((((((M31_0)
                    + ((input_limb_22_col22) * (input_limb_55_col55)))
                    + ((input_limb_23_col23) * (input_limb_54_col54)))
                    + ((input_limb_24_col24) * (input_limb_53_col53)))
                    + ((input_limb_25_col25) * (input_limb_52_col52)))
                    + ((input_limb_26_col26) * (input_limb_51_col51)))
                    + ((input_limb_27_col27) * (input_limb_50_col50)));
                let conv_tmp_9a554_51 = ((((((M31_0)
                    + ((input_limb_23_col23) * (input_limb_55_col55)))
                    + ((input_limb_24_col24) * (input_limb_54_col54)))
                    + ((input_limb_25_col25) * (input_limb_53_col53)))
                    + ((input_limb_26_col26) * (input_limb_52_col52)))
                    + ((input_limb_27_col27) * (input_limb_51_col51)));
                let conv_tmp_9a554_52 = (((((M31_0)
                    + ((input_limb_24_col24) * (input_limb_55_col55)))
                    + ((input_limb_25_col25) * (input_limb_54_col54)))
                    + ((input_limb_26_col26) * (input_limb_53_col53)))
                    + ((input_limb_27_col27) * (input_limb_52_col52)));
                let conv_tmp_9a554_53 = ((((M31_0)
                    + ((input_limb_25_col25) * (input_limb_55_col55)))
                    + ((input_limb_26_col26) * (input_limb_54_col54)))
                    + ((input_limb_27_col27) * (input_limb_53_col53)));
                let conv_tmp_9a554_54 = (((M31_0)
                    + ((input_limb_26_col26) * (input_limb_55_col55)))
                    + ((input_limb_27_col27) * (input_limb_54_col54)));
                let conv_tmp_9a554_55 = ((M31_0) + ((input_limb_27_col27) * (input_limb_55_col55)));
                let conv_mod_tmp_9a554_56 = ((((M31_0) + ((M31_32) * (conv_tmp_9a554_1)))
                    - ((M31_4) * (conv_tmp_9a554_22)))
                    + ((M31_8) * (conv_tmp_9a554_50)));
                let conv_mod_tmp_9a554_57 = (((((M31_0) + ((M31_1) * (conv_tmp_9a554_1)))
                    + ((M31_32) * (conv_tmp_9a554_2)))
                    - ((M31_4) * (conv_tmp_9a554_23)))
                    + ((M31_8) * (conv_tmp_9a554_51)));
                let conv_mod_tmp_9a554_58 = (((((M31_0) + ((M31_1) * (conv_tmp_9a554_2)))
                    + ((M31_32) * (conv_tmp_9a554_3)))
                    - ((M31_4) * (conv_tmp_9a554_24)))
                    + ((M31_8) * (conv_tmp_9a554_52)));
                let conv_mod_tmp_9a554_59 = (((((M31_0) + ((M31_1) * (conv_tmp_9a554_3)))
                    + ((M31_32) * (conv_tmp_9a554_4)))
                    - ((M31_4) * (conv_tmp_9a554_25)))
                    + ((M31_8) * (conv_tmp_9a554_53)));
                let conv_mod_tmp_9a554_60 = (((((M31_0) + ((M31_1) * (conv_tmp_9a554_4)))
                    + ((M31_32) * (conv_tmp_9a554_5)))
                    - ((M31_4) * (conv_tmp_9a554_26)))
                    + ((M31_8) * (conv_tmp_9a554_54)));
                let conv_mod_tmp_9a554_61 = (((((M31_0) + ((M31_1) * (conv_tmp_9a554_5)))
                    + ((M31_32) * (conv_tmp_9a554_6)))
                    - ((M31_4) * (conv_tmp_9a554_27)))
                    + ((M31_8) * (conv_tmp_9a554_55)));
                let conv_mod_tmp_9a554_62 = ((((M31_0) + ((M31_1) * (conv_tmp_9a554_6)))
                    + ((M31_32) * (conv_tmp_9a554_7)))
                    - ((M31_4) * (conv_tmp_9a554_28)));
                let conv_mod_tmp_9a554_63 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_1)))
                    + ((M31_1) * (conv_tmp_9a554_7)))
                    + ((M31_32) * (conv_tmp_9a554_8)))
                    - ((M31_4) * (conv_tmp_9a554_29)));
                let conv_mod_tmp_9a554_64 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_2)))
                    + ((M31_1) * (conv_tmp_9a554_8)))
                    + ((M31_32) * (conv_tmp_9a554_9)))
                    - ((M31_4) * (conv_tmp_9a554_30)));
                let conv_mod_tmp_9a554_65 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_3)))
                    + ((M31_1) * (conv_tmp_9a554_9)))
                    + ((M31_32) * (conv_tmp_9a554_10)))
                    - ((M31_4) * (conv_tmp_9a554_31)));
                let conv_mod_tmp_9a554_66 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_4)))
                    + ((M31_1) * (conv_tmp_9a554_10)))
                    + ((M31_32) * (conv_tmp_9a554_11)))
                    - ((M31_4) * (conv_tmp_9a554_32)));
                let conv_mod_tmp_9a554_67 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_5)))
                    + ((M31_1) * (conv_tmp_9a554_11)))
                    + ((M31_32) * (conv_tmp_9a554_12)))
                    - ((M31_4) * (conv_tmp_9a554_33)));
                let conv_mod_tmp_9a554_68 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_6)))
                    + ((M31_1) * (conv_tmp_9a554_12)))
                    + ((M31_32) * (conv_tmp_9a554_13)))
                    - ((M31_4) * (conv_tmp_9a554_34)));
                let conv_mod_tmp_9a554_69 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_7)))
                    + ((M31_1) * (conv_tmp_9a554_13)))
                    + ((M31_32) * (conv_tmp_9a554_14)))
                    - ((M31_4) * (conv_tmp_9a554_35)));
                let conv_mod_tmp_9a554_70 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_8)))
                    + ((M31_1) * (conv_tmp_9a554_14)))
                    + ((M31_32) * (conv_tmp_9a554_15)))
                    - ((M31_4) * (conv_tmp_9a554_36)));
                let conv_mod_tmp_9a554_71 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_9)))
                    + ((M31_1) * (conv_tmp_9a554_15)))
                    + ((M31_32) * (conv_tmp_9a554_16)))
                    - ((M31_4) * (conv_tmp_9a554_37)));
                let conv_mod_tmp_9a554_72 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_10)))
                    + ((M31_1) * (conv_tmp_9a554_16)))
                    + ((M31_32) * (conv_tmp_9a554_17)))
                    - ((M31_4) * (conv_tmp_9a554_38)));
                let conv_mod_tmp_9a554_73 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_11)))
                    + ((M31_1) * (conv_tmp_9a554_17)))
                    + ((M31_32) * (conv_tmp_9a554_18)))
                    - ((M31_4) * (conv_tmp_9a554_39)));
                let conv_mod_tmp_9a554_74 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_12)))
                    + ((M31_1) * (conv_tmp_9a554_18)))
                    + ((M31_32) * (conv_tmp_9a554_19)))
                    - ((M31_4) * (conv_tmp_9a554_40)));
                let conv_mod_tmp_9a554_75 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_13)))
                    + ((M31_1) * (conv_tmp_9a554_19)))
                    + ((M31_32) * (conv_tmp_9a554_20)))
                    - ((M31_4) * (conv_tmp_9a554_41)));
                let conv_mod_tmp_9a554_76 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_14)))
                    + ((M31_1) * (conv_tmp_9a554_20)))
                    + ((M31_32) * (conv_tmp_9a554_21)))
                    - ((M31_4) * (conv_tmp_9a554_42)));
                let conv_mod_tmp_9a554_77 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_15)))
                    + ((M31_1) * (conv_tmp_9a554_21)))
                    - ((M31_4) * (conv_tmp_9a554_43)))
                    + ((M31_64) * (conv_tmp_9a554_50)));
                let conv_mod_tmp_9a554_78 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_16)))
                    - ((M31_4) * (conv_tmp_9a554_44)))
                    + ((M31_2) * (conv_tmp_9a554_50)))
                    + ((M31_64) * (conv_tmp_9a554_51)));
                let conv_mod_tmp_9a554_79 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_17)))
                    - ((M31_4) * (conv_tmp_9a554_45)))
                    + ((M31_2) * (conv_tmp_9a554_51)))
                    + ((M31_64) * (conv_tmp_9a554_52)));
                let conv_mod_tmp_9a554_80 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_18)))
                    - ((M31_4) * (conv_tmp_9a554_46)))
                    + ((M31_2) * (conv_tmp_9a554_52)))
                    + ((M31_64) * (conv_tmp_9a554_53)));
                let conv_mod_tmp_9a554_81 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_19)))
                    - ((M31_4) * (conv_tmp_9a554_47)))
                    + ((M31_2) * (conv_tmp_9a554_53)))
                    + ((M31_64) * (conv_tmp_9a554_54)));
                let conv_mod_tmp_9a554_82 = (((((M31_0) + ((M31_2) * (conv_tmp_9a554_20)))
                    - ((M31_4) * (conv_tmp_9a554_48)))
                    + ((M31_2) * (conv_tmp_9a554_54)))
                    + ((M31_64) * (conv_tmp_9a554_55)));
                let conv_mod_tmp_9a554_83 = ((((M31_0) + ((M31_2) * (conv_tmp_9a554_21)))
                    - ((M31_4) * (conv_tmp_9a554_49)))
                    + ((M31_2) * (conv_tmp_9a554_55)));
                let k_mod_2_18_biased_tmp_9a554_84 =
                    ((((PackedUInt32::from_m31(((conv_mod_tmp_9a554_56) + (M31_134217728))))
                        + (((PackedUInt32::from_m31(
                            ((conv_mod_tmp_9a554_57) + (M31_134217728)),
                        )) & (UInt32_511))
                            << (UInt32_9)))
                        + (UInt32_65536))
                        & (UInt32_262143));
                let k_col84 = ((k_mod_2_18_biased_tmp_9a554_84.low().as_m31())
                    + (((k_mod_2_18_biased_tmp_9a554_84.high().as_m31()) - (M31_1)) * (M31_65536)));
                *row[84] = k_col84;
                let range_check_19_inputs_0 = [((k_col84) + (M31_262144))].unpack();
                *lookup_data.range_check_19_0 = [((k_col84) + (M31_262144))];
                let carry_0_col85 =
                    ((((conv_mod_tmp_9a554_56) - ((M31_1) * (k_col84))) + (M31_0)) * (M31_4194304));
                *row[85] = carry_0_col85;
                let range_check_19_inputs_1 = [((carry_0_col85) + (M31_131072))].unpack();
                *lookup_data.range_check_19_1 = [((carry_0_col85) + (M31_131072))];
                let carry_1_col86 = (((conv_mod_tmp_9a554_57) + (carry_0_col85)) * (M31_4194304));
                *row[86] = carry_1_col86;
                let range_check_19_inputs_2 = [((carry_1_col86) + (M31_131072))].unpack();
                *lookup_data.range_check_19_2 = [((carry_1_col86) + (M31_131072))];
                let carry_2_col87 = (((conv_mod_tmp_9a554_58) + (carry_1_col86)) * (M31_4194304));
                *row[87] = carry_2_col87;
                let range_check_19_inputs_3 = [((carry_2_col87) + (M31_131072))].unpack();
                *lookup_data.range_check_19_3 = [((carry_2_col87) + (M31_131072))];
                let carry_3_col88 = (((conv_mod_tmp_9a554_59) + (carry_2_col87)) * (M31_4194304));
                *row[88] = carry_3_col88;
                let range_check_19_inputs_4 = [((carry_3_col88) + (M31_131072))].unpack();
                *lookup_data.range_check_19_4 = [((carry_3_col88) + (M31_131072))];
                let carry_4_col89 = (((conv_mod_tmp_9a554_60) + (carry_3_col88)) * (M31_4194304));
                *row[89] = carry_4_col89;
                let range_check_19_inputs_5 = [((carry_4_col89) + (M31_131072))].unpack();
                *lookup_data.range_check_19_5 = [((carry_4_col89) + (M31_131072))];
                let carry_5_col90 = (((conv_mod_tmp_9a554_61) + (carry_4_col89)) * (M31_4194304));
                *row[90] = carry_5_col90;
                let range_check_19_inputs_6 = [((carry_5_col90) + (M31_131072))].unpack();
                *lookup_data.range_check_19_6 = [((carry_5_col90) + (M31_131072))];
                let carry_6_col91 = (((conv_mod_tmp_9a554_62) + (carry_5_col90)) * (M31_4194304));
                *row[91] = carry_6_col91;
                let range_check_19_inputs_7 = [((carry_6_col91) + (M31_131072))].unpack();
                *lookup_data.range_check_19_7 = [((carry_6_col91) + (M31_131072))];
                let carry_7_col92 = (((conv_mod_tmp_9a554_63) + (carry_6_col91)) * (M31_4194304));
                *row[92] = carry_7_col92;
                let range_check_19_inputs_8 = [((carry_7_col92) + (M31_131072))].unpack();
                *lookup_data.range_check_19_8 = [((carry_7_col92) + (M31_131072))];
                let carry_8_col93 = (((conv_mod_tmp_9a554_64) + (carry_7_col92)) * (M31_4194304));
                *row[93] = carry_8_col93;
                let range_check_19_inputs_9 = [((carry_8_col93) + (M31_131072))].unpack();
                *lookup_data.range_check_19_9 = [((carry_8_col93) + (M31_131072))];
                let carry_9_col94 = (((conv_mod_tmp_9a554_65) + (carry_8_col93)) * (M31_4194304));
                *row[94] = carry_9_col94;
                let range_check_19_inputs_10 = [((carry_9_col94) + (M31_131072))].unpack();
                *lookup_data.range_check_19_10 = [((carry_9_col94) + (M31_131072))];
                let carry_10_col95 = (((conv_mod_tmp_9a554_66) + (carry_9_col94)) * (M31_4194304));
                *row[95] = carry_10_col95;
                let range_check_19_inputs_11 = [((carry_10_col95) + (M31_131072))].unpack();
                *lookup_data.range_check_19_11 = [((carry_10_col95) + (M31_131072))];
                let carry_11_col96 = (((conv_mod_tmp_9a554_67) + (carry_10_col95)) * (M31_4194304));
                *row[96] = carry_11_col96;
                let range_check_19_inputs_12 = [((carry_11_col96) + (M31_131072))].unpack();
                *lookup_data.range_check_19_12 = [((carry_11_col96) + (M31_131072))];
                let carry_12_col97 = (((conv_mod_tmp_9a554_68) + (carry_11_col96)) * (M31_4194304));
                *row[97] = carry_12_col97;
                let range_check_19_inputs_13 = [((carry_12_col97) + (M31_131072))].unpack();
                *lookup_data.range_check_19_13 = [((carry_12_col97) + (M31_131072))];
                let carry_13_col98 = (((conv_mod_tmp_9a554_69) + (carry_12_col97)) * (M31_4194304));
                *row[98] = carry_13_col98;
                let range_check_19_inputs_14 = [((carry_13_col98) + (M31_131072))].unpack();
                *lookup_data.range_check_19_14 = [((carry_13_col98) + (M31_131072))];
                let carry_14_col99 = (((conv_mod_tmp_9a554_70) + (carry_13_col98)) * (M31_4194304));
                *row[99] = carry_14_col99;
                let range_check_19_inputs_15 = [((carry_14_col99) + (M31_131072))].unpack();
                *lookup_data.range_check_19_15 = [((carry_14_col99) + (M31_131072))];
                let carry_15_col100 =
                    (((conv_mod_tmp_9a554_71) + (carry_14_col99)) * (M31_4194304));
                *row[100] = carry_15_col100;
                let range_check_19_inputs_16 = [((carry_15_col100) + (M31_131072))].unpack();
                *lookup_data.range_check_19_16 = [((carry_15_col100) + (M31_131072))];
                let carry_16_col101 =
                    (((conv_mod_tmp_9a554_72) + (carry_15_col100)) * (M31_4194304));
                *row[101] = carry_16_col101;
                let range_check_19_inputs_17 = [((carry_16_col101) + (M31_131072))].unpack();
                *lookup_data.range_check_19_17 = [((carry_16_col101) + (M31_131072))];
                let carry_17_col102 =
                    (((conv_mod_tmp_9a554_73) + (carry_16_col101)) * (M31_4194304));
                *row[102] = carry_17_col102;
                let range_check_19_inputs_18 = [((carry_17_col102) + (M31_131072))].unpack();
                *lookup_data.range_check_19_18 = [((carry_17_col102) + (M31_131072))];
                let carry_18_col103 =
                    (((conv_mod_tmp_9a554_74) + (carry_17_col102)) * (M31_4194304));
                *row[103] = carry_18_col103;
                let range_check_19_inputs_19 = [((carry_18_col103) + (M31_131072))].unpack();
                *lookup_data.range_check_19_19 = [((carry_18_col103) + (M31_131072))];
                let carry_19_col104 =
                    (((conv_mod_tmp_9a554_75) + (carry_18_col103)) * (M31_4194304));
                *row[104] = carry_19_col104;
                let range_check_19_inputs_20 = [((carry_19_col104) + (M31_131072))].unpack();
                *lookup_data.range_check_19_20 = [((carry_19_col104) + (M31_131072))];
                let carry_20_col105 =
                    (((conv_mod_tmp_9a554_76) + (carry_19_col104)) * (M31_4194304));
                *row[105] = carry_20_col105;
                let range_check_19_inputs_21 = [((carry_20_col105) + (M31_131072))].unpack();
                *lookup_data.range_check_19_21 = [((carry_20_col105) + (M31_131072))];
                let carry_21_col106 = ((((conv_mod_tmp_9a554_77) - ((M31_136) * (k_col84)))
                    + (carry_20_col105))
                    * (M31_4194304));
                *row[106] = carry_21_col106;
                let range_check_19_inputs_22 = [((carry_21_col106) + (M31_131072))].unpack();
                *lookup_data.range_check_19_22 = [((carry_21_col106) + (M31_131072))];
                let carry_22_col107 =
                    (((conv_mod_tmp_9a554_78) + (carry_21_col106)) * (M31_4194304));
                *row[107] = carry_22_col107;
                let range_check_19_inputs_23 = [((carry_22_col107) + (M31_131072))].unpack();
                *lookup_data.range_check_19_23 = [((carry_22_col107) + (M31_131072))];
                let carry_23_col108 =
                    (((conv_mod_tmp_9a554_79) + (carry_22_col107)) * (M31_4194304));
                *row[108] = carry_23_col108;
                let range_check_19_inputs_24 = [((carry_23_col108) + (M31_131072))].unpack();
                *lookup_data.range_check_19_24 = [((carry_23_col108) + (M31_131072))];
                let carry_24_col109 =
                    (((conv_mod_tmp_9a554_80) + (carry_23_col108)) * (M31_4194304));
                *row[109] = carry_24_col109;
                let range_check_19_inputs_25 = [((carry_24_col109) + (M31_131072))].unpack();
                *lookup_data.range_check_19_25 = [((carry_24_col109) + (M31_131072))];
                let carry_25_col110 =
                    (((conv_mod_tmp_9a554_81) + (carry_24_col109)) * (M31_4194304));
                *row[110] = carry_25_col110;
                let range_check_19_inputs_26 = [((carry_25_col110) + (M31_131072))].unpack();
                *lookup_data.range_check_19_26 = [((carry_25_col110) + (M31_131072))];
                let carry_26_col111 =
                    (((conv_mod_tmp_9a554_82) + (carry_25_col110)) * (M31_4194304));
                *row[111] = carry_26_col111;
                let range_check_19_inputs_27 = [((carry_26_col111) + (M31_131072))].unpack();
                *lookup_data.range_check_19_27 = [((carry_26_col111) + (M31_131072))];
                *lookup_data.verify_mul_252_0 = [
                    input_limb_0_col0,
                    input_limb_1_col1,
                    input_limb_2_col2,
                    input_limb_3_col3,
                    input_limb_4_col4,
                    input_limb_5_col5,
                    input_limb_6_col6,
                    input_limb_7_col7,
                    input_limb_8_col8,
                    input_limb_9_col9,
                    input_limb_10_col10,
                    input_limb_11_col11,
                    input_limb_12_col12,
                    input_limb_13_col13,
                    input_limb_14_col14,
                    input_limb_15_col15,
                    input_limb_16_col16,
                    input_limb_17_col17,
                    input_limb_18_col18,
                    input_limb_19_col19,
                    input_limb_20_col20,
                    input_limb_21_col21,
                    input_limb_22_col22,
                    input_limb_23_col23,
                    input_limb_24_col24,
                    input_limb_25_col25,
                    input_limb_26_col26,
                    input_limb_27_col27,
                    input_limb_28_col28,
                    input_limb_29_col29,
                    input_limb_30_col30,
                    input_limb_31_col31,
                    input_limb_32_col32,
                    input_limb_33_col33,
                    input_limb_34_col34,
                    input_limb_35_col35,
                    input_limb_36_col36,
                    input_limb_37_col37,
                    input_limb_38_col38,
                    input_limb_39_col39,
                    input_limb_40_col40,
                    input_limb_41_col41,
                    input_limb_42_col42,
                    input_limb_43_col43,
                    input_limb_44_col44,
                    input_limb_45_col45,
                    input_limb_46_col46,
                    input_limb_47_col47,
                    input_limb_48_col48,
                    input_limb_49_col49,
                    input_limb_50_col50,
                    input_limb_51_col51,
                    input_limb_52_col52,
                    input_limb_53_col53,
                    input_limb_54_col54,
                    input_limb_55_col55,
                    input_limb_56_col56,
                    input_limb_57_col57,
                    input_limb_58_col58,
                    input_limb_59_col59,
                    input_limb_60_col60,
                    input_limb_61_col61,
                    input_limb_62_col62,
                    input_limb_63_col63,
                    input_limb_64_col64,
                    input_limb_65_col65,
                    input_limb_66_col66,
                    input_limb_67_col67,
                    input_limb_68_col68,
                    input_limb_69_col69,
                    input_limb_70_col70,
                    input_limb_71_col71,
                    input_limb_72_col72,
                    input_limb_73_col73,
                    input_limb_74_col74,
                    input_limb_75_col75,
                    input_limb_76_col76,
                    input_limb_77_col77,
                    input_limb_78_col78,
                    input_limb_79_col79,
                    input_limb_80_col80,
                    input_limb_81_col81,
                    input_limb_82_col82,
                    input_limb_83_col83,
                ];

                *row[112] = padding_col.packed_at(row_index);

                // Add sub-components inputs.
                range_check_19_state.add_inputs(&range_check_19_inputs_0);
                range_check_19_state.add_inputs(&range_check_19_inputs_1);
                range_check_19_state.add_inputs(&range_check_19_inputs_2);
                range_check_19_state.add_inputs(&range_check_19_inputs_3);
                range_check_19_state.add_inputs(&range_check_19_inputs_4);
                range_check_19_state.add_inputs(&range_check_19_inputs_5);
                range_check_19_state.add_inputs(&range_check_19_inputs_6);
                range_check_19_state.add_inputs(&range_check_19_inputs_7);
                range_check_19_state.add_inputs(&range_check_19_inputs_8);
                range_check_19_state.add_inputs(&range_check_19_inputs_9);
                range_check_19_state.add_inputs(&range_check_19_inputs_10);
                range_check_19_state.add_inputs(&range_check_19_inputs_11);
                range_check_19_state.add_inputs(&range_check_19_inputs_12);
                range_check_19_state.add_inputs(&range_check_19_inputs_13);
                range_check_19_state.add_inputs(&range_check_19_inputs_14);
                range_check_19_state.add_inputs(&range_check_19_inputs_15);
                range_check_19_state.add_inputs(&range_check_19_inputs_16);
                range_check_19_state.add_inputs(&range_check_19_inputs_17);
                range_check_19_state.add_inputs(&range_check_19_inputs_18);
                range_check_19_state.add_inputs(&range_check_19_inputs_19);
                range_check_19_state.add_inputs(&range_check_19_inputs_20);
                range_check_19_state.add_inputs(&range_check_19_inputs_21);
                range_check_19_state.add_inputs(&range_check_19_inputs_22);
                range_check_19_state.add_inputs(&range_check_19_inputs_23);
                range_check_19_state.add_inputs(&range_check_19_inputs_24);
                range_check_19_state.add_inputs(&range_check_19_inputs_25);
                range_check_19_state.add_inputs(&range_check_19_inputs_26);
                range_check_19_state.add_inputs(&range_check_19_inputs_27);
            },
        );

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    range_check_19_0: Vec<[PackedM31; 1]>,
    range_check_19_1: Vec<[PackedM31; 1]>,
    range_check_19_2: Vec<[PackedM31; 1]>,
    range_check_19_3: Vec<[PackedM31; 1]>,
    range_check_19_4: Vec<[PackedM31; 1]>,
    range_check_19_5: Vec<[PackedM31; 1]>,
    range_check_19_6: Vec<[PackedM31; 1]>,
    range_check_19_7: Vec<[PackedM31; 1]>,
    range_check_19_8: Vec<[PackedM31; 1]>,
    range_check_19_9: Vec<[PackedM31; 1]>,
    range_check_19_10: Vec<[PackedM31; 1]>,
    range_check_19_11: Vec<[PackedM31; 1]>,
    range_check_19_12: Vec<[PackedM31; 1]>,
    range_check_19_13: Vec<[PackedM31; 1]>,
    range_check_19_14: Vec<[PackedM31; 1]>,
    range_check_19_15: Vec<[PackedM31; 1]>,
    range_check_19_16: Vec<[PackedM31; 1]>,
    range_check_19_17: Vec<[PackedM31; 1]>,
    range_check_19_18: Vec<[PackedM31; 1]>,
    range_check_19_19: Vec<[PackedM31; 1]>,
    range_check_19_20: Vec<[PackedM31; 1]>,
    range_check_19_21: Vec<[PackedM31; 1]>,
    range_check_19_22: Vec<[PackedM31; 1]>,
    range_check_19_23: Vec<[PackedM31; 1]>,
    range_check_19_24: Vec<[PackedM31; 1]>,
    range_check_19_25: Vec<[PackedM31; 1]>,
    range_check_19_26: Vec<[PackedM31; 1]>,
    range_check_19_27: Vec<[PackedM31; 1]>,
    verify_mul_252_0: Vec<[PackedM31; 84]>,
}

pub struct InteractionClaimGenerator {
    n_rows: usize,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        range_check_19: &relations::RangeCheck_19,
        verify_mul_252: &relations::VerifyMul252,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let log_size = std::cmp::max(self.n_rows.next_power_of_two().ilog2(), LOG_N_LANES);
        let padding_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_0,
            &self.lookup_data.range_check_19_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_2,
            &self.lookup_data.range_check_19_3,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_4,
            &self.lookup_data.range_check_19_5,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_6,
            &self.lookup_data.range_check_19_7,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_8,
            &self.lookup_data.range_check_19_9,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_10,
            &self.lookup_data.range_check_19_11,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_12,
            &self.lookup_data.range_check_19_13,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_14,
            &self.lookup_data.range_check_19_15,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_16,
            &self.lookup_data.range_check_19_17,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_18,
            &self.lookup_data.range_check_19_19,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_20,
            &self.lookup_data.range_check_19_21,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_22,
            &self.lookup_data.range_check_19_23,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_24,
            &self.lookup_data.range_check_19_25,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_26,
            &self.lookup_data.range_check_19_27,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, values) in self.lookup_data.verify_mul_252_0.iter().enumerate() {
            let denom = verify_mul_252.combine(values);
            col_gen.write_frac(i, -PackedQM31::one() * padding_col.packed_at(i), denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
