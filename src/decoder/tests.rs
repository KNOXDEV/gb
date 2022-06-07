use crate::decoder::commands::FlagCondition;
use crate::decoder::decode_flag_check;

#[test]
fn flag_condition_decode() {
    assert_eq!(decode_flag_check(0x18), FlagCondition::NoCheck);
    assert_eq!(decode_flag_check(0x20), FlagCondition::NotZero);
    assert_eq!(decode_flag_check(0x28), FlagCondition::Zero);
    assert_eq!(decode_flag_check(0x30), FlagCondition::NotCarry);
    assert_eq!(decode_flag_check(0x38), FlagCondition::Carry);

    assert_eq!(decode_flag_check(0xC9), FlagCondition::NoCheck);
    assert_eq!(decode_flag_check(0xC0), FlagCondition::NotZero);
    assert_eq!(decode_flag_check(0xC8), FlagCondition::Zero);
    assert_eq!(decode_flag_check(0xD0), FlagCondition::NotCarry);
    assert_eq!(decode_flag_check(0xD8), FlagCondition::Carry);

    assert_eq!(decode_flag_check(0xC3), FlagCondition::NoCheck);
    assert_eq!(decode_flag_check(0xC2), FlagCondition::NotZero);
    assert_eq!(decode_flag_check(0xCA), FlagCondition::Zero);
    assert_eq!(decode_flag_check(0xD2), FlagCondition::NotCarry);
    assert_eq!(decode_flag_check(0xDA), FlagCondition::Carry);
    assert_eq!(decode_flag_check(0xE9), FlagCondition::NoCheck);

    assert_eq!(decode_flag_check(0xCD), FlagCondition::NoCheck);
    assert_eq!(decode_flag_check(0xC4), FlagCondition::NotZero);
    assert_eq!(decode_flag_check(0xCC), FlagCondition::Zero);
    assert_eq!(decode_flag_check(0xD4), FlagCondition::NotCarry);
    assert_eq!(decode_flag_check(0xDC), FlagCondition::Carry);
}
