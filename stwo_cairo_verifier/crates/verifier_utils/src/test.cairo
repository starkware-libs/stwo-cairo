#[cfg(not(feature: "poseidon252_verifier"))]
use crate::hash_memory_section_with_digest;
use crate::{construct_f252, deconstruct_f252, encode_and_hash_memory_section, hash_memory_section};


#[cfg(not(feature: "poseidon252_verifier"))]
#[test]
fn test_hash_memory_section() {
    let section = array![
        (0, [1, 2, 3, 4, 5, 6, 7, 8]), (0, [2, 3, 4, 5, 6, 7, 8, 9]),
        (0, [3, 4, 5, 6, 7, 8, 9, 10]),
    ]
        .span();

    assert_eq!(
        hash_memory_section(section).unbox(),
        [
            3098114871, 843612567, 2372208999, 1823639248, 1136624132, 2551058277, 1389013608,
            1207876589,
        ],
    );
}

#[cfg(not(feature: "poseidon252_verifier"))]
#[test]
fn test_hash_memory_section_with_digest() {
    let section = array![
        (0, [1, 2, 3, 4, 5, 6, 7, 8]), (0, [2, 3, 4, 5, 6, 7, 8, 9]),
        (0, [3, 4, 5, 6, 7, 8, 9, 10]),
    ]
        .span();

    assert_eq!(
        hash_memory_section_with_digest(section, BoxTrait::new([4, 5, 6, 7, 8, 9, 10, 11])).unbox(),
        [
            1479455185, 3761196396, 3867379740, 2755766202, 1450301503, 3044597930, 1024740531,
            2933086058,
        ],
    );
}

#[cfg(not(feature: "poseidon252_verifier"))]
#[test]
fn test_encode_and_hash_memory_section() {
    let section = array![
        (0, [1, 2, 3, 4, 5, 6, 7, 8]), (0, [2, 3, 4, 5, 6, 7, 8, 9]),
        (0, [3, 4, 5, 6, 7, 8, 9, 10]),
    ]
        .span();

    assert_eq!(
        encode_and_hash_memory_section(section).unbox(),
        [
            2282645979, 950124099, 2516699392, 1970475562, 1990856745, 3122040212, 3513633402,
            1645114886,
        ],
    );
}

#[cfg(feature: "poseidon252_verifier")]
#[test]
fn test_hash_memory_section() {
    let section = array![
        (0, [1, 2, 3, 4, 5, 6, 7, 8]), (0, [2, 3, 4, 5, 6, 7, 8, 9]),
        (0, [3, 4, 5, 6, 7, 8, 9, 10]),
    ]
        .span();

    assert_eq!(
        hash_memory_section(section).unbox(),
        [
            2433336977, 2153250057, 881002283, 2835163344, 2300811583, 376217666, 1436681392,
            91789842,
        ],
    );
}

#[cfg(feature: "poseidon252_verifier")]
#[test]
fn test_encode_and_hash_memory_section() {
    let section = array![
        (0, [1, 2, 3, 4, 5, 6, 7, 8]), (0, [2, 3, 4, 5, 6, 7, 8, 9]),
        (0, [3, 4, 5, 6, 7, 8, 9, 10]),
    ]
        .span();

    assert_eq!(
        encode_and_hash_memory_section(section).unbox(),
        [
            2433336977, 2153250057, 881002283, 2835163344, 2300811583, 376217666, 1436681392,
            91789842,
        ],
    );
}

#[test]
fn test_construct_felt() {
    assert_eq!(
        construct_f252(BoxTrait::new([1_u32, 2, 3, 4, 5, 6, 7, 8])),
        0x800000007000000060000000500000004000000030000000200000001,
    );
}

#[test]
fn test_deconstruct_felt() {
    assert_eq!(
        deconstruct_f252(0x800000007000000060000000500000004000000030000000200000001).unbox(),
        [1_u32, 2, 3, 4, 5, 6, 7, 8],
    );
}
