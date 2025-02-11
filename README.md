# stwo-cairo-air

# Generate and verify proof

Install `cairo-execute`

```bash
git clone https://github.com/starkware-libs/cairo
cd cairo
export CAIRO_PATH=$(pwd)
cargo build --release
```

Generate the cairo proof.

```bash
(cd ./stwo_cairo_prover && bash ./scripts/fetch_large_files.sh ./scripts/slow_tests.json)
(cd ./stwo_cairo_prover && cargo run -r --bin adapted_stwo -- --pub_json ./crates/prover/test_data/test_read_from_small_files/pub.json --priv_json ./crates/prover/test_data/test_read_from_small_files/priv.json --proof_path /dev/null --cairo_proof_path cairo_proof.txt)
cat ./stwo_cairo_prover/cairo_proof.txt | python3 converter.py > /tmp/proof_params.json
```

Verify the proof.

Clone to the branch with the latest verifier

```
git clone -b 02-11-Add_Scarb_manifest_for_cairo_verifier_crate git@github.com:starkware-libs/stwo-cairo.git
```

Run the verifier from within the repo.

```
$CAIRO_PATH/target/release/cairo-execute ./stwo_cairo_verifier --executable stwo_cairo_verifier::main --layout=starknet --args-file ./proof_params.json  --print-outputs --output-path ./out --allow-warnings --ignore-warnings
```