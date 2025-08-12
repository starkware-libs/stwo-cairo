use stwo_cairo_air::RelationUsesDict;
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::channel::Channel;

pub trait ClaimTrait<T> {
    fn mix_into(self: @T, ref channel: Channel);
    fn log_sizes(self: @T) -> TreeArray<Span<u32>>;
    fn accumulate_relation_uses(self: @T, ref relation_uses: RelationUsesDict) {
        panic!()
    }
}
