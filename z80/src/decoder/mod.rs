
enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

enum WideRegister {
    AF,
    BC,
    DE,
    HL,

    PC,
    SP
}

enum Operand {
    ImmediateValue(u8),
    ImmediateReference(u8),
    WideImmediateReference(u16),
    RegisterValue(Register),
    WideRegisterReference(WideRegister),
    StackPointerOffset(i8),
}

// this represents the highest level of abstraction that the emulator will run
enum Command {
    // loads
    Load(Operand, Operand),
    LoadDecrement(Operand, Operand),
    LoadIncrement(Operand, Operand),
    Push(Operand),
    Pop(Operand),
    // ALU
    // most 8-bit arithmetic is done on A, while for 16-bit we use HL, so for those operations, we omit an operand
    Add(Operand),
    AddCarry(Operand),
    Sub(Operand),
    SubCarry(Operand),
    And(Operand),
    Or(Operand),
    Xor(Operand),
    Compare(Operand),
    Increment(Operand),
    Decrement(Operand),
    // MISC

}

// there are commands with 1 argument, 2 arguments, and no arguments
// there are commands with short opcodes and wide opcodes
// at the end of the day, I want a function that takes in an iterator of u8's and produces
// fully qualified commands to run

// this enum captures the semantic meaning of an opcode without its parameters
enum Opcode {
    // put immediate 8-bit value into a register
    LoadRegFromImm8(Register),
    // put value in one register into another
    LoadRegFromReg(Register, Register),
    // put value addressed by one register into another
    LoadRegFromAddress(Register, Register),
    // put two byte immediate value into register
    LoadRegFromReg(Register, Register),

}

// one-byte commands
const OPCODE_LOOKUP: [Opcode; 256] = (0..256).map(|i| decode(i)).collect();

// two-byte wide commands that start with 0xCB
const WIDE_OPCODE_LOOKUP: [Opcode; 256] = (0..256).map(|i| wide_decode(i)).collect();


const fn decode(opcode: u8) -> Opcode {
    let high_nibble = opcode >> 4;
    let low_nibble = opcode & 0xF;

    // LoadRegFromImm8


    match opcode {

    }
}

const fn wide_decode(opcode: u8) -> Command {

}