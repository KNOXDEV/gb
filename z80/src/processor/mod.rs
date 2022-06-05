
type Register = u8;
type WideRegister = u16;

struct Processor {
    // general purpose registers
    a: Register,
    b: Register,
    c: Register,
    d: Register,
    e: Register,
    h: Register,
    l: Register,

    // so called f register, but for flags. from highest to lowest bit:
    // zero, subtraction, half carry, and carry flag. The lowest four bits are not used.
    flags: Register,

    // somewhat important registers
    program_counter: WideRegister, // initialized to 0x0100 at startup
    stack_pointer: WideRegister // initialized to 0xFFFE at startup, but 0xE000 would probably be more useful
}