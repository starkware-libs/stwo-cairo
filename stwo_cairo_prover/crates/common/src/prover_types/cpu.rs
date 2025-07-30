use std::array::from_fn;
use std::fmt::Debug;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub};

use ruint::Uint;
use serde::{Deserialize, Serialize};
use starknet_ff::FieldElement;
use starknet_types_core::felt::Felt as StarknetTypesFelt;
use stwo::core::channel::Channel;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};

pub type M31 = stwo::core::fields::m31::M31;
pub type QM31 = stwo::core::fields::qm31::QM31;

pub const PRIME: u32 = 2_u32.pow(31) - 1;

pub trait AlgebraicType: ProverType + Add + Sub + Mul + Div {}
impl AlgebraicType for M31 {}
impl AlgebraicType for Felt252 {}
impl<const B: usize, const L: usize, const F: usize> AlgebraicType for BigUInt<B, L, F> {}

pub trait NumericType: ProverType + Rem + Shl + Shr + BitAnd + BitOr + BitXor {}
impl NumericType for UInt16 {}
impl NumericType for UInt32 {}
impl NumericType for UInt64 {}

pub trait SingleFeltType: ProverType {
    fn as_m31(&self) -> M31;
}

/// Expression Types - the basic type of the variables composing the expression.
/// For example, felt or bool. The expression types are divided into group, depending on
/// the operations that can be performed on them.
pub trait ProverType: Debug + Clone + Copy + Default {
    // Returns the value of the prover type as a string.
    fn calc(&self) -> String;
    // Returns the type of the prover type as a string.
    fn r#type() -> String;
}

impl ProverType for M31 {
    fn calc(&self) -> String {
        self.to_string()
    }
    fn r#type() -> String {
        "M31".to_string()
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    Default,
    Eq,
    PartialEq,
    Hash,
    CairoSerialize,
    CairoDeserialize,
)]
pub struct CasmState {
    pub pc: M31,
    pub ap: M31,
    pub fp: M31,
}

impl CasmState {
    pub fn values(&self) -> [M31; 3] {
        [self.pc, self.ap, self.fp]
    }
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.pc.0 as u64);
        channel.mix_u64(self.ap.0 as u64);
        channel.mix_u64(self.fp.0 as u64);
    }
}

impl ProverType for CasmState {
    fn calc(&self) -> String {
        format!("{{ pc: {}, ap: {}, fp: {} }}", self.pc, self.ap, self.fp)
    }
    fn r#type() -> String {
        "CasmState".to_string()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq, Hash)]
pub struct Bool {
    pub value: bool,
}

impl Bool {
    pub fn from_m31(felt: M31) -> Self {
        assert!(felt.0 == 0 || felt.0 == 1, "M31 value is not a bool");
        Self { value: felt.0 != 0 }
    }
}

impl ProverType for Bool {
    fn calc(&self) -> String {
        self.value.to_string()
    }
    fn r#type() -> String {
        "Bool".to_string()
    }
}

impl SingleFeltType for Bool {
    fn as_m31(&self) -> M31 {
        M31::from_u32_unchecked(if self.value { 1 } else { 0 })
    }
}

impl From<bool> for Bool {
    fn from(value: bool) -> Bool {
        Bool { value }
    }
}

impl Not for Bool {
    type Output = Bool;
    fn not(self) -> Bool {
        Bool { value: !self.value }
    }
}

impl BitAnd for Bool {
    type Output = Bool;
    fn bitand(self, other: Self) -> Self::Output {
        Bool {
            value: self.value & other.value,
        }
    }
}

impl BitOr for Bool {
    type Output = Bool;
    fn bitor(self, other: Self) -> Self::Output {
        Bool {
            value: self.value | other.value,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq, Hash)]
pub struct UInt16 {
    pub value: u16,
}

impl UInt16 {
    pub fn from_bool(val: Bool) -> Self {
        Self {
            value: val.value as u16,
        }
    }

