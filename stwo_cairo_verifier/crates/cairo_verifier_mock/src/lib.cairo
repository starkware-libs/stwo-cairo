use stwo_cairo_air::{CairoProof, VerificationOutput, get_verification_output};

/// An executable (can be run using cairo-execute) of a mock verifier.
/// To build, run this from the root folder:
/// ```
///     cairo-execute stwo_cairo_verifier/ --executable stwo_cairo_verifier_mock::main --build-only
///     stwo_cairo_verifier/compiled_mock_verifier.json --allow-warnings
/// ```
///
/// Then to run it:
/// ```
///     cairo-execute --prebuilt path/to/compiled_mock_verifier.json --layout=all_cairo --args-file
///     path/to/mock_verifier_inputs.json
/// ```
/// Use `--cairo-pie-output path/to/output/pie.zip` to create a Cairo PIE for this run.
/// And/or use `--print-outputs` if you want the outputs to be streamed to stdout.
///
/// Note: the input JSON file should contain data that is deserialized to `CairoProof`. The data
/// should be given as an array of hex numbers (felt252s) as strings. For example: ["0x7", "0x80"].
#[executable]
fn main(proof: CairoProof) -> VerificationOutput {
    // In the real verifier, here is the actual verification. Here we skip it.

    get_verification_output(proof: @proof)
}
