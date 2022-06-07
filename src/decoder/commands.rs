#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub enum WideRegister {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}

#[derive(Copy, Clone)]
pub enum MemoryOperand {
    RegisterValue(Register),
    RegisterReference(Register),
    WideRegisterValue(WideRegister),
    WideRegisterReference(WideRegister),
    ImmediateValue,
    ImmediateSignedStackPointerOffset,
    ImmediateSignedProgramCounterOffset,
    ImmediateReference,
    WideImmediateValue,
    WideImmediateReference,
}

#[derive(Debug)]
pub enum FlagCondition {
    NoCheck,
    NotZero,
    Zero,
    NotCarry,
    Carry,
}

#[derive(Debug, Copy, Clone)]
pub enum Mnemonic {
    // loads
    // we omit some special types of loads because they can be implemented the same
    LD,
    LDD,
    LDI,
    PUSH,
    POP,
    // alu
    ADD,
    ADC,
    SUB,
    SBC,
    AND,
    OR,
    XOR,
    CP,
    INC,
    DEC,
    // misc
    SWAP,
    DAA,
    CPL,
    CCF,
    SCF,
    NOP,
    HALT,
    STOP,
    DI,
    EI,
    PREFIX,
    // rotates and shifts
    // we omit the A-specific ones since they can be implemented the same way
    RLC,
    RL,
    RRC,
    RR,
    SLA,
    SRA,
    SRL,
    // bits
    BIT,
    SET,
    RES,
    // jumps, calls, restarts, returns
    JP,
    JR,
    CALL,
    RST,
    RET,
    RETI,
}
