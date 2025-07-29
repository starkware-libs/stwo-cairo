# Compact binary format specs

## Needs

Stwo proofs can be serialized in two formats:

- `json`: each field of each struct is serialized as base json format.
- `cairo-serde`: the proof is first converted into a `Vec<FieldElement>`, that is then serialized in a serde json format.

The aim is to design and implement a third proof serialization format, `compact-binary`, where the proof is serialized as a `Vec<u8>` in a compact way.

## Format description

- Integers (`u32`, `u64` and `usize`) should be handled as VarInts (see ...)
- Relevant `FieldElement` fields should be compressed (e.g. public inputs, but not FRI layers, that are expected to be random)
- Structured data should have:
  - versions, to be able to update the structure
  - tags for each field, to be able to add new fields

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

For this example proof, here are the results:

| File                      | Format         | Size on disk (bytes) | Gain    |
|---------------------------|----------------|---------------------:|--------:|
| example_proof.base_json   | json           |          12 364 809  |    --   |
| example_proof.cairo_serde | cairo-serde    |           3 224 193  |  73.9 % |
| example_proof.compact_bin | compact-binary |             834 606  |  93.2 % |

## Potential improvements

Here is a list of potential improvements that can be made:

- Better error handling (by returning a `Result<(&[u8], DeserializedData)>` in `compact_deserialize()`) if needed instead of panics.
- ?