    pub fn from_m31(felt: M31) -> Self {
        assert!(
            felt < M31::from_u32_unchecked(2_u32.pow(16)),
            "M31 value is not a u16"
        );
        Self {
            value: felt.0 as u16,
        }
    }
}

impl ProverType for UInt16 {
    fn calc(&self) -> String {
        self.value.to_string()
    }
    fn r#type() -> String {
        "UInt16".to_string()
    }
}

impl Add for UInt16 {
    type Output = UInt16;
    fn add(self, other: UInt16) -> UInt16 {
        UInt16 {
            value: self.value.wrapping_add(other.value),
        }
    }
}

impl Sub for UInt16 {
    type Output = UInt16;
    fn sub(self, rhs: UInt16) -> UInt16 {
        UInt16 {
            value: self.value.wrapping_sub(rhs.value),
        }
    }
}

impl SingleFeltType for UInt16 {
    fn as_m31(&self) -> M31 {
        M31::from_u32_unchecked(self.value as u32)
    }
}

impl From<u16> for UInt16 {
    fn from(value: u16) -> UInt16 {
        UInt16 { value }
    }
}

impl Rem for UInt16 {
    type Output = UInt16;
    fn rem(self, other: UInt16) -> UInt16 {
        UInt16 {
            value: self.value % other.value,
        }
    }
}
impl Shl for UInt16 {
    type Output = UInt16;
    fn shl(self, other: UInt16) -> UInt16 {
        UInt16 {
            value: self.value << other.value,
        }
    }
}
impl Shr for UInt16 {
    type Output = UInt16;
    fn shr(self, other: UInt16) -> UInt16 {
        UInt16 {
            value: self.value >> other.value,
        }
    }
}
impl BitAnd for UInt16 {
    type Output = UInt16;
    fn bitand(self, other: UInt16) -> UInt16 {
        UInt16 {
            value: self.value & other.value,
        }
    }
}
impl BitOr for UInt16 {
    type Output = UInt16;
    fn bitor(self, other: UInt16) -> UInt16 {
        UInt16 {
            value: self.value | other.value,
        }
    }
}
impl BitXor for UInt16 {
    type Output = UInt16;
    fn bitxor(self, other: UInt16) -> UInt16 {
        UInt16 {
            value: self.value ^ other.value,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq, Hash)]
pub struct UInt32 {
    pub value: u32,
}

impl UInt32 {
    pub fn low(&self) -> UInt16 {
        UInt16 {
            value: (self.value & 0xFFFF) as u16,
        }
    }

    pub fn high(&self) -> UInt16 {
        UInt16 {
            value: (self.value >> 16) as u16,
        }
    }

    pub fn from_m31(felt: M31) -> Self {
        Self { value: felt.0 }
    }

    pub fn from_limbs(low: M31, high: M31) -> Self {
        Self {
            value: (low.0 & 0xFFFF) | ((high.0 & 0xFFFF) << 16),
        }
    }
}

impl From<u32> for UInt32 {
    fn from(value: u32) -> UInt32 {
        UInt32 { value }
    }
}

impl ProverType for UInt32 {
    fn calc(&self) -> String {
        self.value.to_string()
    }
    fn r#type() -> String {
        "UInt32".to_string()
    }
}

