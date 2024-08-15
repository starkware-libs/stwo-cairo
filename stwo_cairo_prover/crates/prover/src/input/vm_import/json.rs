use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicInput {
    pub layout: String,
    pub rc_min: u64,
    pub rc_max: u64,
    pub n_steps: u64,
    pub memory_segments: BTreeMap<String, Segment>,
    pub public_memory: Vec<PublicMemEntry>,
    pub dynamic_params: Option<()>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Segment {
    pub begin_addr: u64,
    pub stop_ptr: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicMemEntry {
    pub address: u64,
    pub value: FeltValue,
    pub page: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivateInput {
    pub trace_path: String,
    pub memory_path: String,
    pub pedersen: Vec<PedersenData>,
    pub range_check: Vec<RangeCheckData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PedersenData {
    pub index: u64,
    pub x: FeltValue,
    pub y: FeltValue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RangeCheckData {
    pub index: u64,
    pub value: FeltValue,
}

#[derive(Clone, Debug)]
pub struct FeltValue([u8; 32]);

impl Serialize for FeltValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert the [u8; 32] to a hexadecimal string
        let hex_string = format!("0x{}", hex::encode(self.0));
        serializer.serialize_str(&hex_string)
    }
}

impl<'de> Deserialize<'de> for FeltValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex_string = String::deserialize(deserializer)?;

        // Remove the "0x" prefix if present
        let hex_str = hex_string.strip_prefix("0x").unwrap_or(&hex_string);
        let hex_str = format!("{:0>64}", hex_str);

        // Convert the hexadecimal string back into a [u8; 32]
        let mut bytes = [0u8; 32];
        hex::decode_to_slice(hex_str, &mut bytes).map_err(serde::de::Error::custom)?;

        Ok(FeltValue(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_felt_value_serde() {
        let felt_value = FeltValue([0x12; 32]);
        let json = sonic_rs::to_string(&felt_value).unwrap();
        assert_eq!(
            json,
            r#""0x1212121212121212121212121212121212121212121212121212121212121212""#
        );

        let deserialized: FeltValue = sonic_rs::from_str(&json).unwrap();
        assert_eq!(felt_value.0, deserialized.0);
    }
}
