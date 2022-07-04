type WideRegister = u16;

struct Processor {
    // general purpose registers
    af: WideRegister,
    bc: WideRegister,
    de: WideRegister,
    hl: WideRegister,

    // somewhat important registers
    program_counter: WideRegister,
    stack_pointer: WideRegister,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            // these initial values are recovered from PanDocs
            af: 0x01B0,
            bc: 0x0013,
            de: 0x00D8,
            hl: 0x014D,
            program_counter: 0x0100,
            stack_pointer: 0xFFFE,
        }
    }

    // interpret the next instruction and execute
    // this takes in the immutable program data (ROM) as well as the mutable memory space (RAM)
    pub fn tick(&mut self) {
        // decode the opcode at the current program counter
    }
}
