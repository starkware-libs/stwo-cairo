# ⚡ S-two Cairo ⚡

Prove Cairo programs with the blazing-fast [S-Two prover](https://github.com/starkware-libs/stwo), powered by the cryptographic breakthrough of [Circle STARKs](https://eprint.iacr.org/2024/278).

* [Prerequisites](#prerequisites)
* [`scarb-prove`](#scarb-prove)

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install/)
- [Scarb](https://docs.swmansion.com/scarb/download.html)
  - The recommended installation method is using [asdf](https://asdf-vm.com/)
  - Make sure to use version 2.10.0 and onwards, and preferably the latest nightly version.
  
    To use the latest nightly version, run:
    
    ```
    asdf set -u scarb latest:nightly
    ```

## Installation

This repository now focuses on the prover and verifier crates under `stwo_cairo_prover/` and `stwo_cairo_verifier/`. The former `cairo-prove` CLI has been removed. The equivalent utility is now provided in `proving-utils`: https://github.com/starkware-libs/proving-utils

## `scarb prove`

As of Scarb version 2.10.0, `scarb prove` can be used instead of manually building and running `stwo-cairo`.

However, `scarb prove` is still a work in progress, and using `stwo-cairo` directly is preferable for now.
