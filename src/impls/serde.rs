use crate::U256;
use crate_serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

impl Serialize for U256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut slice = [0u8; 66];
        let bytes = self.to_be_bytes();
        impl_serde::serialize::serialize_uint(&mut slice, &bytes, serializer)
    }
}

impl<'de> Deserialize<'de> for U256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut bytes = [0u8; 32];
        let wrote = impl_serde::serialize::deserialize_check_len(
            deserializer,
            impl_serde::serialize::ExpectedLen::Between(0, &mut bytes),
        )?;
        let mut padded = [0u8; 32];
        padded[32 - wrote..].copy_from_slice(&bytes[..wrote]);

        Ok(U256::from_be_bytes(padded))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_serialization() {
        assert_eq!(
            serde_json::to_value(&U256::new(0x12345)).unwrap(),
            "0x12345",
        );
        assert_eq!(
            serde_json::from_str::<U256>("\"0x12345\"").unwrap(),
            U256::new(0x12345),
        );
    }
}
