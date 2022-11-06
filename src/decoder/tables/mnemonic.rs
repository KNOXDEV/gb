use crate::decoder::commands::Mnemonic;
use crate::decoder::tables::tablebuilder::TableBuilder;

pub(crate) const MNEMONIC_TABLE: [Mnemonic; 256] = TableBuilder::new(Mnemonic::NOP)
    // top quarter
    // 12 loads (4 to be overwritten later)
    .bitmask_map(0b11000011, 0x2, Mnemonic::LD)
    // 8 increments and 8 decrements
    .bitmask_map_sequence(0b11000111, 0x4, 0x1, [Mnemonic::INC, Mnemonic::DEC])
    // 4 more increments and 4 more decrements
    .bitmask_map_sequence(0b11001111, 0x3, 0x8, [Mnemonic::INC, Mnemonic::DEC])
    // 5 relative jumps, (3 to be overwritten later)
    .bitmask_map(0b11000111, 0x0, Mnemonic::JR)
    // 4 more loads
    .bitmask_map_sequence(0b11001111, 0x1, 0x8, [Mnemonic::LD, Mnemonic::ADD])
    // 2 rotates, a shift, and decimal
    .bitmask_map_sequence(
        0b11111111,
        0x7,
        0x10,
        [Mnemonic::RLC, Mnemonic::RL, Mnemonic::DAA, Mnemonic::SCF],
    )
    // 2 right rotates and 2 compares
    .bitmask_map_sequence(
        0b11111111,
        0xF,
        0x10,
        [Mnemonic::RRC, Mnemonic::RR, Mnemonic::CPL, Mnemonic::CCF],
    )
    // 2 incrementing loads and 2 decrementing loads
    .bitmask_map_sequence(0b11110111, 0x22, 0x10, [Mnemonic::LDI, Mnemonic::LDD])
    // 2 special instructions
    .bitmask_map_sequence(0b11111111, 0x0, 0x10, [Mnemonic::NOP, Mnemonic::STOP])
    // 1 last special load
    .bitmask_map(0xb11111111, 0x8, Mnemonic::LD)
    // ===================
    // second quarter
    // 63 loads
    .bitmask_map(0b11000000, 0x40, Mnemonic::LD)
    // one special halt instruction
    .bitmask_map(0b11111111, 0x76, Mnemonic::HALT)
    // ===================
    // third quarter (just 64 ALU instructions)
    .bitmask_map_sequence(
        0b11111000,
        0x80,
        0x8,
        [
            Mnemonic::ADD,
            Mnemonic::ADC,
            Mnemonic::SUB,
            Mnemonic::SBC,
            Mnemonic::AND,
            Mnemonic::XOR,
            Mnemonic::OR,
            Mnemonic::CP,
        ],
    )
    // =================
    // fourth quarter
    // 4 returns, 4 jumps, and 4 calls
    .bitmask_map_sequence(
        0b11100111,
        0xC0,
        0x2,
        [Mnemonic::RET, Mnemonic::JP, Mnemonic::CALL],
    )
    // 8 restarts
    .bitmask_map(0b11000111, 0xC7, Mnemonic::RST)
    // 4 pops and 4 pushes
    .bitmask_map_sequence(0b11001111, 0xC1, 0x4, [Mnemonic::POP, Mnemonic::PUSH])
    // 7 more loads (1 to be overwritten later)
    .bitmask_map(0b11100101, 0xE0, Mnemonic::LD)
    // 8 alu operations
    .bitmask_map_sequence(
        0b11111111,
        0xC6,
        0x8,
        [
            Mnemonic::ADD,
            Mnemonic::ADC,
            Mnemonic::SUB,
            Mnemonic::SBC,
            Mnemonic::AND,
            Mnemonic::XOR,
            Mnemonic::OR,
            Mnemonic::CP,
        ],
    )
    // 2 returns, 1 jump, 1 load
    .bitmask_map_sequence(
        0b11111111,
        0xC9,
        0x10,
        [Mnemonic::RET, Mnemonic::RETI, Mnemonic::JP, Mnemonic::LD],
    )
    // 1 jump and 1 prefix
    .bitmask_map_sequence(0b11111111, 0xC3, 0x8, [Mnemonic::JP, Mnemonic::PREFIX])
    // 2 interrupts
    .bitmask_map_sequence(0b11111111, 0xF3, 0x8, [Mnemonic::DI, Mnemonic::EI])
    // 1 weird add
    .bitmask_map(0b11111111, 0xE8, Mnemonic::ADD)
    // 1 call
    .bitmask_map(0b11111111, 0xCD, Mnemonic::CALL)
    .build();

pub(crate) const PREFIXED_MNEMONIC_TABLE: [Mnemonic; 256] = TableBuilder::new(Mnemonic::NOP)
    // first quarter
    .bitmask_map_sequence(
        0b11111000,
        0x0,
        0x8,
        [
            Mnemonic::RLC,
            Mnemonic::RRC,
            Mnemonic::RL,
            Mnemonic::RR,
            Mnemonic::SLA,
            Mnemonic::SRA,
            Mnemonic::SWAP,
            Mnemonic::SRL,
        ],
    )
    // the rest of it lol
    .bitmask_map_sequence(
        0b11000000,
        0x040,
        0x40,
        [Mnemonic::BIT, Mnemonic::RES, Mnemonic::SET],
    )
    .build();
