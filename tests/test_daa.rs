#![allow(non_snake_case)]

use std::ops::Range;

use ruboy;
use ruboy::cpu;
use ruboy::memory::Mmu;
use ruboy::opcodes::RegisterId::A;

use crate::common::build_cartridge;

mod common;

macro_rules! daa_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (a, n, c, h, expected_a, expected_c) = $value;

            let cartridge = build_cartridge(vec![
                0x27, // DAA
                0x10, 0x00 // STOP
            ]);

            let mut cpu = cpu::init_cpu();
            let mut mmu = Mmu::new(cartridge);

            cpu.regs.flags.n = n;
            cpu.regs.flags.h = h;
            cpu.regs.flags.c = c;
            cpu.regs[A] = a;

            cpu.run(&mut mmu);

            assert_eq!(expected_a, cpu.regs[A]);
            assert_eq!(expected_c, cpu.regs.flags.c);
        }
    )*
    }
}

daa_tests! {
    test_DAA_00_after_add: (0x00, false, false, false, 0x00, false),
    test_DAA_09_after_add: (0x09, false, false, false, 0x09, false),
    test_DAA_10_after_add: (0x10, false, false, false, 0x10, false),
    test_DAA_19_after_add: (0x19, false, false, false, 0x19, false),
    test_DAA_90_after_add: (0x90, false, false, false, 0x90, false),
    test_DAA_99_after_add: (0x99, false, false, false, 0x99, false),

    test_DAA_0A_after_add: (0x0A, false, false, false, 0x10, false),
    test_DAA_0B_after_add: (0x0B, false, false, false, 0x11, false),
    test_DAA_8A_after_add: (0x8A, false, false, false, 0x90, false),
    test_DAA_8F_after_add: (0x8F, false, false, false, 0x95, false),

    test_DAA_40_after_add: (0x40, false, false, true, 0x46, false),
    test_DAA_43_after_add: (0x43, false, false, true, 0x49, false),

    test_DAA_A0_after_add: (0xA0, false, false, false, 0x00, true),
    test_DAA_F9_after_add: (0xF9, false, false, false, 0x59, true),

    test_DAA_9A_after_add: (0x9A, false, false, false, 0x00, true),
    test_DAA_FF_after_add: (0xFF, false, false, false, 0x65, true),

    test_DAA_A1_after_add: (0xA1, false, false, true, 0x07, true),
    test_DAA_F3_after_add: (0xF3, false, false, true, 0x59, true),

    test_DAA_07_after_add: (0x07, false, true, false, 0x67, true),
    test_DAA_29_after_add: (0x29, false, true, false, 0x89, true),

    test_DAA_1A_after_add: (0x1A, false, true, false, 0x80, true),
    test_DAA_2F_after_add: (0x2F, false, true, false, 0x95, true),

    test_DAA_05_after_add: (0x05, false, true, true, 0x6B, true),
    test_DAA_33_after_add: (0x33, false, true, true, 0x99, true),
}

daa_tests! {
    test_DAA_00_after_sub: (0x00, true, false, false, 0x00, false),
    test_DAA_99_after_sub: (0x99, true, false, false, 0x99, false),

    test_DAA_06_after_sub: (0x06, true, false, true, 0x00, false),
    test_DAA_8F_after_sub: (0x8F, true, false, true, 0x89, false),

    test_DAA_70_after_sub: (0x70, true, true, false, 0x10, true),
    test_DAA_F9_after_sub: (0xF9, true, true, false, 0x99, true),

    test_DAA_66_after_sub: (0x66, true, true, true, 0x00, true),
    test_DAA_FF_after_sub: (0xFF, true, true, true, 0x99, true),
}

#[test]
fn test_DAA_after_add() {
    assert_daa(false, false, 0x0..0x9, false, 0x0..0x9, 0x00, false);
    assert_daa(false, false, 0x0..0x8, false, 0xA..0xF, 0x06, false);
    assert_daa(false, false, 0x0..0x9, true, 0x0..0x3, 0x06, false);
    assert_daa(false, false, 0xA..0xF, false, 0x0..0x9, 0x60, true);
    assert_daa(false, false, 0x9..0xF, false, 0xA..0xF, 0x66, true);
    assert_daa(false, false, 0xA..0xF, true, 0x0..0x3, 0x66, true);
    assert_daa(false, true, 0x0..0x2, false, 0x0..0x9, 0x60, true);
    assert_daa(false, true, 0x0..0x2, false, 0xA..0xF, 0x66, true);
    assert_daa(false, true, 0x0..0x3, true, 0x0..0x3, 0x66, true);
}

#[test]
fn test_DAA_after_sub() {
    assert_daa(true, false, 0x0..0x9, false, 0x0..0x9, 0x00, false);
    assert_daa(true, false, 0x0..0x8, true, 0x6..0xF, 0xFA, false);
    assert_daa(true, true, 0x7..0xF, false, 0x0..0x9, 0xA0, true);
    assert_daa(true, true, 0x6..0xF, true, 0x6..0xF, 0x9A, true);
}

fn assert_daa(n: bool, cy: bool, hi_bits: Range<u8>, h: bool, lo_bits: Range<u8>, expected_added: u8, expected_cy: bool) {
    for hi in hi_bits {
        for lo in lo_bits.clone() {
            let cartridge = build_cartridge(vec![
                0x27, // DAA
                0x10, 0x00, // STOP
            ]);

            let mut cpu = cpu::init_cpu();
            let mut mmu = Mmu::new(cartridge);

            let a = (hi << 4) + lo;

            cpu.regs.flags.n = n;
            cpu.regs.flags.h = h;
            cpu.regs.flags.c = cy;
            cpu.regs[A] = a;

            cpu.run(&mut mmu);

            assert_eq!(a.wrapping_add(expected_added), cpu.regs[A]);
            assert_eq!(expected_cy, cpu.regs.flags.c);
        }
    }
}