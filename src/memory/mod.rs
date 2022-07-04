use crate::cartridge::Cartridge;

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

pub trait ReadOnlyMemory {
    fn read(&self, addr: u16) -> u8;
    fn read_wide(&self, addr: u16) -> u16 {
        u16::from_le_bytes([self.read(addr), self.read(addr + 1)])
    }
}

pub trait ReadWriteMemory: ReadOnlyMemory {
    fn write(&mut self, addr: u16, value: u8);
    fn write_wide(&mut self, addr: u16, value: u16) {
        let bytes = value.to_le_bytes();
        self.write(addr, bytes[0]);
        self.write(addr + 1, bytes[1]);
    }
}

struct GameBoyMemory<T: Cartridge> {
    cartridge: T,
    // TODO: add other memory regions
}

// these should match depending on the region being read / written to
impl<T: Cartridge> ReadOnlyMemory for GameBoyMemory<T> {
    fn read(&self, addr: u16) -> u8 {
        todo!()
    }
}

impl<T: Cartridge> ReadWriteMemory for GameBoyMemory<T> {
    fn write(&mut self, addr: u16, value: u8) {
        todo!()
    }
}
