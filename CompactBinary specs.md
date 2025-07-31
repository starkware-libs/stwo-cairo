# Compact binary format specs

## Needs

Stwo proofs can be serialized in two formats:

- `json`: each field of each struct is serialized as base json format.
- `cairo-serde`: the proof is first converted into a `Vec<FieldElement>`, that is then serialized in a serde json format.

The aim is to design and implement a third proof serialization format, `compact-binary`, where the proof is serialized as a `Vec<u8>` in a compact way.

## Format description

- Integers (`u32`, `u64` and `usize`) should be handled as VarInts
- Relevant `FieldElement` fields should be compactified if possible, or compressed (e.g. public inputs, but not FRI layers, that are expected to be random)
- Structured data should have:
  - versions numbers, to be able to update the structure
  - tags for each field, to be able to add new fields easily

## Versioning description

If we want to add or change a field of a struct `StructA`, while still being able to deserialize previous versions of this struct, we should:

- Update `compact_serialize()` to serialize a new version number, and serialize the new struct
- Update `compact_deserialize()` to:
  - Get the version of the deserialized struct
  - Match on it and dispatch to the deserialization logic corresponding to this version

## Implementation

The current implementation consists of the following elements

- Updated argument handling for proof and verification in the CLI ( added `--proof-format compact-binary`). See `cairo-prove/src/main.rs` and `cairo-prove/src/args.rs`.
- Added `CompactBinary` trait in `stwo_cairo_prover/crates/cairo-serialize/src/compact.rs`, and implemented this trait for all structures needed.
- Added a `#[derive(CompactBinary)]` proc macro to implement the trait for structures composed of fields implementing it. Note that the proc macro is only expected to produce a `0` version, if a given structure is to be updated it's implementation should be done manually, while keeping back-compatibility of all previous serialization versions for this structure. See `stwo_cairo_prover/crates/cairo-serialize-derive/src/lib.rs`

## Tests and benchmarks

By following the `INSTRUCTIONS.md` document, you can serialize proofs using this new format as such:

```bash
cairo-prove/target/release/cairo-prove prove cairo-prove/example/target/dev/example.executable.json ./example_proof.compact_bin --arguments 10000 --proof-format compact-binary
cairo-prove/target/release/cairo-prove verify ./example_proof.compact_bin --proof-format compact-binary
```

**Note that we've adapted the serialize_proof_to_file() function to use serde_json without a JSON prettier to have more accurate results** 

For this example proof, here are the results: (compact_bin_v2 corresponds to a format where the fields of the CairoProof `claim`, `interaction_claim` and `stark_proof` are zipped with BZip2).

| File                               | Format                   | Size on disk (bytes) | Gain     |
|------------------------------------|--------------------------|---------------------:|---------:|
| example_proof.base_json            | json                     |           2 528 114  |    --    |
| example_proof.cairo_serde          | cairo-serde              |           2 448 494  |  - 3.1 % | 
| example_proof.compact_bin_unzipped | compact-binary-unzipped  |             834 606  | - 67.0 % |
| example_proof.compact_bin_zipped   | compact-binary-zipped    |             582 626  | - 77.0 % |

With a bigger proof:

| File                               | Format                   | Size on disk (bytes) | Gain     |
|------------------------------------|--------------------------|---------------------:|---------:|
| proof.json                         | json                     |          4 446 151   |    --    |
| proof.cairo_serde                  | cairo-serde              |          4 661 005   |  + 4.8 % |
| proof.compact_bin_unzipped         | compact-binary-unzipped  |          1 506 237   | - 66.1 % |
| proof.compact_bin_zipped           | compact-binary-zipped    |          1 458 557   | - 67.2 % |

## Potential improvements

Here is a list of potential improvements that can be made:

- Depending Better error handling (by returning a `Result<(&[u8], DeserializedData)>` in `compact_deserialize()`) if needed instead of panics.
