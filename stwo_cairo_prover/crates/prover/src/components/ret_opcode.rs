use std::simd::Simd;

use num_traits::{One, Zero};
use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::EvalAtRow;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::lookups::utils::Fraction;

use super::opcode::PackedVmState;
use super::{
    LookupFunc, Standard, StandardClaim, StandardComponent, StandardLookupData, StandardProver,
};
use crate::components::opcode::{OpcodeElements, OpcodeGenContext};

pub type RetProver = StandardProver<RetOpcode>;
pub type RetClaim = StandardClaim<RetOpcode>;
pub type RetComponent = StandardComponent<RetOpcode>;

const RET_FLAGS: u32 = 0b010000010001011;

#[derive(Clone)]
pub struct RetOpcode;

impl Standard for RetOpcode {
    type LookupElements = OpcodeElements;
    type Context<'a> = OpcodeGenContext<'a>;
    type PackedInput = PackedVmState;
    type LookupData = RetLookupData;
    const N_REPETITIONS: usize = 4;

    fn n_columns() -> usize {
        5
    }
    fn new_lookup_data(log_size: u32) -> Self::LookupData {
        RetLookupData {
            log_size,
            pc: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
            fp: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
            new_pc: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
            new_fp: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
        }
    }
    fn evaluate<E: EvalAtRow>(
        eval: &mut E,
        logup: &mut LogupAtRow<2, E>,
        elements: &OpcodeElements,
    ) {
        let pc = eval.next_trace_mask();
        let _ap = eval.next_trace_mask();
        let fp = eval.next_trace_mask();

        // Instruction lookup.
        logup.push_lookup(
            eval,
            E::EF::one(),
            &[
                pc,
                M31::from(-2).into(),
                M31::from(-1).into(),
                M31::from(-1).into(),
                M31::from(RET_FLAGS).into(),
            ],
            &elements.instruction_elements,
        );

        // fp-1 lookup.
        let new_fp = eval.next_trace_mask();
        logup.push_lookup(
            eval,
            E::EF::one(),
            &[fp - M31::from(1).into(), new_fp],
            &elements.memory_elements.addr_to_id,
        );

        // fp-2 lookup
        let new_pc = eval.next_trace_mask();
        logup.push_lookup(
            eval,
            E::EF::one(),
            &[fp - M31::from(2).into(), new_pc],
            &elements.memory_elements.addr_to_id,
        );

        // TODO: State lookups.
    }
    fn write_trace_row(
        dst: &mut [BaseColumn],
        state_input: &PackedVmState,
        row_index: usize,
        ctx: &mut OpcodeGenContext<'_>,
        lookup_data: &mut Self::LookupData,
    ) {
        let pc = unsafe { PackedM31::from_simd_unchecked(state_input.pc) };
        let ap = unsafe { PackedM31::from_simd_unchecked(state_input.ap) };
        let fp = unsafe { PackedM31::from_simd_unchecked(state_input.fp) };
        dst[0].data[row_index] = pc;
        dst[1].data[row_index] = ap;
        dst[2].data[row_index] = fp;
        lookup_data.pc.push(pc);
        lookup_data.fp.push(fp);

        let new_fp = ctx
            .addr_to_id
            .add_inputs_simd(state_input.fp - Simd::splat(1));
        let new_fp = unsafe { PackedM31::from_simd_unchecked(new_fp) };
        dst[3].data[row_index] = new_fp;
        lookup_data.new_fp.push(new_fp);

        let new_pc = ctx
            .addr_to_id
            .add_inputs_simd(state_input.fp - Simd::splat(2));
        let new_pc = unsafe { PackedM31::from_simd_unchecked(new_pc) };
        dst[4].data[row_index] = new_pc;
        lookup_data.new_pc.push(new_pc);
    }
}

pub struct RetLookupData {
    log_size: u32,
    pc: Vec<PackedM31>,
    fp: Vec<PackedM31>,
    new_pc: Vec<PackedM31>,
    new_fp: Vec<PackedM31>,
}

impl StandardLookupData for RetLookupData {
    const N_LOOKUPS: usize = 3;

    type Elements = OpcodeElements;

    fn lookups<'a>(&'a self, elements: &'a Self::Elements) -> Vec<LookupFunc<'a>> {
        vec![
            // Instruction lookup.
            Box::new((0..(1 << self.log_size)).map(|i| {
                let denom = elements.instruction_elements.combine(&[
                    self.pc[i],
                    M31::from(-2).into(),
                    M31::from(-1).into(),
                    M31::from(-1).into(),
                    M31::from(RET_FLAGS).into(),
                ]);
                Fraction::new(PackedM31::one(), denom)
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            // fp-1 lookup.
            Box::new((0..(1 << self.log_size)).map(|i| {
                let denom = elements.memory_elements.addr_to_id.combine(&[
                    self.fp[i] - PackedM31::broadcast(M31::from(1)),
                    self.new_fp[i],
                ]);
                Fraction::new(PackedM31::one(), denom)
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            // fp-2 lookup.
            Box::new((0..(1 << self.log_size)).map(|i| {
                let denom = elements.memory_elements.addr_to_id.combine(&[
                    self.fp[i] - PackedM31::broadcast(M31::from(2)),
                    self.new_pc[i],
                ]);
                Fraction::new(PackedM31::one(), denom)
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
        ]
    }
}
