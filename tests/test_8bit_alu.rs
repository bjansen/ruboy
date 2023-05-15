#![allow(non_snake_case)]

use ruboy;
use ruboy::cpu;
use ruboy::cpu::Flag;
use ruboy::memory::Mmu;
use ruboy::opcodes::RegisterId::{A, B, C, D, E, H, L};

use crate::common::build_cartridge;

mod common;

macro_rules! add_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (opcode, expected) = $value;

            let cartridge = build_cartridge(vec![
                0x3E, 0x01, // LD A, $01
                0x01, 0x22, 0x11, // LD BC, $1122
                0x11, 0x44, 0x33, // LD DE, $3344
                0x21, 0x66, 0x55, // LD HL, $5566
                opcode, 0x69, // ADD x, y
                0x10, 0x00, // STOP
                0x10, 0x00, // STOP
            ]);

            let mut cpu = cpu::init_cpu();
            let mut mmu = Mmu::new(cartridge);

            cpu.regs.flags.c = true;

            cpu.run(&mut mmu);

            assert_eq!(expected, cpu.regs[A]);
        }
    )*
    }
}

add_tests! {
    test_ADD_AA: (0x87, 0x02),
    test_ADD_AB: (0x80, 0x12),
    test_ADD_AC: (0x81, 0x23),
    test_ADD_AD: (0x82, 0x34),
    test_ADD_AE: (0x83, 0x45),
    test_ADD_AH: (0x84, 0x56),
    test_ADD_AL: (0x85, 0x67),
}

