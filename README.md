# ⚡ Stwo Cairo ⚡

Prove Cairo programs with the new [Stwo prover](https://github.com/starkware-libs/stwo), which is based on the [circle STARK](https://eprint.iacr.org/2024/278) protocol

* [Using Stwo to Prove Cairo Programs](#using-stwo-to-prove-cairo-programs)
* [Creating a Cairo Executable](#creating-a-cairo-executable)
  * [Prerequisites](#prerequisites)
  * [Defining an Executable Package](#defining-an-executable-package)
  * [Execution targets](#execution-targets)
  * [Input Format](#input-format)
  * [Output Format](#output-format)
  * [Limitations](#limitations)
    * [Gas](#gas)
    * [Syscalls](#syscalls)
    * [Padding Overhead](#padding-overhead)

## Disclaimer

🚧 Stwo is WIP and is not yet Sound 🚧

🚧 The Stwo prover and its Cairo adaptation are still being built, therefore API breaking changes might happen
often, so use it at your own risk. 🚧

## Using Stwo to Prove Cairo Programs

After executing a Cairo program one should be in the possession of four files:
* air_public_inputs.json
* air_private_inputs.json
* trace.bin
* memory.bin

With the paths to the trace and memory files appearing in `air_private_inputs`. After building stwo-cairo, you can run the `adapted_stwo` binary and obtain a Stwo proof:

```
./target/release/adapted_stwo \
--pub_json <path_to_air_public_input> \
--priv_json <path_to_air_private_input> \
--proof_path <path for proof output>
```

For best performance, run with `RUSTFLAGS="-C target-cpu=native -C opt-level=3" --features="std"`

In the next section we'll see how to generate the inputs to adapted stwo from a Cairo program.

# Creating a Cairo Executable

## Prerequisites

Install [Scarb](https://docs.swmansion.com/scarb/docs.html#installation), the Cairo package manager. The recommended installation is via [asdf](https://asdf-vm.com/).

Make sure to use Scarb 2.10.0 onwards, or alternatively the latest nightly. After going through the installation steps above, you can do this by running:

`asdf global scarb 2.10.0`

or

`asdf global scarb latest:nightly`

## Defining an Executable Package

Start a new Scarb project with `Scarb new <project_name>`, and add the following to your `Scarb.toml` file:

1. Specify that this package should compile to a Cairo executable by adding `[[target.executable]]` to your toml file (note that `lib` or `starknet-contract` targets cannot be executed in this way)  
2. Add the `cairo_execute="2.10.0"`	 plugin to your dependencies  
3. Disable gas usage (gas is only supported for `lib` or `starknet-contract` targets) by adding `enable-gas = false` under the `[cairo]` section in your toml.

Below we have an example of the manifest file of a simple executable

```
[package]
name = "test_execute"
version = "0.1.0"
edition = "2024_07"

[[target.executable]]

[cairo]
enable-gas = false

[dependencies]
cairo_execute = "2.10.0"
```

Now we can move on to the code itself. An executable project must have **exactly one function** annotated with the `#[executable]` attribute. Consider the following simple `lib.cairo` file of an executable project:

```
#[executable]
fn main(num: u8) -> u8 {
    num
}
```

You can now run:

```
scarb execute -p test_execute --print-program-output --arguments 5
```

Where `test_execute` is the name of the package with the executable target (as defined in our Scarb.toml manifest)

The above command runs our executable function within the `test-execute` package and prints the program's output segment, which contains a success bit followed by the Cairo Serde of main’s output or the panic reason in case of a panic.

## Execution targets

The `--target` flag allows specifying either a `standalone` target or `bootloader` target. Standalone means that the program will be executed as-is, and intended to be proven directly with Stwo. When we run with the bootloader target, the program’s execution is expected to be wrapped by the bootloader’s execution, which itself will be proven via Stwo.

When executing with `--target standalone` (the default if not specified) we get the four files which consist as the input for the `adapted_stwo` binary (`air_private_input.json`, `air_public_input.json`, `trace.bin`, `memory.bin`), while when executing with `--target bootloader`, the output is given in the CairoPie format (Position Indenpendent Execution).

In the meantime, `stwo-cairo` does not contain an API for executing the bootloader with the user's program as input, hence the only way to get a proof for a bootloader target is to take the generated CairoPie and send it to a third party like SHARP or [Atlantic](https://docs.herodotus.cloud/atlantic/introduction).

## Input format

The expected input with `--arguments` is a comma-separated list of integers. This list should correspond to the Cairo’s Serde of main’s arguments, for example:

| main’s signature | valid arguments example |
| :---- | :---- |
| `fn main(num: u8)` | 1 |
| `fn main(num1: u8, num2: u16)` | 1,2 |
| `fn main(num1: u8, tuple: (u16, u16))` | 1,2,3 |
| `fn main(num1: u8, num2: u256)` | 1,2,3 |
| `fn main(num1: u8, arr: Array<u8>)` | 1,2,1,2 |

See the [documentation](https://docs.starknet.io/architecture-and-concepts/smart-contracts/serialization-of-cairo-types/) for more information about Cairo’s Serde.

Note that when using `--arguments-file`, the expected input is an array of felts represented as hex string. For example, `1,2,3` in the above table becomes `[0x1, 0x2, 0x3]`.

## Output format

For standalone targets, the supported output is trace files (`air_public_input.json` & `air_private_input.json`)

For bootloader targets, the supported output is a Cairo pie

## Limitations

### Gas

Executables must be compiled with the `enable-gas = false` config in the manifest file. Gas tracking introduces a computation overhead and makes less sense outside the context of Starnet smart contracts.

### Syscalls

Syscalls are not supported in an executable target. Using syscalls, directly or via corelib functions that uses syscall (such as sha256, keccak, secp256k/r1 operations) will cause compilation to fail.

### Padding overhead

At the time of writing, the execution (\# of steps and \# of builtin application per builtin) with a `standalone` target is still padded to powers of 2, w.r.t to the ratios in the [all_cairo](https://github.com/lambdaclass/cairo-vm/blob/15bf79470cdd8eff29f41fc0a87143dce5499c7e/vm/src/types/instance_definitions/builtins_instance_def.rs#L157) layout. This will be removed in the future as Stwo does not rely on padding. Bootloader target executions are not padded.

