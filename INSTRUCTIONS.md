# Instructions
## Clone the repos:
```bash
git clone https://github.com/massalabs/stwo-cairo.git massa-stwo-cairo
cd massa-stwo-cairo
git checkout add_binary_proof_serialization
cd ../

git clone https://github.com/massalabs/stwo.git massa-stwo
cd massa-stwo
git checkout add_binary_proof_serialization
cd ../
```

## Installation
### Install scarb
https://docs.swmansion.com/scarb/download#install-via-installation-script

## Test the example
Go to the `massa-stwo-cairo` directory
```bash
cd massa-stwo-cairo
```

### Build the example
```bash
cd cairo-prove/examples/
scarb build
cd ../..
```

### Build cairo-prove
```bash
cd cairo-prove/
./build.sh
cd ..
```

## Test the example
```bash
cairo-prove/target/release/cairo-prove prove cairo-prove/example/target/dev/example.executable.json ./example_proof.compact_bin --arguments 10000 --proof-format compact-binary
```
