use num_traits::One;
use stwo::prover::backend::simd::conversion::{Pack, Unpack};
use stwo::prover::backend::simd::m31::PackedM31;
use stwo_cairo_common::preprocessed_consts::sha256::K;
use stwo_cairo_common::prover_types::cpu::{UInt32, M31};
use stwo_cairo_common::prover_types::simd::PackedUInt32;

fn big_sigma0(val: u32) -> UInt32 {
    UInt32::from(val.rotate_right(2) ^ val.rotate_right(13) ^ val.rotate_right(22))
}
fn big_sigma1(val: u32) -> UInt32 {
    UInt32::from(val.rotate_right(6) ^ val.rotate_right(11) ^ val.rotate_right(25))
}
fn small_sigma0(val: u32) -> UInt32 {
    UInt32::from(val.rotate_right(7) ^ val.rotate_right(18) ^ val >> 3)
}
fn small_sigma1(val: u32) -> UInt32 {
    UInt32::from(val.rotate_right(17) ^ val.rotate_right(19) ^ val >> 10)
}

pub struct Sha256Round {}
impl Sha256Round {
    pub fn deduce_output(
        chain: M31,
        round: M31,
        state: ([UInt32; 8], [UInt32; 16]),
    ) -> (M31, M31, ([UInt32; 8], [UInt32; 16])) {
        let (
            [a, b, c, d, e, f, g, h],
            [w0, w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13, w14, w15],
        ) = state;

        let ch = |e: UInt32, f: UInt32, g: UInt32| -> UInt32 {
            (e & f) ^ ((UInt32::from(!e.value)) & g)
        };
        // We're computing w[16]
        // w[16 - 15] = w[1]

        // w[16 - 16] = w[0] and w[16 - 7] = w[9]
        let w16 = UInt32::from(
            ((w0.value as u64
                + small_sigma0(w0.value).value as u64
                + w9.value as u64
                + small_sigma1(w14.value).value as u64)
                % (1 << 32)) as u32,
        );

        let t1 = UInt32::from(
            (h.value as u64
                + big_sigma1(e.value).value as u64
                + ch(e, f, g).value as u64
                + K[round.0 as usize] as u64
                + w0.value as u64) as u32,
        );

        let maj = |a: UInt32, b: UInt32, c: UInt32| -> UInt32 {
            UInt32::from((a.value & b.value) ^ (a.value & c.value) ^ (b.value & c.value))
        };

        let t2 =
            UInt32::from((big_sigma0(a.value).value as u64 + maj(a, b, c).value as u64) as u32);

        let h = g;
        let g = f;
        let f = e;
        let e = d + t1;
        let d = c;
        let c = b;
        let b = a;
        let a = t1 + t2;
        (
            chain,
            round + M31::one(),
            (
                [a, b, c, d, e, f, g, h],
                [
                    w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13, w14, w15, w16,
                ],
            ),
        )
    }
}

pub struct PackedSha256Round {}
impl PackedSha256Round {
    pub fn deduce_output(
        input: (
            PackedM31,
            PackedM31,
            ([PackedUInt32; 8], [PackedUInt32; 16]),
        ),
    ) -> (
        PackedM31,
        PackedM31,
        ([PackedUInt32; 8], [PackedUInt32; 16]),
    ) {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(
            unpacked_inputs
                .map(|(chain, round, state)| Sha256Round::deduce_output(chain, round, state)),
        )
    }
}

pub struct PackedSha256Schedule {}
impl PackedSha256Schedule {
    pub fn deduce_output(input: [PackedUInt32; 16]) -> PackedUInt32 {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(unpacked_inputs.map(Sha256Schedule::deduce_output))
    }
}

pub struct Sha256Schedule {}
impl Sha256Schedule {
    pub fn deduce_output(input: [UInt32; 16]) -> UInt32 {
        let [w0, w1, _w2, _w3, _w4, _w5, _w6, _w7, _w8, w9, _w10, _w11, _w12, _w13, w14, _w15] =
            input;
        UInt32::from(
            ((w0.value as u64
                + small_sigma0(w1.value).value as u64
                + w9.value as u64
                + small_sigma1(w14.value).value as u64)
                % (1 << 32)) as u32,
        )
    }
}

pub struct PackedSha256KTable {}

impl PackedSha256KTable {
    pub fn deduce_output([input]: [PackedM31; 1]) -> PackedUInt32 {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(unpacked_inputs.map(Sha256KTable::deduce_output))
    }
}

pub struct Sha256KTable {}
impl Sha256KTable {
    pub fn deduce_output(round: M31) -> UInt32 {
        UInt32::from(K[round.0 as usize])
    }
}

pub struct PackedSha256BigSigma1 {}
impl PackedSha256BigSigma1 {
    pub fn deduce_output(input: PackedUInt32) -> PackedUInt32 {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(unpacked_inputs.map(Sha256BigSigma1::deduce_output))
    }
}

pub struct Sha256BigSigma1 {}
impl Sha256BigSigma1 {
    pub fn deduce_output(number: UInt32) -> UInt32 {
        big_sigma1(number.value)
    }
}
pub struct PackedSha256BigSigma0 {}
impl PackedSha256BigSigma0 {
    pub fn deduce_output(input: PackedUInt32) -> PackedUInt32 {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(unpacked_inputs.map(Sha256BigSigma0::deduce_output))
    }
}

pub struct Sha256BigSigma0 {}
impl Sha256BigSigma0 {
    pub fn deduce_output(number: UInt32) -> UInt32 {
        big_sigma0(number.value)
    }
}
pub struct PackedSha256SmallSigma0 {}
impl PackedSha256SmallSigma0 {
    pub fn deduce_output(input: PackedUInt32) -> PackedUInt32 {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(unpacked_inputs.map(Sha256SmallSigma0::deduce_output))
    }
}

pub struct Sha256SmallSigma0 {}
impl Sha256SmallSigma0 {
    pub fn deduce_output(number: UInt32) -> UInt32 {
        small_sigma0(number.value)
    }
}
pub struct PackedSha256SmallSigma1 {}
impl PackedSha256SmallSigma1 {
    pub fn deduce_output(input: PackedUInt32) -> PackedUInt32 {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(unpacked_inputs.map(Sha256SmallSigma1::deduce_output))
    }
}

pub struct Sha256SmallSigma1 {}
impl Sha256SmallSigma1 {
    pub fn deduce_output(number: UInt32) -> UInt32 {
        small_sigma1(number.value)
    }
}