impl Add for UInt32 {
    type Output = UInt32;
    fn add(self, other: UInt32) -> UInt32 {
        UInt32 {
            value: self.value.wrapping_add(other.value),
        }
    }
}
impl Rem for UInt32 {
    type Output = UInt32;
    fn rem(self, other: UInt32) -> UInt32 {
        UInt32 {
            value: self.value % other.value,
        }
    }
}
impl Shl for UInt32 {
    type Output = UInt32;
    fn shl(self, other: UInt32) -> UInt32 {
        UInt32 {
            value: self.value << other.value,
        }
    }
}
impl Shr for UInt32 {
    type Output = UInt32;
    fn shr(self, other: UInt32) -> UInt32 {
        UInt32 {
            value: self.value >> other.value,
        }
    }
}
impl BitAnd for UInt32 {
    type Output = UInt32;
    fn bitand(self, other: UInt32) -> UInt32 {
        UInt32 {
            value: self.value & other.value,
        }
    }
}
impl BitOr for UInt32 {
    type Output = UInt32;
    fn bitor(self, other: UInt32) -> UInt32 {
        UInt32 {
            value: self.value | other.value,
        }
    }
}
impl BitXor for UInt32 {
    type Output = UInt32;
    fn bitxor(self, other: UInt32) -> UInt32 {
        UInt32 {
            value: self.value ^ other.value,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq, Hash)]
pub struct UInt64 {
    pub value: u64,
}

impl UInt64 {
    pub fn low(&self) -> UInt32 {
        UInt32 {
            value: (self.value & 0xFFFFFFFF) as u32,
        }
    }

    pub fn high(&self) -> UInt32 {
        UInt32 {
            value: (self.value >> 32) as u32,
        }
    }
}

impl From<u64> for UInt64 {
    fn from(value: u64) -> UInt64 {
        UInt64 { value }
    }
}

impl ProverType for UInt64 {
    fn calc(&self) -> String {
        self.value.to_string()
    }
    fn r#type() -> String {
        "UInt64".to_string()
    }
}

impl Add for UInt64 {
    type Output = UInt64;
    fn add(self, other: UInt64) -> UInt64 {
        UInt64 {
            value: self.value.wrapping_add(other.value),
        }
    }
}
impl Rem for UInt64 {
    type Output = UInt64;
    fn rem(self, other: UInt64) -> UInt64 {
        UInt64 {
            value: self.value % other.value,
        }
    }
}
impl Shl for UInt64 {
    type Output = UInt64;
    fn shl(self, other: UInt64) -> UInt64 {
        UInt64 {
            value: self.value << other.value,
        }
    }
}
impl Shr for UInt64 {
    type Output = UInt64;
    fn shr(self, other: UInt64) -> UInt64 {
        UInt64 {
            value: self.value >> other.value,
        }
    }
}
impl BitAnd for UInt64 {
    type Output = UInt64;
    fn bitand(self, other: UInt64) -> UInt64 {
        UInt64 {
            value: self.value & other.value,
        }
    }
}
impl BitOr for UInt64 {
    type Output = UInt64;
    fn bitor(self, other: UInt64) -> UInt64 {
        UInt64 {
            value: self.value | other.value,
        }
    }
}
impl BitXor for UInt64 {
    type Output = UInt64;
    fn bitxor(self, other: UInt64) -> UInt64 {
        UInt64 {
            value: self.value ^ other.value,
        }
    }
}

pub const FELT252_N_WORDS: usize = 28;
pub const FELT252_BITS_PER_WORD: usize = 9;

// NOTE! This assumes Felt252 has shape (28, 9).
pub const P_FELTS: [u32; FELT252_N_WORDS] = [
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136, 0, 0, 0, 0, 0, 256,
];

// A non-redundant representation of a 252-bit element in the field of numbers
// modulo the prime 2**251 + 17 * 2**192 + 1.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq, Hash)]
pub struct Felt252 {
    // Ordered least to most significant.
    pub limbs: [u64; 4],
}

impl Felt252 {
    pub fn get_m31(&self, index: usize) -> M31 {
        let mask = (1u64 << FELT252_BITS_PER_WORD) - 1;
        let shift = FELT252_BITS_PER_WORD * index;
        let low_limb = shift / 64;
        let shift_low = shift & 0x3F;
        let high_limb = (shift + FELT252_BITS_PER_WORD - 1) / 64;
        let value = if low_limb == high_limb {
            ((self.limbs[low_limb] >> (shift_low)) & mask) as u32
        } else {
            (((self.limbs[low_limb] >> (shift_low)) | (self.limbs[high_limb] << (64 - shift_low)))
                & mask) as u32
        };
        M31::from_u32_unchecked(value)
    }

