use crate::decoder::commands::{FlagCondition, Mnemonic};
use crate::decoder::tables::mnemonic::{MNEMONIC_TABLE, PREFIXED_MNEMONIC_TABLE};

mod commands;
mod tables;
#[cfg(test)]
mod tests;

// flag check rule. its simpler to just have this instead of keeping a table for it
#[inline]
const fn decode_flag_check(opcode: u8) -> FlagCondition {
    match opcode {
        // catch non-cases
        0x18 | 0xC3 | 0xE9 | 0xCD | 0xC9 => FlagCondition::NoCheck,
        _ => match (opcode >> 3) & 0b11 {
            0 => FlagCondition::NotZero,
            1 => FlagCondition::Zero,
            2 => FlagCondition::NotCarry,
            3 => FlagCondition::Carry,
            _ => FlagCondition::NoCheck,
        },
    }
}

// returns the bit to select based on the opcode,
// used for bit-wise instructions in the prefix table
// its one line so a full table would be overkill
#[inline]
const fn decode_bit_selection(opcode: u8) -> u8 {
    (opcode >> 3) & 0b111
}

// returns address for restart opcodes,
// its just math so a full table would be overkill
#[inline]
const fn decode_restart_address(opcode: u8) -> u16 {
    (opcode - 0xC7) as u16
}

#[inline]
const fn decode_mnemonic(opcode: u8) -> Mnemonic {
    // MNEMONIC_TABLE[opcode as usize]
    todo!();
}
