#![feature(portable_simd)]

pub mod binary_utils;
pub mod file_utils;
pub mod logging_utils;
pub mod vm_utils;

#[cfg(test)]
mod tests {
    use std::mem::transmute;

    use blake2::{Blake2s256, Digest};
    use cairo_vm::hint_processor::builtin_hint_processor::blake2s_hash::blake2s_compress;

    #[cfg(not(feature = "simd"))]
    macro_rules! decl_simd {
        ($($decl:item)*) => {
            $(
                #[derive(Clone, Copy, Debug)]
                #[repr(C)]
                $decl
            )*
        }
    }

    decl_simd! {
        pub struct Simd2<T>(pub T, pub T);
        pub struct Simd4<T>(pub T, pub T, pub T, pub T);
        pub struct Simd8<T>(pub T, pub T, pub T, pub T,
                            pub T, pub T, pub T, pub T);
        pub struct Simd16<T>(pub T, pub T, pub T, pub T,
                             pub T, pub T, pub T, pub T,
                             pub T, pub T, pub T, pub T,
                             pub T, pub T, pub T, pub T);
        pub struct Simd32<T>(pub T, pub T, pub T, pub T,
                             pub T, pub T, pub T, pub T,
                             pub T, pub T, pub T, pub T,
                             pub T, pub T, pub T, pub T,
                             pub T, pub T, pub T, pub T,
                             pub T, pub T, pub T, pub T,
                             pub T, pub T, pub T, pub T,
                             pub T, pub T, pub T, pub T);
    }

    pub type u32x4 = Simd4<u32>;

    #[test]
    fn test_blake_hash() {
        let mut hasher = Blake2s256::new();

        struct MyHasher {
            h: [u32x4; 2],
            t: u64,
            // h0: [u8; 32],
        }

        let init = unsafe { transmute::<Blake2s256, MyHasher>(hasher) };

        hasher.update(&[0; 64]);

        let compress_bytes = blake2s_compress(&[0; 8], &[0; 16], 64, 0, 0xffffffff, 0)
            .into_iter()
            .flat_map(|word| word.to_le_bytes())
            .collect::<Vec<u8>>();

        assert_eq!(compress_bytes, hasher.finalize().to_vec());
    }
}