    pub fn get_limbs(&self) -> [M31; FELT252_N_WORDS] {
        from_fn(|i| self.get_m31(i))
    }

    pub fn from_limbs(felts: &[M31]) -> Self {
        assert!(felts.len() <= FELT252_N_WORDS, "Invalid number of felts");
        let mut limbs = [0u64; 4];
        for (index, felt) in felts.iter().enumerate() {
            let shift = FELT252_BITS_PER_WORD * index;
            let shift_low = shift & 0x3F;
            let low_limb = shift / 64;
            let high_limb = (shift + FELT252_BITS_PER_WORD - 1) / 64;
            limbs[low_limb] |= (felt.0 as u64) << shift_low;
            if high_limb != low_limb {
                limbs[high_limb] |= (felt.0 as u64) >> (64 - shift_low);
            }
        }

        Self { limbs }
    }

    pub fn from_m31(felt: M31) -> Self {
        Self {
            limbs: [felt.0 as u64, 0, 0, 0],
        }
    }
}

// Convert between Felt252 and FieldElement for performing field operations.
// Note that FieldElements are in Montgomery form, and for efficiency and simplicity, we skip the
// conversion in both directions. We thus have to compensate with extra factors when performing
// multiplication and division. We also ensure the FieldElement is reduced modulo P, by adding zero
// to the initial conversion. This results in subtracting P once if needed, which suffices as the
// limbs of a Felt252 are smaller than 2**252 < 2*P.
impl From<Felt252> for FieldElement {
    fn from(n: Felt252) -> FieldElement {
        FieldElement::from_mont(n.limbs) + FieldElement::ZERO
    }
}
impl From<FieldElement> for Felt252 {
    fn from(n: FieldElement) -> Felt252 {
        Felt252 {
            limbs: n.into_mont(),
        }
    }
}

impl From<StarknetTypesFelt> for Felt252 {
    fn from(value: StarknetTypesFelt) -> Self {
        Felt252 {
            limbs: value.to_le_digits(),
        }
    }
}
impl From<Felt252> for StarknetTypesFelt {
    fn from(value: Felt252) -> StarknetTypesFelt {
        StarknetTypesFelt::from_bytes_le_slice(&value.limbs.map(|limb| limb.to_le_bytes()).concat())
    }
}

impl Add for Felt252 {
    type Output = Felt252;
    fn add(self, other: Felt252) -> Felt252 {
        let self_ff: FieldElement = self.into();
        let other_ff: FieldElement = other.into();

        (self_ff + other_ff).into()
    }
}

impl Sub for Felt252 {
    type Output = Felt252;
    fn sub(self, other: Felt252) -> Felt252 {
        let self_ff: FieldElement = self.into();
        let other_ff: FieldElement = other.into();

        (self_ff - other_ff).into()
    }
}

// This value is equal to 2**512 % PRIME, which compensates for the two Montgomery divisions
// by 2**256 performed in the two multiplications below.
const FELT252_MONT_MUL_FACTOR: FieldElement = FieldElement::from_mont([
    18446741271209837569,
    5151653887,
    18446744073700081664,
    576413109808302096,
]);

impl Mul for Felt252 {
    type Output = Felt252;
    fn mul(self, other: Felt252) -> Felt252 {
        let self_ff: FieldElement = self.into();
        let other_ff: FieldElement = other.into();

        (self_ff * other_ff * FELT252_MONT_MUL_FACTOR).into()
    }
}

// The Montgomery inversion adds a factor of 2**512 to the inverse of `other`, so it is only
// necessary to perform one more Montgomery reduction after computing self * other.invert().
// The reduction is accessible by multipliying by 1 (i.e. the Montgomery form of 2**-256).
const FELT252_MONT_DIV_FACTOR: FieldElement = FieldElement::from_mont([1, 0, 0, 0]);

