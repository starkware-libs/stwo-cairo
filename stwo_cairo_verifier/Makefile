install-scarb-burn:
	scarb install --git https://github.com/m-kus/scarb-burn --rev f01a5164576e29c002098ab397fb015808a4fb7b scarb-burn

bench-proof: build-prover
	mkdir -p target/bench
	../stwo_cairo_prover/target/release/prove_from_compiled_program \
		--compiled_program ../stwo_cairo_prover/test_data/test_prove_verify_all_opcode_components/compiled.json \
		--proof_path target/bench/proof.serde.json \
		--proof-format cairo-serde \
		--verify

build-prover:
	cd ../stwo_cairo_prover \
		&& RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo build --release --bin prove_from_compiled_program

bench:
	scarb build \
		--package stwo_cairo_verifier \
		--features qm31_opcode \
        --target-kinds executable
	scarb execute \
		--no-build \
		--package stwo_cairo_verifier \
		--arguments-file target/bench/proof.serde.json \
		--print-resource-usage \
		--output none

profile:
	scarb build \
		--package stwo_cairo_verifier \
		--features qm31_opcode \
        --target-kinds lib
	scarb burn \
		--no-build \
		--package stwo_cairo_verifier \
		--arguments-file target/bench/proof.serde.json \
		--output-file target/bench/profile.pb.gz \
		--output-type pprof \
		--open-in-browser
