# Cairo-Prove

Blazing ⚡ fast ⚡ Cairo prover.

## Prerequisites

- Rust, version mentioned in `rust-toolchain.toml`.
- Scarb (Optional/Recommended). To run the example it's recommended to compile with `scarb 2.12.2`.

## Installation

```bash
# Clone the repository
git clone https://github.com/starkware-libs/stwo-cairo.git
cd stwo-cairo/cairo-prove

# Build the project,
./build.sh

# Optional: Add the binary to your PATH
# For example, to add it to /usr/local/bin:
sudo cp target/release/cairo-prove /usr/local/bin/
```

Alternatively use cargo install:

```bash
cargo install --git https://github.com/starkware-libs/stwo-cairo cairo-prove
```

## Usage

### Compiling a project.

Use scarb to compile an executable:
```bash
cd example
scarb  --profile release build 
```

### Generating a Proof

To generate a proof for a Cairo program:

```bash
cairo-prove prove <path-to-cairo-program> <output-proof-path> --arguments <args> 
```

Example:
```bash
cairo-prove prove target/release/example.executable.json ./example_proof.json --arguments 10000
```

#### Loading arguments from file

You can also provide program arguments via JSON file using the following format (compatible with `scarb execute`):

```json
["0x01", "0x02", "0x03"]
```

Use `--arguments-file` command line option:

```sh
cairo-prove prove target/release/example.executable.json ./example_proof.json --arguments-file ./args.json
```

#### Serializing proof for recursive verification

By default the proof is serialized to file using serde-json, but if you need to later use the proof as argument for Stwo Cairo verifier set `--proof-format cairo-serde` CLI option.

Example:

```bash
cairo-prove prove \
    target/release/example.executable.json \
    ./example_proof.json \
    --arguments-file ./args.json \
    --proof-format cairo-serde
```

### Verifying a Proof

To verify an existing proof:

```bash
cairo-prove verify <path-to-proof-file>
```

Example:
```bash
cairo-prove verify ./example_proof.json
```

## Pre-processed trace:
When pedersen is used in the proof, more pre-processed columns are needed. The variant is automatically deduced during `prove`. 
Verify with `--with-pedersen`.

## Note:
The currently used cairo-vm version is `starkware-development` rather than an official release. 
This is temporary.