#[test]
fn test_ADD_AHL() {
    let cartridge = build_cartridge(vec![
        0x3E, 0x12, // LD A, $12
        0x26, 0xC0, // LD H, $C0
        0x2E, 0x00, // LD L, $00
        0x86, // ADD A, (HL)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);
    mmu[0xC000] = 8;

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs[A], 0x1A);
}

#[test]
fn test_ADD_An() {
    let cartridge = build_cartridge(vec![
        0x3E, 0x12, // LD A, $12
        0xC6, 0x09, // ADD A, $09
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs[A], 0x1B);
    assert_eq!(false, cpu.regs[Flag::Z]);
    assert_eq!(false, cpu.regs[Flag::N]);
}

#[test]
fn test_ADD_flags() {
    let cartridge = build_cartridge(vec![
        0x3E, 0xFF, // LD A, $12
        0xC6, 0x01, // ADD A, $01
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(true, cpu.regs[Flag::Z]);
    assert_eq!(false, cpu.regs[Flag::N]);
    // TODO test flags H and C
}

add_tests! {
    test_ADC_AA: (0x8F, 0x03),
    test_ADC_AB: (0x88, 0x13),
    test_ADC_AC: (0x89, 0x24),
    test_ADC_AD: (0x8A, 0x35),
    test_ADC_AE: (0x8B, 0x46),
    test_ADC_AH: (0x8C, 0x57),
    test_ADC_AL: (0x8D, 0x68),
}

#[test]
fn test_ADC_AHL() {
    let cartridge = build_cartridge(vec![
        0x3E, 0x12, // LD A, $12
        0x26, 0xC0, // LD H, $C0
        0x2E, 0x00, // LD L, $00
        0x8E, // ADC A, (HL)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);
    mmu[0xC000] = 8;
    cpu.regs.flags.c = true;

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs[A], 0x1B);
}

#[test]
fn test_ADC_An() {
    let cartridge = build_cartridge(vec![
        0x3E, 0x12, // LD A, $12
        0xCE, 0x09, // ADC A, $09
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);
    cpu.regs.flags.c = true;

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs[A], 0x1C);
}

add_tests! {
    test_SUB_A: (0x97, 0x00),
    test_SUB_B: (0x90, 0xF0),
    test_SUB_C: (0x91, 0xDF),
    test_SUB_D: (0x92, 0xCE),
    test_SUB_E: (0x93, 0xBD),
    test_SUB_H: (0x94, 0xAC),
    test_SUB_L: (0x95, 0x9B),
}

#[test]
fn test_SUB_AHL() {
    let cartridge = build_cartridge(vec![
        0x3E, 0x12, // LD A, $12
        0x26, 0xC0, // LD H, $C0
        0x2E, 0x00, // LD L, $00
        0x96, // SUB A, (HL)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);
    mmu[0xC000] = 8;
    cpu.regs.flags.c = true;

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs[A], 0x0A);
}

#[test]
fn test_SUB_An() {
    let cartridge = build_cartridge(vec![
        0x3E, 0x12, // LD A, $12
        0xD6, 0x09, // SUB A, $09
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);
    cpu.regs.flags.c = true;

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs[A], 0x09);
}

add_tests! {
    test_SBC_A: (0x9F, 0xFF),
    test_SBC_B: (0x98, 0xEF),
    test_SBC_C: (0x99, 0xDE),
    test_SBC_D: (0x9A, 0xCD),
    test_SBC_E: (0x9B, 0xBC),
    test_SBC_H: (0x9C, 0xAB),
    test_SBC_L: (0x9D, 0x9A),
}

#[test]
fn test_SBC_AHL() {
    let cartridge = build_cartridge(vec![
        0x3E, 0x12, // LD A, $12
        0x26, 0xC0, // LD H, $C0
        0x2E, 0x00, // LD L, $00
        0x9E, // SBC A, (HL)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);
    mmu[0xC000] = 8;
    cpu.regs.flags.c = true;

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs[A], 0x09);
}

#[test]
fn test_SBC_An() {
    let cartridge = build_cartridge(vec![
        0x3E, 0x12, // LD A, $12
        0xDE, 0x09, // SBC A, $09
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);
    cpu.regs.flags.c = true;

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs[A], 0x08);
}

macro_rules! logic_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (opcode, expected) = $value;

            let cartridge = build_cartridge(vec![
                0x3E, 0b10101010, // LD A, $AA
                0x01, 0x22, 0x11, // LD BC, $1122
                0x11, 0x44, 0x33, // LD DE, $3344
                0x21, 0x66, 0x55, // LD HL, $5566
                opcode, // <op> A,x
                0x10, 0x00, // STOP
                0x10, 0x00, // STOP
            ]);

            let mut cpu = cpu::init_cpu();
            let mut mmu = Mmu::new(cartridge);

            mmu[0x5566] = 0xFF;

            cpu.run(&mut mmu);

            assert_eq!(expected, cpu.regs[A]);
        }
    )*
    }
}

logic_tests! {
    test_AND_AA: (0xA7, 0b10101010),
    test_AND_AB: (0xA0, 0b00000000),
    test_AND_AC: (0xA1, 0b00100010),
    test_AND_AD: (0xA2, 0b00100010),
    test_AND_AE: (0xA3, 0b00000000),
    test_AND_AH: (0xA4, 0b00000000),
    test_AND_AL: (0xA5, 0b00100010),
    test_AND_AHL: (0xA6, 0b10101010),
    test_AND_An: (0xE6, 0b00000000),
}

logic_tests! {
    test_OR_AA: (0xB7, 0b10101010),
    test_OR_AB: (0xB0, 0b10111011),
    test_OR_AC: (0xB1, 0b10101010),
    test_OR_AD: (0xB2, 0b10111011),
    test_OR_AE: (0xB3, 0b11101110),
    test_OR_AH: (0xB4, 0b11111111),
    test_OR_AL: (0xB5, 0b11101110),
    test_OR_AHL: (0xB6, 0b11111111),
    test_OR_An: (0xF6, 0b10111010),
}

logic_tests! {
    test_XOR_AA: (0xAF, 0b00000000),
    test_XOR_AB: (0xA8, 0b10111011),
    test_XOR_AC: (0xA9, 0b10001000),
    test_XOR_AD: (0xAA, 0b10011001),
    test_XOR_AE: (0xAB, 0b11101110),
    test_XOR_AH: (0xAC, 0b11111111),
    test_XOR_AL: (0xAD, 0b11001100),
    test_XOR_AHL: (0xAE, 0b01010101),
    test_XOR_An: (0xEE, 0b10111010),
}

macro_rules! cp_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (opcode, expected_z, expected_n, expected_h, expected_c) = $value;

            let cartridge = build_cartridge(vec![
                0x3E, 0b10101010, // LD A, $AA
                0x01, 0x22, 0x11, // LD BC, $1122
                0x11, 0x44, 0x33, // LD DE, $3344
                0x21, 0x66, 0x55, // LD HL, $5566
                opcode, // CP A,x
                0x10, 0x00, // STOP
                0x10, 0x00, // STOP
            ]);

            let mut cpu = cpu::init_cpu();
            let mut mmu = Mmu::new(cartridge);

            mmu[0x5566] = 0xFF;

            cpu.run(&mut mmu);

            assert_eq!(expected_z, cpu.regs.flags.z);
            assert_eq!(expected_n, cpu.regs.flags.n);
            assert_eq!(expected_h, cpu.regs.flags.h);
            assert_eq!(expected_c, cpu.regs.flags.c);
        }
    )*
    }
}

