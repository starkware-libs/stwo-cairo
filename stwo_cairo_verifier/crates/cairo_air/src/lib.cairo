pub mod presets {
    #[cfg(not(feature: "minimal_verifier"))]
    pub mod all_cairo;
    #[cfg(feature: "minimal_verifier")]
    pub mod minimal;
}
pub mod components;
pub mod utils;
use core::dict::Felt252Dict;
#[cfg(not(feature: "minimal_verifier"))]
pub use presets::all_cairo::{CairoProof, VerificationOutput, get_verification_output, verify_cairo};
#[cfg(feature: "minimal_verifier")]
pub use presets::minimal::{CairoProof, VerificationOutput, get_verification_output, verify_cairo};
use stwo_constraint_framework::LookupElements;

type Cube252Elements = LookupElements<20>;

type MemoryAddressToIdElements = LookupElements<2>;

type MemoryIdToBigElements = LookupElements<29>;

type OpcodesElements = LookupElements<3>;

type PartialEcMulElements = LookupElements<73>;

type PedersenPointsTableElements = LookupElements<57>;

type PoseidonFullRoundChainElements = LookupElements<32>;

type Poseidon3PartialRoundsChainElements = LookupElements<42>;

type PoseidonRoundKeysElements = LookupElements<31>;

type BlakeGElements = LookupElements<20>;

type BlakeRoundElements = LookupElements<35>;

type BlakeRoundSigmaElements = LookupElements<17>;

type TripleXor32Elements = LookupElements<8>;

type RangeCheck_6Elements = LookupElements<1>;

type RangeCheck_8Elements = LookupElements<1>;

type RangeCheck_11Elements = LookupElements<1>;

type RangeCheck_12Elements = LookupElements<1>;

type RangeCheck_18Elements = LookupElements<1>;
type RangeCheck_18_BElements = LookupElements<1>;

type RangeCheck_19Elements = LookupElements<1>;
type RangeCheck_19_BElements = LookupElements<1>;
type RangeCheck_19_CElements = LookupElements<1>;
type RangeCheck_19_DElements = LookupElements<1>;
type RangeCheck_19_EElements = LookupElements<1>;
type RangeCheck_19_FElements = LookupElements<1>;
type RangeCheck_19_GElements = LookupElements<1>;
type RangeCheck_19_HElements = LookupElements<1>;

type RangeCheck_9_9Elements = LookupElements<2>;
type RangeCheck_9_9_BElements = LookupElements<2>;
type RangeCheck_9_9_CElements = LookupElements<2>;
type RangeCheck_9_9_DElements = LookupElements<2>;
type RangeCheck_9_9_EElements = LookupElements<2>;
type RangeCheck_9_9_FElements = LookupElements<2>;
type RangeCheck_9_9_GElements = LookupElements<2>;
type RangeCheck_9_9_HElements = LookupElements<2>;

type RangeCheck_4_3Elements = LookupElements<2>;

type RangeCheck_4_4Elements = LookupElements<2>;

type RangeCheck_5_4Elements = LookupElements<2>;

type RangeCheck_7_2_5Elements = LookupElements<3>;

type RangeCheck_3_6_6_3Elements = LookupElements<4>;

type RangeCheck_4_4_4_4Elements = LookupElements<4>;

type RangeCheck_3_3_3_3_3Elements = LookupElements<5>;

type RangeCheckFelt252Width27Elements = LookupElements<10>;

type VerifyInstructionElements = LookupElements<7>;

type VerifyBitwiseXor_4Elements = LookupElements<3>;

type VerifyBitwiseXor_7Elements = LookupElements<3>;

type VerifyBitwiseXor_8Elements = LookupElements<3>;

type VerifyBitwiseXor_9Elements = LookupElements<3>;

type VerifyBitwiseXor_12Elements = LookupElements<3>;

// A dict from relation_id, which is a string encoded as a felt252, to the number of uses of the
// corresponding relation.
type RelationUsesDict = Felt252Dict<u64>;

// A tuple of (relation_id, uses).
type RelationUse = (felt252, u32);
