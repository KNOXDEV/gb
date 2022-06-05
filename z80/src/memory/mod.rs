use crate::memory::MemoryAddress::{LowROM, SwitchableROM};

enum MemoryAddress {
    InteruptEnableRegister,
    // FFFF
    HighInternalRAM(u8),
    // FF80
    HighUnusable(u8),
    // FF4C
    IOPorts(u8),
    // FF00
    LowUnusable(u8),
    // FEA0
    SpriteAttribMemory(u8),
    // FE00
    EchoLowInternalRAM(u16),
    // E000
    LowInternalRAM(u16),
    // C000
    SwitchableRAM(u16),
    // A000
    VideoRAM(u16),
    // 8000
    SwitchableROM(u16),
    // 4000
    LowROM(u16),
    // 0000
}

fn decode_address(address: u16) -> MemoryAddress {
    match address {
        0x0000..=0x3FFF => LowROM(address),
        0x4000..=0x7FFF => SwitchableROM(address - 0x4000),
    }
}