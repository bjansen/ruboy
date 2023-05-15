#![allow(non_snake_case)]

use ruboy;
use ruboy::cpu;
use ruboy::memory::Mmu;
use ruboy::opcodes::Register16Id::{BC, DE, HL, SP};

use crate::common::build_cartridge;

mod common;

macro_rules! arithmetic_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (opcode, reg, expected) = $value;

            let cartridge = build_cartridge(vec![
                0x01, 0x22, 0x11, // LD BC, $1122
                0x11, 0x44, 0x33, // LD DE, $3344
                0x21, 0x66, 0x55, // LD HL, $5566
                opcode, // ADD x, y
                0x10, 0x00, // STOP
            ]);

            let mut cpu = cpu::init_cpu();
            let mut mmu = Mmu::new(cartridge);

            cpu.run(&mut mmu);

            assert_eq!(expected, cpu.regs.get(reg));
        }
    )*
    }
}

arithmetic_tests! {
    test_ADD_HL_BC: (0x09, HL, 0x6688),
    test_ADD_HL_DE: (0x19, HL, 0x88AA),
    test_ADD_HL_HL: (0x29, HL, 0xAACC),
    test_ADD_HL_SP: (0x39, HL, 0x5564),
}

#[test]
fn test_ADD_SPn() {
    let cartridge = build_cartridge(vec![
        0xE8, 0xAA, // ADD SP, $AA
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(0xFFA8, cpu.regs.sp);
}

arithmetic_tests! {
    test_INC_BC: (0x03, BC, 0x1123),
    test_INC_DE: (0x13, DE, 0x3345),
    test_INC_HL: (0x23, HL, 0x5567),
    test_INC_SP: (0x33, SP, 0xFFFF),
}

arithmetic_tests! {
    test_DEC_BC: (0x0B, BC, 0x1121),
    test_DEC_DE: (0x1B, DE, 0x3343),
    test_DEC_HL: (0x2B, HL, 0x5565),
    test_DEC_SP: (0x3B, SP, 0xFFFD),
}
