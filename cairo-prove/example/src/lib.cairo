use core::blake::blake2s_compress;

#[executable]
fn main(mut n: felt252) -> [u32; 8] {
    chain_hash(n)
}

fn chain_hash(mut n: felt252) -> [u32; 8] {
    let mut hash = BoxTrait::new([0; 8]);
    while n != 0 {
        hash = blake2s_compress(hash, 0, BoxTrait::new([0; 16]));
        n = n - 1;
    }
    hash.unbox()
}
