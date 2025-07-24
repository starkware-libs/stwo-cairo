install-scarb-burn:
	scarb install --git https://github.com/m-kus/scarb-burn --rev f01a5164576e29c002098ab397fb015808a4fb7b scarb-burn

bench-artifacts:
	cd crates/cairo_verifier/tests/programs/bench \
	&& rm -rf target/execute \
	&& scarb --profile release execute \
		--package bench_program \
		--target bootloader \
		--output cairo-pie \
	&& stwo-bootloader \
		--pie target/execute/bench_program/execution1/cairo_pie.zip \
		--output-path artifacts/

bench-proof: build-adapted-prover
	mkdir -p target/bench
	../stwo_cairo_prover/target/release/adapted_stwo \
		--priv_json crates/cairo_verifier/tests/programs/bench/artifacts/priv.json \
		--pub_json crates/cairo_verifier/tests/programs/bench/artifacts/pub.json \
		--params_json crates/cairo_verifier/tests/prover_params.json \
		--proof_path target/bench/proof.serde.json \
		--proof-format cairo-serde \
		--verify

build-adapted-prover:
	cd ../stwo_cairo_prover \
		&& RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo build --release --bin adapted_stwo

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
