use crate::cartridge::Cartridge;
use crate::memory::{ReadOnlyMemory, ReadWriteMemory};

struct RomOnlyCartridge {
    // 32KiB
    rom: [u8; 32 * 1024],
}

impl ReadOnlyMemory for RomOnlyCartridge {
    fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
}

impl ReadWriteMemory for RomOnlyCartridge {
    fn write(&mut self, addr: u16, value: u8) {
        self.rom[addr as usize] = value;
    }
}

impl Cartridge for RomOnlyCartridge {}