impl Div for Felt252 {
    type Output = Felt252;
    fn div(self, other: Felt252) -> Felt252 {
        let self_ff: FieldElement = self.into();
        let other_ff: FieldElement = other.into();

        (self_ff * other_ff.invert().expect("Division by zero") * FELT252_MONT_DIV_FACTOR).into()
    }
}

impl From<[u64; 4]> for Felt252 {
    fn from(limbs: [u64; 4]) -> Felt252 {
        Felt252 {
            limbs: [
                limbs[0],
                limbs[1],
                limbs[2],
                limbs[3] & 0x0fffffff_ffffffffu64,
            ],
        }
    }
}

impl ProverType for Felt252 {
    fn calc(&self) -> String {
        format!(
            "[{}, {}, {}, {}]",
            self.limbs[0], self.limbs[1], self.limbs[2], self.limbs[3]
        )
    }
    fn r#type() -> String {
        "Felt252".to_string()
    }
}

pub const FELT252WIDTH27_N_WORDS: usize = 10;
pub const FELT252WIDTH27_BITS_PER_WORD: usize = 27;

pub const P_PACKED27_FELTS: [u32; FELT252WIDTH27_N_WORDS] = [1, 0, 0, 0, 0, 0, 0, 136, 0, 256];
/// A version of Felt252 whose values are packed into 27-bit limbs instead of 9-bit.
/// The only supported operations are conversions to and from Felt252 and FieldElement.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq, Hash)]
pub struct Felt252Width27 {
    pub limbs: [u64; 4],
}

impl Felt252Width27 {
    pub fn get_m31(&self, index: usize) -> M31 {
        let mask = (1u64 << FELT252WIDTH27_BITS_PER_WORD) - 1;
        let shift = FELT252WIDTH27_BITS_PER_WORD * index;
        let low_limb = shift / 64;
        let shift_low = shift & 0x3F;
        let high_limb = (shift + FELT252WIDTH27_BITS_PER_WORD - 1) / 64;
        let value = if low_limb == high_limb || index == (FELT252WIDTH27_N_WORDS - 1) {
            ((self.limbs[low_limb] >> (shift_low)) & mask) as u32
        } else {
            (((self.limbs[low_limb] >> (shift_low)) | (self.limbs[high_limb] << (64 - shift_low)))
                & mask) as u32
        };
        M31::from_u32_unchecked(value)
    }

    pub fn from_limbs(felts: &[M31]) -> Self {
        assert!(
            felts.len() <= FELT252WIDTH27_N_WORDS,
            "Invalid number of felts"
        );
        let mut limbs = [0u64; 4];
        for (index, felt) in felts.iter().enumerate() {
            let shift = FELT252WIDTH27_BITS_PER_WORD * index;
            let shift_low = shift & 0x3F;
            let low_limb = shift / 64;
            let high_limb = (shift + FELT252WIDTH27_BITS_PER_WORD - 1) / 64;
            limbs[low_limb] |= (felt.0 as u64) << shift_low;
            if high_limb != low_limb && index < (FELT252WIDTH27_N_WORDS - 1) {
                limbs[high_limb] |= (felt.0 as u64) >> (64 - shift_low);
            }
        }

        Self { limbs }
    }

    pub fn from_m31(felt: M31) -> Self {
        Self {
            limbs: [felt.0 as u64, 0, 0, 0],
        }
    }
}

impl From<Felt252> for Felt252Width27 {
    fn from(n: Felt252) -> Felt252Width27 {
        Felt252Width27 { limbs: n.limbs }
    }
}

impl From<Felt252Width27> for Felt252 {
    fn from(n: Felt252Width27) -> Felt252 {
        Felt252 { limbs: n.limbs }
    }
}

