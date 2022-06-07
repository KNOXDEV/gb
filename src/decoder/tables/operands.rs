use crate::decoder::commands::{MemoryOperand, Register, WideRegister};
use crate::decoder::tables::tablebuilder::TableBuilder;

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

const SECONDARY_WIDE_REGISTER_SEQUENCE: [MemoryOperand; 4] = [
    MemoryOperand::WideRegisterValue(WideRegister::BC),
    MemoryOperand::WideRegisterValue(WideRegister::DE),
    MemoryOperand::WideRegisterValue(WideRegister::HL),
    MemoryOperand::WideRegisterValue(WideRegister::AF),
];

const PRIMARY_WIDE_REGISTER_REFERENCE_SEQUENCE: [MemoryOperand; 4] = [
    MemoryOperand::WideRegisterReference(WideRegister::BC),
    MemoryOperand::WideRegisterReference(WideRegister::DE),
    MemoryOperand::WideRegisterReference(WideRegister::HL),
    MemoryOperand::WideRegisterReference(WideRegister::HL),
];

const DESTINATION_MEMORY_OPERAND_TABLE: [MemoryOperand; 256] =
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
        // mostly immediate jumps or calls
        .bitmask_map(0b11100000, 0xC0, MemoryOperand::WideImmediateReference)
        // pops (but not pushes)
        .bitmask_map_sequence(0b11111111, 0xC1, 0x10, SECONDARY_WIDE_REGISTER_SEQUENCE)
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

const PREFIXED_DESTINATION_MEMORY_OPERAND_TABLE: [MemoryOperand; 256] =
    TableBuilder::new(MemoryOperand::ImmediateValue)
        // the entire table follows one rule
        .bitmask_map_sequence(0b00000111, 0x0, 0x1, PRIMARY_REGISTER_SEQUENCE)
        .build();

// this table stores the source memory operand for operations that have one
const SOURCE_MEMORY_OPERAND_TABLE: [MemoryOperand; 256] =
    TableBuilder::new(MemoryOperand::ImmediateValue)
        // this is the pattern for almost the entire table, including all loads and ALU mnemonics
        .bitmask_map_sequence(0b00000111, 0x0, 0x1, PRIMARY_REGISTER_SEQUENCE)
        // wide immediate loads
        .bitmask_map(0b11001111, 0x01, MemoryOperand::WideImmediateValue)
        // loads of A into wide register references
        .bitmask_map(0b11001111, 0x02, MemoryOperand::RegisterValue(Register::A))
        // loads into A from wide register references
        .bitmask_map_sequence(
            0b11111111,
            0x0A,
            0x10,
            PRIMARY_WIDE_REGISTER_REFERENCE_SEQUENCE,
        )
        // immediate loads
        .bitmask_map(0b11000111, 0x06, MemoryOperand::ImmediateValue)
        // load stack pointer
        .bitmask_map(
            0b11111111,
            0x08,
            MemoryOperand::WideRegisterValue(WideRegister::SP),
        )
        // wide adds to HL
        .bitmask_map_sequence(0b11111111, 0x09, 0x10, PRIMARY_WIDE_REGISTER_SEQUENCE)
        // bottom half
        // pushes (but not pops)
        .bitmask_map_sequence(0b11111111, 0xC5, 0x10, SECONDARY_WIDE_REGISTER_SEQUENCE)
        // immediate ALU
        .bitmask_map(0b11000111, 0xC6, MemoryOperand::ImmediateValue)
        // loads involving c high reference
        .bitmask_map_sequence(
            0b11111111,
            0xE2,
            0x10,
            [
                MemoryOperand::RegisterValue(Register::A),
                MemoryOperand::RegisterReference(Register::C),
            ],
        )
        // loads involving wide immediate references
        .bitmask_map_sequence(
            0b11111111,
            0xEA,
            0x10,
            [
                MemoryOperand::RegisterValue(Register::A),
                MemoryOperand::WideImmediateReference,
            ],
        )
        // stack pointer loads
        .bitmask_map_sequence(
            0b11111111,
            0xF8,
            0x1,
            [
                MemoryOperand::ImmediateSignedStackPointerOffset,
                MemoryOperand::WideRegisterValue(WideRegister::HL),
            ],
        )
        // weird relative stack pointer add
        .bitmask_map(
            0b11111111,
            0xE8,
            MemoryOperand::ImmediateSignedStackPointerOffset,
        )
        .build();
