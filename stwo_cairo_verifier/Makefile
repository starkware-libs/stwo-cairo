CAIRO_EXECUTE_REV=5f7cb0d13edee28b95086f958b6020cdbbbad61b
SCARB_BURN_REV=f01a5164576e29c002098ab397fb015808a4fb7b

install-scarb-burn:
	cargo install --git https://github.com/m-kus/scarb-burn --rev $(SCARB_BURN_REV) scarb-burn

install-cairo-execute:
	cargo install --git https://github.com/starkware-libs/cairo --rev $(CAIRO_EXECUTE_REV) cairo-execute

install-corelib:
	mkdir -p vendor
	rm -rf vendor/cairo
	git clone --single-branch --branch main https://github.com/starkware-libs/cairo vendor/cairo
	(cd vendor/cairo && git checkout $(CAIRO_EXECUTE_REV))
	ln -s "$(CURDIR)/vendor/cairo/corelib" corelib

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
	scarb execute \
		--package stwo_cairo_verifier \
		--arguments-file target/bench/proof.serde.json \
		--print-resource-usage \
		--output none

profile:
	cairo-execute \
		--run-profiler scoped \
		--args-file target/bench/proof.serde.json \
		--output-path target/stwo_cairo_verifier.pie.zip \
		--executable stwo_cairo_verifier::main \
		--layout all_cairo_stwo \
		--ignore-warnings \
		crates/cairo_verifier/blake_cairo_project > target/stwo_cairo_verifier.profile.txt
	
flamegraph:
	scarb-burn \
		--profile-file target/stwo_cairo_verifier.profile.txt \
		--output-file target/stwo_cairo_verifier.flamegraph.svg \
		--output-type flamegraph \
		--open-in-browser

pprof:
	scarb-burn \
		--profile-file target/stwo_cairo_verifier.profile.txt \
		--output-file target/stwo_cairo_verifier.pprof.gz \
		--output-type pprof \
		--open-in-browser