// Convert between Felt252Width27 and FieldElement for performing field operations.
// See the documentation of the conversions between Felt252<>FieldElement for more details.
impl From<Felt252Width27> for FieldElement {
    fn from(n: Felt252Width27) -> FieldElement {
        FieldElement::from_mont(n.limbs) + FieldElement::ZERO
    }
}
impl From<FieldElement> for Felt252Width27 {
    fn from(n: FieldElement) -> Felt252Width27 {
        Felt252Width27 {
            limbs: n.into_mont(),
        }
    }
}

impl ProverType for Felt252Width27 {
    fn calc(&self) -> String {
        format!(
            "[{}, {}, {}, {}]",
            self.limbs[0], self.limbs[1], self.limbs[2], self.limbs[3]
        )
    }
    fn r#type() -> String {
        "Felt252Width27".to_string()
    }
}

// Length of each modulo builtin word in bits.
pub const MOD_BUILTIN_WORD_BIT_LEN: usize = 96;
// Length of each biguint word in bits.
pub const BIGUINT_BITS_PER_WORD: usize = 12;

// B is the number of bits in the BigUInt.
// L is the number of 64-bit limbs in the BigUInt.
// F is the number of BIGUINT_BITS_PER_M31-bit limbs in the BigUInt.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct BigUInt<const B: usize, const L: usize, const F: usize> {
    #[serde(with = "serde_arrays")]
    pub limbs: [u64; L],
}

impl<const B: usize, const L: usize, const F: usize> Default for BigUInt<B, L, F> {
    fn default() -> Self {
        Self { limbs: [0; L] }
    }
}

impl<const B: usize, const L: usize, const F: usize> ProverType for BigUInt<B, L, F> {
    fn calc(&self) -> String {
        format!("{:?}", self.limbs)
    }
    fn r#type() -> String {
        match L {
            6 | 12 => format!(
                "BigUInt<{}, {}, {}>",
                64 * L,
                L,
                (64 * L) / BIGUINT_BITS_PER_WORD
            ),
            _ => panic!("Unsupported BigUInt size"),
        }
    }
}

// Convert between BigUInt and Uint for performing field operations.
impl<const B: usize, const L: usize, const F: usize> From<BigUInt<B, L, F>> for Uint<B, L> {
    fn from(n: BigUInt<B, L, F>) -> Uint<B, L> {
        Uint::from_limbs(n.limbs)
    }
}

impl<const B: usize, const L: usize, const F: usize> From<Uint<B, L>> for BigUInt<B, L, F> {
    fn from(n: Uint<B, L>) -> BigUInt<B, L, F> {
        let limbs = n.into_limbs();
        BigUInt { limbs }
    }
}

impl<const B: usize, const L: usize, const F: usize> From<[u64; L]> for BigUInt<B, L, F> {
    fn from(limbs: [u64; L]) -> BigUInt<B, L, F> {
        BigUInt { limbs }
    }
}

impl<const B: usize, const L: usize, const F: usize> BigUInt<B, L, F> {
    pub fn from_felt252_array(mod_words: Vec<Felt252>) -> Self {
        // only takes MODULO_WORD_BIT_LEN from each Felt252
        let needed_bits = mod_words.len() * MOD_BUILTIN_WORD_BIT_LEN;
        assert!(needed_bits <= B, "BigUIntExpr can have at most {B} bits");
        let needed_limbs_per252 = MOD_BUILTIN_WORD_BIT_LEN.div_ceil(64);
        let inner_limbs_lengths = vec![64, 32];
        let word_pieces = mod_words
            .iter()
            .flat_map(|f| {
                f.limbs[..needed_limbs_per252]
                    .iter()
                    .copied()
                    .zip(inner_limbs_lengths.clone())
            })
            .collect::<Vec<_>>();

        let mut cum_len = 0;
        let mut limbs = [0u64; L];
        for (limb, len) in word_pieces.iter() {
            let shift_low = cum_len & 0x3F;
            let low_limb = cum_len / 64;
            let high_limb = (cum_len + len - 1) / 64;
            limbs[low_limb] |= limb << shift_low;
            if high_limb != low_limb {
                limbs[high_limb] |= limb >> (64 - shift_low);
            }
            cum_len += len;
        }
        assert!(cum_len <= B, "BigUInt is too big");

        Self { limbs }
    }

