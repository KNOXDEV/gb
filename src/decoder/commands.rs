type MemoryAddress = u16;
type AddressOffset = i8;
type BitLabel = u8; // between 0-7

#[derive(Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(Debug)]
pub enum WideRegister {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}

#[derive(Debug)]
pub enum FlagCondition {
    NoCheck,
    NotZero,
    Zero,
    NotCarry,
    Carry,
}

// an operand indicating where an output can be stored
#[derive(Debug)]
pub enum DestinationOperand {
    // an immediate 16 bit location in memory
    WideImmediateReference(MemoryAddress),
    // a particular (8 bit) register to save to
    RegisterLocation(Register),
    // a particular wide (16 bit) register to save to
    WideRegisterLocation(WideRegister),
    // these references are ALWAYS stored in HL
    WideRegisterReference,
}

// an operand indicating an input value
#[derive(Debug)]
pub enum SourceOperand {
    // an immediate 8 bit value
    ImmediateValue(u8),
    // an immediate 16 bit value
    WideImmediateValue(u16),
    // an immediate 16 bit location in memory
    WideImmediateReference(MemoryAddress),
    // the value in a particular register
    RegisterValue(Register),
    // these references are ALWAYS stored in HL
    WideRegisterReference,
    // a signed offset away from the stack pointer
    StackPointerOffset(AddressOffset),
}

// this represents the highest level of abstraction that the emulator will run
// NOTE: these have different timings depending on which register is addressed
#[derive(Debug)]
pub enum Command {
    // loads
    Load(DestinationOperand, SourceOperand),
    LoadDecrement(DestinationOperand, SourceOperand),
    LoadIncrement(DestinationOperand, SourceOperand),
    Push(SourceOperand),
    Pop(DestinationOperand),
    // ALU
    // most 8-bit arithmetic is done on A, while for 16-bit we use HL, so for those operations, we omit an operand
    Add(SourceOperand),
    AddCarry(SourceOperand),
    Sub(SourceOperand),
    SubCarry(SourceOperand),
    And(SourceOperand),
    Or(SourceOperand),
    Xor(SourceOperand),
    Compare(SourceOperand),
    Increment(DestinationOperand),
    Decrement(DestinationOperand),
    // MISC
    Swap(DestinationOperand),
    DecimalAdjust,
    Complement,
    ComplementCarry,
    SetCarry,
    NoOperation,
    Halt,
    Stop,
    DisableInterrupts,
    EnableInterrupts,
    // rotates / shifts
    RotateLeftCarry(DestinationOperand),
    RotateLeft(DestinationOperand),
    RotateRightCarry(DestinationOperand),
    RotateRight(DestinationOperand),
    ShiftLeftCarry(DestinationOperand),
    ShiftRightCarry(DestinationOperand),
    ShiftRightLogical(DestinationOperand),
    // bit manipulation
    BitTest(DestinationOperand, BitLabel),
    BitSet(DestinationOperand, BitLabel),
    BitReset(DestinationOperand, BitLabel),
    // jumps, calls, returns
    Jump(DestinationOperand, FlagCondition),
    JumpAdd(AddressOffset, FlagCondition),
    Call(DestinationOperand, FlagCondition),
    Restart(AddressOffset),
    Return(FlagCondition),
    ReturnEnableInterrupts,
}
