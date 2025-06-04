install:
	cargo install \
		--git https://github.com/m-kus/scarb-burn \
		--rev 126cda26572e6b5957c48ebfe444247b58aa62bc \
		scarb-burn

burn:
	scarb burn \
		--arguments-file tests/data/proof.json \
		--output-file flamegraph.svg \
		--open-in-browser

pprof:
	scarb burn \
		--output-type pprof \
		--output-file profile.pb.gz \
		--arguments-file tests/data/proof.json \
		--open-in-browser