    // Returns the BIGUINT_BITS_PER_WORD-bit word at the given index.
    pub fn get_m31(&self, index: usize) -> M31 {
        let mask = (1u64 << BIGUINT_BITS_PER_WORD) - 1;
        let shift = BIGUINT_BITS_PER_WORD * index;
        let low_limb = shift / 64;
        let shift_low = shift & 0x3F;
        let high_limb = (shift + BIGUINT_BITS_PER_WORD - 1) / 64;
        let value = if low_limb == high_limb {
            ((self.limbs[low_limb] >> (shift_low)) & mask) as u32
        } else {
            (((self.limbs[low_limb] >> (shift_low)) | (self.limbs[high_limb] << (64 - shift_low)))
                & mask) as u32
        };
        M31::from_u32_unchecked(value)
    }

    pub fn from_biguint<const OB: usize, const OL: usize, const OF: usize>(
        other: BigUInt<OB, OL, OF>,
    ) -> Self {
        if OL > L && !other.limbs.iter().skip(L).all(|&x| x == 0) {
            panic!("BigUInt is too big");
        }

        let mut limbs = [0; L];
        for (i, l) in other.limbs.iter().take(L).enumerate() {
            limbs[i] = *l;
        }
        Self { limbs }
    }

    pub fn widening_mul<const DB: usize, const DL: usize, const DF: usize>(
        self,
        rhs: BigUInt<B, L, F>,
    ) -> BigUInt<DB, DL, DF> {
        let self_uint: Uint<B, L> = self.into();
        let rhs_uint: Uint<B, L> = rhs.into();

        (self_uint.widening_mul(rhs_uint)).into()
    }

    pub fn from_felt252(felt: Felt252) -> Self {
        assert!(L >= 4, "BigUInt is too small");
        let mut limbs = [0; L];
        for (i, l) in felt.limbs.iter().enumerate() {
            limbs[i] = *l;
        }
        Self { limbs }
    }
}

impl<const B: usize, const L: usize, const F: usize> Add for BigUInt<B, L, F> {
    type Output = BigUInt<B, L, F>;
    fn add(self, other: BigUInt<B, L, F>) -> BigUInt<B, L, F> {
        let self_uint: Uint<B, L> = self.into();
        let other_uint: Uint<B, L> = other.into();

        (self_uint + other_uint).into()
    }
}

impl<const B: usize, const L: usize, const F: usize> Sub for BigUInt<B, L, F> {
    type Output = BigUInt<B, L, F>;
    fn sub(self, other: BigUInt<B, L, F>) -> BigUInt<B, L, F> {
        let self_uint: Uint<B, L> = self.into();
        let other_uint: Uint<B, L> = other.into();

        (self_uint - other_uint).into()
    }
}

impl<const B: usize, const L: usize, const F: usize> Mul for BigUInt<B, L, F> {
    type Output = BigUInt<B, L, F>;
    fn mul(self, other: BigUInt<B, L, F>) -> BigUInt<B, L, F> {
        let self_uint: Uint<B, L> = self.into();
        let other_uint: Uint<B, L> = other.into();

        (self_uint * other_uint).into()
    }
}

impl<const B: usize, const L: usize, const F: usize> Div for BigUInt<B, L, F> {
    type Output = BigUInt<B, L, F>;
    fn div(self, other: BigUInt<B, L, F>) -> BigUInt<B, L, F> {
        let self_uint: Uint<B, L> = self.into();
        let other_uint: Uint<B, L> = other.into();

        (self_uint / other_uint).into()
    }
}
