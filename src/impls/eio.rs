use crate::U256;
use core::mem::size_of;
use eio::{FromBytes, ToBytes};

impl ToBytes<{ size_of::<U256>() }> for U256 {
    fn to_be_bytes(self) -> [u8; size_of::<U256>()] {
        self.to_be_bytes()
    }

    fn to_le_bytes(self) -> [u8; size_of::<U256>()] {
        self.to_le_bytes()
    }
}

impl FromBytes<{ size_of::<U256>() }> for U256 {
    fn from_be_bytes(bytes: [u8; size_of::<U256>()]) -> Self {
        Self::from_be_bytes(bytes)
    }

    fn from_le_bytes(bytes: [u8; size_of::<U256>()]) -> Self {
        Self::from_le_bytes(bytes)
    }
}
