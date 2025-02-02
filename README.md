# stwo-cairo-air

# Generate and verify proof

Install `cairo-execute`

```bash
git clone https://github.com/starkware-libs/cairo
cd cairo
export CAIRO_PATH=$(pwd)
cargo build --release
```

Generate and verify the cairo proof.

```bash
(cd ./stwo_cairo_prover && bash ./scripts/fetch_large_files.sh ./scripts/slow_tests.json)
(cd ./stwo_cairo_prover && cargo run -r --bin adapted_stwo -- --pub_json ./crates/prover/test_data/test_read_from_small_files/pub.json --priv_json ./crates/prover/test_data/test_read_from_small_files/priv.json --proof_path /dev/null --cairo_proof_path cairo_proof.txt)
cat ./stwo_cairo_prover/cairo_proof.txt | python3 converter.py > ./proof_params.json
$CAIRO_PATH/target/release/cairo-execute ./stwo_cairo_verifier --executable stwo_cairo_verifier::main --layout=starknet --args-file ./proof_params.json  --print-outputs --output-path ./ --allow-warnings --ignore-warnings
```