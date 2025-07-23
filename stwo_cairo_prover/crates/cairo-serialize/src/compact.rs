pub trait CompactBinary {
    /// Serializes the object into a compact binary format.
    fn compact_serialize(&self, output: &mut Vec<u8>);

    /// Deserializes the object from a compact binary format.
    fn compact_deserialize(input: &[u8]) -> Self;
}
