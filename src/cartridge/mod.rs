use crate::memory::ReadWriteMemory;

mod rom_only;

// cartridges may respond to both reads and writes,
// so we require implementations to specify both
pub(crate) trait Cartridge: ReadWriteMemory {}
