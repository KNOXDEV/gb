mod commands;

use crate::decoder::commands::{DestinationOperand, Register, SourceOperand, WideRegister};
use commands::Command;

struct Decoder<I: Iterator<Item = u8>> {
    iter: I,
}

// pass in a three bit pattern and select one of 8 common destinations
fn match_destination(bits: u8) -> DestinationOperand {
    match bits & 0b111 {
        0x0 => DestinationOperand::RegisterLocation(Register::B),
        0x1 => DestinationOperand::RegisterLocation(Register::C),
        0x2 => DestinationOperand::RegisterLocation(Register::D),
        0x3 => DestinationOperand::RegisterLocation(Register::E),
        0x4 => DestinationOperand::RegisterLocation(Register::H),
        0x5 => DestinationOperand::RegisterLocation(Register::L),
        0x6 => DestinationOperand::WideRegisterReference,
        0x7 => DestinationOperand::RegisterLocation(Register::A),
        _ => panic!("unsupported destination bits"),
    }
}

// pass in a three bit pattern and select one of 8 common sources
fn match_source(bits: u8) -> SourceOperand {
    match bits & 0b111 {
        0x0 => SourceOperand::RegisterValue(Register::B),
        0x1 => SourceOperand::RegisterValue(Register::C),
        0x2 => SourceOperand::RegisterValue(Register::D),
        0x3 => SourceOperand::RegisterValue(Register::E),
        0x4 => SourceOperand::RegisterValue(Register::H),
        0x5 => SourceOperand::RegisterValue(Register::L),
        0x6 => SourceOperand::WideRegisterReference,
        0x7 => SourceOperand::RegisterValue(Register::A),
        _ => panic!("invalid source bits"),
    }
}

fn match_alu(opcode: u8) -> Command {
    let source = match_source(opcode);
    match opcode >> 3 & 0b111 {
        0x0 => Command::Add(source),
        0x1 => Command::AddCarry(source),
        0x2 => Command::Sub(source),
        0x3 => Command::SubCarry(source),
        0x4 => Command::And(source),
        0x5 => Command::Xor(source),
        0x6 => Command::Or(source),
        0x7 => Command::Compare(source),
        _ => panic!("invalid ALU bits"),
    }
}

// two bits of an opcode will commonly determine a wide register destination
fn match_wide_destination(bits: u8) -> DestinationOperand {
    match bits & 0b11 {
        0x0 => DestinationOperand::WideRegisterLocation(WideRegister::BC),
        0x1 => DestinationOperand::WideRegisterLocation(WideRegister::DE),
        0x2 => DestinationOperand::WideRegisterLocation(WideRegister::HL),
        0x3 => DestinationOperand::WideRegisterLocation(WideRegister::SP),
        _ => panic!("invalid wide destination bits"),
    }
}

impl<I: Iterator<Item = u8>> Iterator for Decoder<I> {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        let opcode = self.iter.next()?;
        Some(
            // the top two bits will determine if we are in the load section, alu, or other
            match opcode >> 6 & 0b11 {
                // load makes up a quarter of the entire table, fortunately
                0b01 => Command::Load(
                    // notice we only use bits 4, 5, and 6 of the opcode to determine the source
                    // in other words, 0x0 and 0x8 both map to Register B, and so on
                    match_destination(opcode >> 3),
                    // notice we only use the bottom three bits of the opcode to determine the source
                    // in other words, 0x0 and 0x8 both map to Register B, and so on
                    match_source(opcode),
                ),
                // the ALU section makes up another quarter of the table
                0b10 => match_alu(opcode),
                // the first quarter of the table should further switch on the last 4 bits
                0b00 => match opcode & 0xF {
                    // immediate loads to wide registers
                    0x1 => Command::Load(
                        match_wide_destination(opcode >> 4),
                        SourceOperand::WideImmediateValue(u16::from_le_bytes([
                            self.iter.next()?,
                            self.iter.next()?,
                        ])),
                    ),
                    // loads to wide references
                    0x2 => Command::Load(),
                    0x3 => Command::Increment(match_wide_destination(opcode >> 4)),
                    0x4 => Command::Increment(match_destination(opcode >> 3)),
                },
                // the last quarter of the table is a bunch of junk
                0b11 => match opcode {
                    0x76 => Command::Halt,
                },
                _ => panic!("invalid opcode"),
            },
        )
    }
}
