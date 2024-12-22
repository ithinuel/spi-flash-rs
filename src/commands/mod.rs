use core::ops::Deref;

/// 24 bits address.
///
/// Any value if the MSB byte of the `u32` will be ignored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address24Bits(pub u32);
impl Address24Bits {
    pub fn to_le_bytes(self) -> [u8; 3] {
        let [lsb, csb, msb, _] = self.0.to_le_bytes();
        [lsb, csb, msb]
    }
}

/// 32 bits address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address32Bits(pub u32);
impl Deref for Address32Bits {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub mod spansion;
