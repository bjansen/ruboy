#![allow(non_snake_case)]

use ruboy;
use ruboy::cpu;
use ruboy::memory::Mmu;
use ruboy::opcodes::{Register16Id::{BC, DE, HL}};
use ruboy::opcodes::Register16Id::AF;

use crate::common::build_cartridge;

mod common;

macro_rules! push_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (opcode, expected) = $value;

            let cartridge = build_cartridge(vec![
                0x3E, 0x77, // LD A, $77
                0x01, 0x22, 0x11, // LD BC, $1122
                0x11, 0x44, 0x33, // LD DE, $3344
                0x21, 0x66, 0x55, // LD HL, $5566
                opcode, // PUSH xx
                0x10, 0x00, // STOP
            ]);

            let mut cpu = cpu::init_cpu();
            let mut mmu = Mmu::new(cartridge);

            cpu.run(&mut mmu);

            let [lo, hi] = u16::to_be_bytes(expected);

            assert_eq!(lo, mmu[0xFFFD]);
            assert_eq!(hi, mmu[0xFFFC]);
            assert_eq!(0xFFFC, cpu.regs.sp);
        }
    )*
    }
}

push_tests! {
    test_PUSH_AF: (0xF5, 0x7700),
    test_PUSH_BC: (0xC5, 0x1122),
    test_PUSH_DE: (0xD5, 0x3344),
    test_PUSH_HL: (0xE5, 0x5566),
}

macro_rules! pop_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (opcode, register) = $value;

            let cartridge = build_cartridge(vec![
                opcode, // POP xx
                0x10, 0x00, // STOP
            ]);

            let mut cpu = cpu::init_cpu();
            let mut mmu = Mmu::new(cartridge);

            mmu[0xFFFD] = 0x22;
            mmu[0xFFFC] = 0xF0;
            cpu.regs.sp = 0xFFFC;

            cpu.run(&mut mmu);

            assert_eq!(0x22F0, cpu.regs.get(register));
            assert_eq!(0xFFFE, cpu.regs.sp);
        }
    )*
    }
}

pop_tests! {
    test_POP_AF: (0xF1, AF),
    test_POP_BC: (0xC1, BC),
    test_POP_DE: (0xD1, DE),
    test_POP_HL: (0xE1, HL),
}
