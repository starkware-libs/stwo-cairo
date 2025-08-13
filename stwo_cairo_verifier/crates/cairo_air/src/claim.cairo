use stwo_cairo_air::RelationUsesDict;
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::channel::Channel;

/// Trait that defines the functionality required by a "claim",
/// where a "claim" is an object that holds public information about
/// one or multiple components whose trace needs to be verified.
pub trait ClaimTrait<T> {
    /// Commit to the data.
    fn mix_into(self: @T, ref channel: Channel);
    /// Returns the sizes of the components associated with the claim.
    fn log_sizes(self: @T) -> TreeArray<Span<u32>>;
    /// Record the lookups used by the components associated with the claim.
    fn accumulate_relation_uses(
        self: @T, ref relation_uses: RelationUsesDict,
    ) {
        panic!("Cannot use `accumulate_relation_uses` without implementing it.")
    }
}
