use crate::decoder::tables::tablebuilder::TableBuilder;

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
enum WideRegister {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}

#[derive(Copy, Clone)]
enum MemoryOperand {
    RegisterValue(Register),
    RegisterReference(Register),
    WideRegisterValue(WideRegister),
    WideRegisterReference(WideRegister),
    ImmediateValue,
    ImmediateSignedStackPointerOffset,
    ImmediateSignedProgramCounterOffset,
    ImmediateReference,
    WideImmediateReference,
}

const PRIMARY_REGISTER_SEQUENCE: [MemoryOperand; 8] = [
    MemoryOperand::RegisterValue(Register::B),
    MemoryOperand::RegisterValue(Register::C),
    MemoryOperand::RegisterValue(Register::D),
    MemoryOperand::RegisterValue(Register::E),
    MemoryOperand::RegisterValue(Register::H),
    MemoryOperand::RegisterValue(Register::L),
    MemoryOperand::WideRegisterReference(WideRegister::HL),
    MemoryOperand::RegisterValue(Register::A),
];

const PRIMARY_WIDE_REGISTER_SEQUENCE: [MemoryOperand; 4] = [
    MemoryOperand::WideRegisterValue(WideRegister::BC),
    MemoryOperand::WideRegisterValue(WideRegister::DE),
    MemoryOperand::WideRegisterValue(WideRegister::HL),
    MemoryOperand::WideRegisterValue(WideRegister::SP),
];

const PRIMARY_WIDE_REGISTER_REFERENCE_SEQUENCE: [MemoryOperand; 4] = [
    MemoryOperand::WideRegisterReference(WideRegister::BC),
    MemoryOperand::WideRegisterReference(WideRegister::DE),
    MemoryOperand::WideRegisterReference(WideRegister::HL),
    MemoryOperand::WideRegisterReference(WideRegister::HL),
];

const DESTINATION_OPERAND_TABLE: [MemoryOperand; 256] =
    TableBuilder::new(MemoryOperand::ImmediateValue)
        // the first half of the table basically follows this rule
        .bitmask_map_sequence(0b10111000, 0x0, 0x8, PRIMARY_REGISTER_SEQUENCE)
        // wide register exception
        .bitmask_map_sequence(0b11110101, 0x01, 0x10, PRIMARY_WIDE_REGISTER_SEQUENCE)
        // adds to HL, loads to A
        .bitmask_map_sequence(
            0b11001111,
            0x09,
            0x1,
            [
                MemoryOperand::WideRegisterValue(WideRegister::HL),
                MemoryOperand::RegisterValue(Register::A),
            ],
        )
        // loads to wide references
        .bitmask_map_sequence(
            0b11111111,
            0x02,
            0x10,
            PRIMARY_WIDE_REGISTER_REFERENCE_SEQUENCE,
        )
        // adds against A
        .bitmask_map(0b11111000, 0x80, MemoryOperand::RegisterValue(Register::A))
        // immediate jumps and rotates on A
        .bitmask_map_sequence(
            0b11000111,
            0x00,
            0x7,
            [
                MemoryOperand::ImmediateSignedProgramCounterOffset,
                MemoryOperand::RegisterValue(Register::A),
            ],
        )
        // what follows is the last quarter of the table
        // mostly immediate jumps
        .bitmask_map(0b11100000, 0xCB, MemoryOperand::WideImmediateReference)
        // weird immediate add to A
        .bitmask_map(0b11111111, 0xC6, MemoryOperand::RegisterValue(Register::A))
        // immediate high loads
        .bitmask_map_sequence(
            0b11111111,
            0xE0,
            0x10,
            [
                MemoryOperand::ImmediateReference,
                MemoryOperand::RegisterValue(Register::A),
            ],
        )
        // c high loads
        .bitmask_map_sequence(
            0b11111111,
            0xE2,
            0x10,
            [
                MemoryOperand::RegisterReference(Register::C),
                MemoryOperand::RegisterValue(Register::A),
            ],
        )
        // add and load, stack pointer
        .bitmask_map_sequence(
            0b11111111,
            0xE8,
            0x10,
            [
                MemoryOperand::WideRegisterValue(WideRegister::SP),
                MemoryOperand::WideRegisterValue(WideRegister::HL),
            ],
        )
        // jp and loads on HL
        .bitmask_map_sequence(
            0b11111111,
            0xE9,
            0x10,
            [
                MemoryOperand::WideRegisterReference(WideRegister::HL),
                MemoryOperand::WideRegisterValue(WideRegister::HL),
            ],
        )
        // immediate wide loads
        .bitmask_map_sequence(
            0b11111111,
            0xEA,
            0x10,
            [
                MemoryOperand::WideImmediateReference,
                MemoryOperand::RegisterValue(Register::A),
            ],
        )
        .build();

const PREFIXED_DESTINATION_OPERAND_TABLE: [MemoryOperand; 256] =
    TableBuilder::new(MemoryOperand::ImmediateValue)
        // the entire table follows one rule
        .bitmask_map_sequence(0b00000111, 0x0, 0x1, PRIMARY_REGISTER_SEQUENCE)
        .build();

// TODO:
// source operand table
// prefixed source operand table
// flag check table, although potentially a rule would be better
