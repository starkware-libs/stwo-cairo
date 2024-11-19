use std::fmt::Debug;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub};

use ruint::Uint;
use serde::{Deserialize, Serialize};
use starknet_ff::FieldElement;

pub type M31 = stwo_prover::core::fields::m31::M31;

pub const PRIME: u32 = 2_u32.pow(31) - 1;

pub trait AlgebraicType: ProverType + Add + Sub + Mul + Div {}
impl AlgebraicType for M31 {}
impl AlgebraicType for Felt252 {}
impl<const B: usize, const L: usize> AlgebraicType for BigUInt<B, L> {}

pub trait NumericType: ProverType + Rem + Shl + Shr + BitAnd + BitOr + BitXor {}
impl NumericType for UInt16 {}
impl NumericType for UInt32 {}
impl NumericType for UInt64 {}

pub trait SingleFeltType: ProverType {
    fn as_m31(&self) -> M31;
}

/// Expression Types - the basic type of the variables composing the expression.
/// For exaple, felt or bool. The expression types are devided into group, depending on
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq, Hash)]
pub struct CasmState {
    pub pc: M31,
    pub ap: M31,
    pub fp: M31,
}

impl CasmState {
    pub fn values(&self) -> [M31; 3] {
        [self.pc, self.ap, self.fp]
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
pub const P_BIG_UINT_LIMBS: [u64; 4] = [1, 0, 0, 2_u64.pow(59) + 17];

// A non-redundant representation of a 252-bit element in the field of numbers
// modulo the prime 2**251 + 17 * 2**192 + 1.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq, Hash)]
pub struct Felt252 {
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

    pub fn from_biguint256(biguint: BigUInt<256, 4>) -> Self {
        assert!(
            Uint::<256, 4>::from_limbs(biguint.limbs) < Uint::from_limbs(P_BIG_UINT_LIMBS),
            "BigUInt is too big"
        );

        Self {
            limbs: biguint.limbs,
        }
    }
}

// Convert between Felt252 and FieldElement for performing field operations.
// Note that FieldElements are in Montgomery form, and for efficiency and simplicity, we skip the
// conversion in both direction. We thus have to compensate with extra factors when performing
// multiplication and division. We also ensure the FieldElement is reduced modulo P, by adding zero
// on initial conversion. This results in subtracting P once if needed, which suffices as as the
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct BigUInt<const B: usize, const L: usize> {
    pub limbs: [u64; L],
}

impl<const B: usize, const L: usize> Default for BigUInt<B, L> {
    fn default() -> Self {
        Self { limbs: [0; L] }
    }
}

impl<const B: usize, const L: usize> ProverType for BigUInt<B, L> {
    fn calc(&self) -> String {
        format!("{:?}", self.limbs)
    }
    fn r#type() -> String {
        match L {
            6 | 12 => format!("BigUInt<{}, {}>", 64 * L, L),
            _ => panic!("Unsupported BigUInt size"),
        }
    }
}

// Convert between BigUInt and Uint for performing field operations.
impl<const B: usize, const L: usize> From<BigUInt<B, L>> for Uint<B, L> {
    fn from(n: BigUInt<B, L>) -> Uint<B, L> {
        Uint::from_limbs(n.limbs)
    }
}

impl<const B: usize, const L: usize> From<Uint<B, L>> for BigUInt<B, L> {
    fn from(n: Uint<B, L>) -> BigUInt<B, L> {
        let limbs = n.into_limbs();
        BigUInt { limbs }
    }
}

impl<const B: usize, const L: usize> From<[u64; L]> for BigUInt<B, L> {
    fn from(limbs: [u64; L]) -> BigUInt<B, L> {
        BigUInt { limbs }
    }
}

impl<const B: usize, const L: usize> BigUInt<B, L> {
    pub fn get_u64(&self, index: usize) -> UInt64 {
        UInt64 {
            value: self.limbs[index],
        }
    }

    pub fn from_limbs(limbs: Vec<UInt64>) -> Self {
        assert!(limbs.len() <= L, "Invalid number of limbs");
        let mut res = [0; L];
        for (index, limb) in limbs.iter().enumerate() {
            res[index] = limb.value;
        }
        Self { limbs: res }
    }

    pub fn from_u64(limb: UInt64) -> Self {
        let mut limbs = [0; L];
        limbs[0] = limb.value;
        Self { limbs }
    }

    pub fn from_biguint<const OB: usize, const OL: usize>(other: BigUInt<OB, OL>) -> Self {
        if OL > L && !other.limbs.iter().skip(L).all(|&x| x == 0) {
            panic!("BigUInt is too big");
        }

        let mut limbs = [0; L];
        for (i, l) in other.limbs.iter().take(L).enumerate() {
            limbs[i] = *l;
        }
        Self { limbs }
    }

    pub fn widening_mul<const DB: usize, const DL: usize>(
        self,
        rhs: BigUInt<B, L>,
    ) -> BigUInt<DB, DL> {
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

impl<const B: usize, const L: usize> Add for BigUInt<B, L> {
    type Output = BigUInt<B, L>;
    fn add(self, other: BigUInt<B, L>) -> BigUInt<B, L> {
        let self_uint: Uint<B, L> = self.into();
        let other_uint: Uint<B, L> = other.into();

        (self_uint + other_uint).into()
    }
}

impl<const B: usize, const L: usize> Sub for BigUInt<B, L> {
    type Output = BigUInt<B, L>;
    fn sub(self, other: BigUInt<B, L>) -> BigUInt<B, L> {
        let self_uint: Uint<B, L> = self.into();
        let other_uint: Uint<B, L> = other.into();

        (self_uint - other_uint).into()
    }
}

impl<const B: usize, const L: usize> Mul for BigUInt<B, L> {
    type Output = BigUInt<B, L>;
    fn mul(self, other: BigUInt<B, L>) -> BigUInt<B, L> {
        let self_uint: Uint<B, L> = self.into();
        let other_uint: Uint<B, L> = other.into();

        (self_uint * other_uint).into()
    }
}

impl<const B: usize, const L: usize> Div for BigUInt<B, L> {
    type Output = BigUInt<B, L>;
    fn div(self, other: BigUInt<B, L>) -> BigUInt<B, L> {
        let self_uint: Uint<B, L> = self.into();
        let other_uint: Uint<B, L> = other.into();

        (self_uint / other_uint).into()
    }
}
