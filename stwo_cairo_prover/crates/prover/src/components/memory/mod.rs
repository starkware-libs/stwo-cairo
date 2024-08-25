use stwo_prover::constraint_framework::logup::LookupElements;
use stwo_prover::core::channel::Channel;

pub mod component;
pub mod prover;

pub const N_MEM_BIG_LIMBS: usize = 14;
pub const N_MEM_BIG_LIMB_BITS: usize = 18;

#[derive(Clone)]
pub struct MemoryElements {
    pub addr_to_id: LookupElements<2>,
    pub id_to_big: LookupElements<{ 1 + N_MEM_BIG_LIMBS }>,
}
impl MemoryElements {
    pub fn draw(channel: &mut impl Channel) -> Self {
        Self {
            addr_to_id: LookupElements::draw(channel),
            id_to_big: LookupElements::draw(channel),
        }
    }
}
pub const N_INSTR_LIMBS: usize = 4;
pub type InstructionElements = LookupElements<{ 1 + N_INSTR_LIMBS }>;