cp_tests! {
    test_CP_A: (0xBF, true, true, false, false),
    test_CP_B: (0xB8, false, true, false, false),
    test_CP_C: (0xB9, false, true, false, false),
    test_CP_D: (0xBA, false, true, false, false),
    test_CP_E: (0xBB, false, true, false, false),
    test_CP_H: (0xBC, false, true, false, false),
    test_CP_L: (0xBD, false, true, false, false),
    test_CP_HL: (0xBE, false, true, true, true),
    test_CP_n: (0xFE, false, true, false, false),
}

macro_rules! inc_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (opcode, reg, expected) = $value;

            let cartridge = build_cartridge(vec![
                0x3E, 0b10101010, // LD A, $AA
                0x01, 0x22, 0x11, // LD BC, $1122
                0x11, 0x44, 0x33, // LD DE, $3344
                0x21, 0x66, 0x55, // LD HL, $5566
                opcode, // INC reg
                0x10, 0x00, // STOP
            ]);

            let mut cpu = cpu::init_cpu();
            let mut mmu = Mmu::new(cartridge);

            cpu.run(&mut mmu);

            assert_eq!(expected, cpu.regs[reg]);
        }
    )*
    }
}

inc_tests! {
    test_INC_A: (0x3C, A, 0xAB),
    test_INC_B: (0x04, B, 0x12),
    test_INC_C: (0x0C, C, 0x23),
    test_INC_D: (0x14, D, 0x34),
    test_INC_E: (0x1C, E, 0x45),
    test_INC_H: (0x24, H, 0x56),
    test_INC_L: (0x2C, L, 0x67),
}

#[test]
fn test_INC_HL() {
    let cartridge = build_cartridge(vec![
        0x21, 0x66, 0x55, // LD HL, $5566
        0x34, // INC (HL)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    mmu[0x5566] = 0xFF;

    cpu.run(&mut mmu);

    assert_eq!(0x00, mmu[0x5566]);
}

inc_tests! {
    test_DEC_A: (0x3D, A, 0xA9),
    test_DEC_B: (0x05, B, 0x10),
    test_DEC_C: (0x0D, C, 0x21),
    test_DEC_D: (0x15, D, 0x32),
    test_DEC_E: (0x1D, E, 0x43),
    test_DEC_H: (0x25, H, 0x54),
    test_DEC_L: (0x2D, L, 0x65),
}

#[test]
fn test_DEC_HL() {
    let cartridge = build_cartridge(vec![
        0x21, 0x66, 0x55, // LD HL, $5566
        0x35, // DEC (HL)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    mmu[0x5566] = 0xFF;

    cpu.run(&mut mmu);

    assert_eq!(0xFE, mmu[0x5566]);
}
