use either::Either;

use super::{Address24Bits, Address32Bits};

/// Standard SPI flash command opcodes.
///
/// These are taken from the Winbond W25Q16JV and Spansion S125FL128S datasheet, but most are
/// widely applicable. If SFDP is supported, it is used to discover
/// the relevant erase opcodes and sizes.
///
/// .
#[derive(Copy, Clone, Debug, num_enum::IntoPrimitive)]
#[repr(u8)]
pub enum CommandOpCode {
    // Core instruction set.
    // These commands are almost universally available.
    WriteEnable = 0x06,
    WriteDisable = 0x04,
    ReadData = 0x03,
    PageProgram = 0x02,
    ReadStatusRegister1 = 0x05,
    WriteStatusRegister1 = 0x01,

    // Standard instruction set.
    // These commands are typically available.
    ReadJEDECID = 0x9F,
    FastRead = 0x0B,
    Powerdown = 0xB9,
    /// This is also the Read Electronic Signature command.
    ReleasePowerdown = 0xAB,
    ReadDeviceID = 0x90,
    ChipErase = 0xC7,

    // Extended instruction set.
    // These commands may be available.
    ReadUniqueID = 0x4B,
    ReadSFDPRegister = 0x5A,
    ReadStatusRegister2 = 0x35,
    ReadStatusRegister3 = 0x15,
    ReadFlagStatusRegister = 0x70,
    WriteStatusRegister2 = 0x31,
    WriteStatusRegister3 = 0x11,
    WriteEnableVolatile = 0x50,
    EnableReset = 0x66,
    Reset = 0x99,
    SoftwareReset = 0xF0,
    ProgramSuspend = 0x75,
    ProgramResume = 0x7A,

    // Erase instructions.
    // The size affected by each erase operation can vary.
    // Typical sizes are 4kB for sector erase, 32kB for block erase 1,
    // and 64kB for block erase 2.
    SectorErase = 0x20,
    BlockErase1 = 0x52,
    BlockErase2 = 0xD8,

    // Security/lock related instructions.
    EraseSecurityRegisters = 0x44,
    ProgramSecurityRegisters = 0x42,
    ReadSecurityRegisters = 0x48,
    IndividualBlockLock = 0x36,
    IndividualBlockUnlock = 0x39,
    ReadBlockLock = 0x3D,
    GlobalBlockLock = 0x7E,
    GlobalBlockUnlock = 0x98,

    // DSPI
    ReadDualOut = 0x3B,
    ReadDualIO = 0xBB,

    // QSPI
    ReadQuadOut = 0x6B,
    ReadQuadIO = 0xEB,
}

#[derive(Debug, Clone, Copy)]
pub enum Command {
    // Read device identification
    ReadDeviceID,
    ReadJEDECID,
    ReleasePowerdown,

    // Register Access
    ReadStatusRegister1,
    WriteEnable,
    WriteDisable,
    ReadData(Address24Bits),
    PageProgram(Address24Bits),
    WriteStatusRegister1,

    FastRead(Address24Bits),
    Powerdown,
    ChipErase,

    ReadUniqueID,
    ReadSFDPRegister(Address24Bits),
    ReadStatusRegister2,
    ReadStatusRegister3,
    ReadFlagStatusRegister,
    WriteStatusRegister2,
    WriteStatusRegister3,
    WriteEnableVolatile,
    EnableReset,
    Reset,

    ReadDualOut(Address24Bits),
    ReadQuadOut(Address24Bits),
    ReadDualIO(Address24Bits),
    ReadQuadIO(Address24Bits),
}
impl Command {
    pub(crate) fn to_array(&self) -> arrayvec::ArrayVec<u8, 4> {
        use either::Either::*;
        match *self {
            Command::ReadStatusRegister1 => Left(CommandOpCode::ReadStatusRegister1),
            Command::WriteStatusRegister1 => Left(CommandOpCode::WriteStatusRegister1),
            Command::ReadUniqueID => Left(CommandOpCode::ReadUniqueID),
            Command::ReadJEDECID => Left(CommandOpCode::ReadJEDECID),
            Command::ReadSFDPRegister(addr) => Right((CommandOpCode::ReadSFDPRegister, addr)),
            Command::ReadStatusRegister2 => Left(CommandOpCode::ReadStatusRegister2),
            Command::ReadStatusRegister3 => Left(CommandOpCode::ReadStatusRegister3),
            Command::WriteStatusRegister2 => Left(CommandOpCode::WriteStatusRegister2),
            Command::WriteStatusRegister3 => Left(CommandOpCode::WriteStatusRegister3),
            Command::WriteEnableVolatile => Left(CommandOpCode::WriteEnableVolatile),
            Command::WriteEnable => Left(CommandOpCode::WriteEnable),
            Command::WriteDisable => Left(CommandOpCode::WriteDisable),
            Command::FastRead(addr) => Right((CommandOpCode::FastRead, addr)),
            Command::PageProgram(addr) => Right((CommandOpCode::PageProgram, addr)),
            Command::ReadData(addr) => Right((CommandOpCode::ReadData, addr)),
            Command::ReadDualOut(addr) => Right((CommandOpCode::ReadDualOut, addr)),
            Command::ReadQuadOut(addr) => Right((CommandOpCode::ReadQuadOut, addr)),
            Command::ReadDualIO(addr) => Right((CommandOpCode::ReadDualIO, addr)),
            Command::ReadQuadIO(addr) => Right((CommandOpCode::ReadQuadIO, addr)),
            _ => {
                todo!()
            }
        }
        .map_left(|c| [u8::from(c)].into_iter())
        .map_right(|(c, addr)| {
            Iterator::chain([u8::from(c)].into_iter(), addr.to_le_bytes().into_iter())
        })
        .into_iter()
        .collect()
    }

    pub(crate) fn len(&self) -> usize {
        match self {
            Command::ReadUniqueID => 1,
            Command::ReadJEDECID => 1,
            Command::ReadStatusRegister1 => 1,
            Command::ReadStatusRegister2 => 1,
            Command::ReadStatusRegister3 => 1,
            Command::WriteEnable => 1,
            Command::WriteDisable => 1,
            Command::PageProgram(_) => 4,
            Command::ReadData(_) => 4,
            Command::ReadDualOut(_) => 4,
            Command::ReadQuadOut(_) => 4,
            Command::ReadDualIO(_) => 4,
            Command::ReadQuadIO(_) => 4,
            _ => todo!(),
        }
    }

    pub fn try_from_byte(
        op_code: u8,
        addr: Option<Either<Address24Bits, Address32Bits>>,
    ) -> crate::Result<Self> {
        todo!()
    }
}
