mod commands;

use crate::decoder::commands::Command::{
    Add, DisableInterrupts, EnableInterrupts, Load, Return, ReturnEnableInterrupts,
};
use crate::decoder::commands::{
    Command, DestinationOperand, FlagCondition, Register, SourceOperand, WideRegister,
};

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
        0x6 => DestinationOperand::WideRegisterReference(WideRegister::HL),
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
        0x6 => SourceOperand::WideRegisterReference(WideRegister::HL),
        0x7 => SourceOperand::RegisterValue(Register::A),
        _ => panic!("invalid source bits"),
    }
}

fn match_alu(opcode: u8, source: SourceOperand) -> Command {
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

// two bits of an opcode will commonly determine a wide register
fn match_wide_register(bits: u8) -> WideRegister {
    match bits & 0b11 {
        0x0 => WideRegister::BC,
        0x1 => WideRegister::DE,
        0x2 => WideRegister::HL,
        0x3 => WideRegister::SP,
        _ => panic!("invalid wide register bits"),
    }
}

// two bits is enough to indicate a particular flag condition
fn match_flag_condition(bits: u8) -> FlagCondition {
    match bits & 0b11 {
        0b0 => FlagCondition::NotZero,
        0b1 => FlagCondition::Zero,
        0b10 => FlagCondition::NotCarry,
        0b11 => FlagCondition::Carry,
        _ => panic!("invalid jump add opcode"),
    }
}

// two bits of an opcode will sometimes determine a wide register in the context of a load
fn match_wide_register_reference(bits: u8) -> WideRegister {
    match bits & 0b11 {
        0x0 => WideRegister::BC,
        0x1 => WideRegister::DE,
        0x2 => WideRegister::HL,
        0x3 => WideRegister::HL,
        _ => panic!("invalid wide register reference bits"),
    }
}

// two bits to identify a particular wide register in the context of a stack operation
fn match_wide_register_stack(bits: u8) -> WideRegister {
    match bits & 0b11 {
        0x0 => WideRegister::BC,
        0x1 => WideRegister::DE,
        0x2 => WideRegister::HL,
        0x3 => WideRegister::AF,
        _ => panic!("invalid wide register stack bits"),
    }
}

impl<I: Iterator<Item = u8>> Decoder<I> {
    #[inline]
    fn get_immediate(&mut self) -> Option<u8> {
        self.iter.next()
    }

    #[inline]
    fn get_immediate_signed(&mut self) -> Option<i8> {
        Some(self.iter.next()? as i8)
    }

    #[inline]
    fn get_wide_immediate(&mut self) -> Option<u16> {
        Some(u16::from_le_bytes([self.iter.next()?, self.iter.next()?]))
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
                // special case: handle halt
                0b01 if opcode == 0x76 => Command::Halt,
                0b01 => Command::Load(
                    // notice we only use bits 4, 5, and 6 of the opcode to determine the source
                    // in other words, 0x0 and 0x8 both map to Register B, and so on
                    match_destination(opcode >> 3),
                    // notice we only use the bottom three bits of the opcode to determine the source
                    // in other words, 0x0 and 0x8 both map to Register B, and so on
                    match_source(opcode),
                ),
                // the ALU section makes up another quarter of the table
                0b10 => match_alu(opcode, match_source(opcode)),
                // the first quarter of the table should further switch on the last 4 bits
                0b00 => match opcode & 0xF {
                    // conditional relative jumps
                    // note this is not where 0X and 1X are handled,
                    // those are special cases handled in the else block
                    0x0 | 0x8 if opcode & 0b10000 != 0 => Command::JumpAdd(
                        self.get_immediate_signed()?,
                        match_flag_condition(opcode >> 3),
                    ),
                    // immediate loads to wide registers
                    0x1 => Command::Load(
                        DestinationOperand::WideRegisterLocation(match_wide_register(opcode >> 4)),
                        SourceOperand::WideImmediateValue(self.get_wide_immediate()?),
                    ),
                    // loads to wide references
                    0x2 => Command::Load(
                        DestinationOperand::WideRegisterReference(match_wide_register_reference(
                            opcode >> 4,
                        )),
                        SourceOperand::RegisterValue(Register::A),
                    ),
                    0x3 => Command::Increment(DestinationOperand::WideRegisterLocation(
                        match_wide_register(opcode >> 4),
                    )),
                    // increment and decrement
                    0x4 | 0xC => Command::Increment(match_destination(opcode >> 3)),
                    0x5 | 0xD => Command::Decrement(match_destination(opcode >> 3)),
                    // load an immediate 8bit value
                    0x6 | 0xE => Command::Load(
                        match_destination(opcode >> 3),
                        SourceOperand::ImmediateValue(self.get_immediate()?),
                    ),
                    // wide adds, which have a destination of HL instead of A
                    0x9 => Command::Add(SourceOperand::WideRegisterValue(match_wide_register(
                        opcode >> 4,
                    ))),
                    // weird reference loads into A idk
                    0xA => Command::Load(
                        DestinationOperand::RegisterLocation(Register::A),
                        SourceOperand::WideRegisterReference(match_wide_register_reference(
                            opcode >> 4,
                        )),
                    ),
                    // decrement a wide register
                    0xB => Command::Decrement(DestinationOperand::WideRegisterLocation(
                        match_wide_register(opcode >> 4),
                    )),
                    // no patterns here, these are the remaining special cases for the first quad
                    _ => match opcode {
                        0x00 => Command::NoOperation,
                        0x10 => Command::Stop,
                        0x08 => Command::Load(
                            DestinationOperand::WideImmediateReference(self.get_wide_immediate()?),
                            SourceOperand::WideRegisterValue(WideRegister::SP),
                        ),
                        0x18 => {
                            Command::JumpAdd(self.get_immediate_signed()?, FlagCondition::NoCheck)
                        }
                        0x07 => Command::RotateLeftCarry(DestinationOperand::RegisterLocation(
                            Register::A,
                        )),
                        0x17 => {
                            Command::RotateLeft(DestinationOperand::RegisterLocation(Register::A))
                        }
                        0x27 => Command::DecimalAdjust,
                        0x37 => Command::SetCarry,
                        0x0F => Command::RotateRightCarry(DestinationOperand::RegisterLocation(
                            Register::A,
                        )),
                        0x1F => {
                            Command::RotateRight(DestinationOperand::RegisterLocation(Register::A))
                        }
                        0x2F => Command::Complement,
                        0x3F => Command::ComplementCarry,
                        _ => panic!("invalid first quarter opcode"),
                    },
                },
                // the last quarter of the table is also a bunch of random stuff
                0b11 => match opcode & 0xF {
                    // pop instructions
                    0x1 => Command::Pop(DestinationOperand::WideRegisterLocation(
                        match_wide_register_stack(opcode >> 4),
                    )),
                    // push instructions
                    0x5 => Command::Push(SourceOperand::WideRegisterValue(
                        match_wide_register_stack(opcode >> 4),
                    )),
                    // immediate arithmetic
                    0x6 | 0xE => {
                        match_alu(opcode, SourceOperand::ImmediateValue(self.get_immediate()?))
                    }
                    // restarts
                    0x7 | 0xF => Command::Restart(opcode & 0b111000),
                    _ => {
                        // now split this quarter further into a top and bottom half
                        if opcode & 0b100000 == 0 {
                            // once again split by columns
                            match opcode & 0xF {
                                // conditional returns, jumps, and calls
                                0x0 | 0x8 => Command::Return(match_flag_condition(opcode >> 3)),
                                0x2 | 0xA => Command::Jump(
                                    DestinationOperand::WideImmediateReference(
                                        self.get_wide_immediate()?,
                                    ),
                                    match_flag_condition(opcode >> 3),
                                ),
                                0x4 | 0xC => Command::Call(
                                    DestinationOperand::WideImmediateReference(
                                        self.get_wide_immediate()?,
                                    ),
                                    match_flag_condition(opcode >> 3),
                                ),
                                // unconditional absolute jump
                                0x3 => Command::Jump(
                                    DestinationOperand::WideImmediateReference(
                                        self.get_wide_immediate()?,
                                    ),
                                    FlagCondition::NoCheck,
                                ),
                                // unconditional absolute call
                                0xD => Command::Call(
                                    DestinationOperand::WideImmediateReference(
                                        self.get_wide_immediate()?,
                                    ),
                                    FlagCondition::NoCheck,
                                ),
                                // unconditional returns
                                0x9 => {
                                    if opcode & 0x10 == 0 {
                                        Return(FlagCondition::NoCheck)
                                    } else {
                                        ReturnEnableInterrupts
                                    }
                                }
                                // prefix for larger codes
                                0xB => {
                                    todo!()
                                }
                                _ => panic!("invalid last quarter opcode"),
                            }
                        } else {
                            // no patterns, just special cases at this point
                            match opcode {
                                0xE0 => Load(
                                    DestinationOperand::ImmediateReference(self.get_immediate()?),
                                    SourceOperand::RegisterValue(Register::A),
                                ),
                                0xF0 => Load(
                                    DestinationOperand::RegisterReference(Register::A),
                                    SourceOperand::ImmediateReference(self.get_immediate()?),
                                ),
                                0xE2 => Load(
                                    DestinationOperand::RegisterReference(Register::C),
                                    SourceOperand::RegisterValue(Register::A),
                                ),
                                0xF2 => Load(
                                    DestinationOperand::RegisterReference(Register::A),
                                    SourceOperand::RegisterReference(Register::C),
                                ),
                                0xF3 => DisableInterrupts,
                                0xFB => EnableInterrupts,
                                0xE8 => todo!("add to the stack pointer"),
                                0xF8 => Load(
                                    DestinationOperand::WideRegisterLocation(WideRegister::HL),
                                    SourceOperand::StackPointerOffset(self.get_immediate_signed()?),
                                ),
                                0xE9 => Command::Jump(
                                    DestinationOperand::WideRegisterReference(WideRegister::HL),
                                    FlagCondition::NoCheck,
                                ),
                                0xF9 => Command::Load(
                                    DestinationOperand::WideRegisterLocation(WideRegister::SP),
                                    SourceOperand::WideRegisterValue(WideRegister::HL),
                                ),
                                0xEA => Command::Load(
                                    DestinationOperand::WideImmediateReference(
                                        self.get_wide_immediate()?,
                                    ),
                                    SourceOperand::RegisterValue(Register::A),
                                ),
                                0xFA => Command::Load(
                                    DestinationOperand::RegisterLocation(Register::A),
                                    SourceOperand::WideImmediateReference(
                                        self.get_wide_immediate()?,
                                    ),
                                ),
                                _ => panic!("invalid last eighth opcode"),
                            }
                        }
                    }
                },
                _ => panic!("invalid opcode"),
            },
        )
    }
}
