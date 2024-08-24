use num_traits::One;
use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::EvalAtRow;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::PackedM31;
use stwo_prover::core::lookups::utils::Fraction;

use crate::components::opcode::prover::PackedVmState;
use crate::components::opcode::{FVmState, LookupFunc, Opcode, OpcodeElements, OpcodeGenContext};

struct RetOpcode;
const RET_FLAGS: u32 = 0b010000010001011;
impl Opcode for RetOpcode {
    type LookupData = RetLookupData;

    fn n_columns() -> usize {
        5
    }
    fn new_lookup_data(log_size: u32) -> Self::LookupData {
        todo!()
    }
    fn lookups<'a, 'b>(
        lookup_data: &'a Self::LookupData,
        elements: &'a OpcodeElements,
    ) -> Vec<LookupFunc<'a, 'b>> {
        vec![
            // Instruction lookup.
            Box::new(move |i, dst| {
                for (j, dst) in dst.iter_mut().enumerate() {
                    let denom = elements.instruction_elements.combine(&[
                        lookup_data.pc[i + j],
                        (-2).into(),
                        (-1).into(),
                        (-1).into(),
                        RET_FLAGS.into(),
                    ]);
                    *dst = Fraction::new(PackedM31::one(), denom);
                }
            }),
            // fp-1 lookup.
            Box::new(move |i, dst| {
                for (j, dst) in dst.iter_mut().enumerate() {
                    let denom = elements
                        .memory_elements
                        .combine(&[lookup_data.fp[i + j] - 1, lookup_data.new_fp[i + j]]);
                    *dst = Fraction::new(PackedM31::one(), denom);
                }
            }),
            // fp-2 lookup.
            Box::new(move |i, dst| {
                for (j, dst) in dst.iter_mut().enumerate() {
                    let denom = elements
                        .memory_elements
                        .combine(&[lookup_data.fp[i + j] - 2, lookup_data.new_pc[i + j]]);
                    *dst = Fraction::new(PackedM31::one(), denom);
                }
            }),
        ]
    }
    fn evaluate<E: EvalAtRow>(
        eval: &mut E,
        logup: &mut LogupAtRow<2, E>,
        state: FVmState<E::F>,
        elements: &OpcodeElements,
    ) -> FVmState<E::F> {
        let pc = eval.next_trace_mask();
        let ap = eval.next_trace_mask();
        let fp = eval.next_trace_mask();

        // Instruction lookup.
        logup.push_lookup(
            &mut eval,
            E::EF::one(),
            &[pc, (-2).into(), (-1).into(), (-1).into(), RET_FLAGS.into()],
            &elements.instruction_elements,
        );

        // fp-1 lookup.
        let new_fp = eval.next_trace_mask();
        logup.push_lookup(
            &mut eval,
            E::EF::one(),
            &[fp - 1, new_fp],
            &elements.memory_elements,
        );

        // fp-2 lookup
        let new_pc = eval.next_trace_mask();
        logup.push_lookup(
            &mut eval,
            E::EF::one(),
            &[fp - 2, new_pc],
            &elements.memory_elements,
        );

        FVmState {
            pc: new_pc,
            ap,
            fp: new_fp,
        }
    }
    fn write_trace_row(
        dst: &mut [BaseColumn],
        state_input: &PackedVmState,
        row_index: usize,
        ctx: OpcodeGenContext<'_>,
        lookup_data: &mut Self::LookupData,
    ) {
        dst[0].data[row_index] = state_input.pc;
        dst[1].data[row_index] = state_input.ap;
        dst[2].data[row_index] = state_input.fp;
        lookup_data.pc.push(state_input.pc);
        lookup_data.fp.push(state_input.fp);

        let new_fp = ctx.mem.deduce_output(state_input.fp - 1);
        dst[3].data[row_index] = new_fp;
        lookup_data.new_fp.push(new_fp);

        let new_pc = ctx.mem.deduce_output(state_input.fp - 2);
        dst[4].data[row_index] = new_pc;
        lookup_data.new_pc.push(new_pc);
    }
}

struct RetLookupData {
    pc: Vec<PackedM31>,
    fp: Vec<PackedM31>,
    new_pc: Vec<PackedM31>,
    new_fp: Vec<PackedM31>,
}
