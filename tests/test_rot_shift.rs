#![allow(non_snake_case)]

use ruboy;
use ruboy::cpu;
use ruboy::cpu::Flag;
use ruboy::memory::Mmu;
use ruboy::opcodes::RegisterId::{A, B, C, D, E, H, L};

use crate::common::{assert_flags_eq, build_cartridge};

mod common;

#[test]
fn test_RLCA() {
    let cartridge = build_cartridge(vec![
        0x3E, 0b10101010, // LD A, value
        0x07, // RLCA
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs[A], 0b01010101);
    assert_flags_eq(&cpu, false, false, false, true);
}

#[test]
fn test_RLCA_0() {
    let cartridge = build_cartridge(vec![
        0x3E, 0b00000000, // LD A, value
        0x07, // RLCA
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs[A], 0b00000000);
    assert_flags_eq(&cpu, false, false, false, false);
}

#[test]
fn test_RLA() {
    let cartridge = build_cartridge(vec![
        0x3E, 0b00101010, // LD A, value
        0x17, // RLA
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.regs[Flag::C] = true;

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs[A], 0b01010101);
    assert_flags_eq(&cpu, false, false, false, false);
}

#[test]
fn test_RRCA() {
    let cartridge = build_cartridge(vec![
        0x3E, 0b11101010, // LD A, value
        0x0F, // RRCA
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs[A], 0b01110101);
    assert_flags_eq(&cpu, false, false, false, false);
}

#[test]
fn test_RRCA_0() {
    let cartridge = build_cartridge(vec![
        0x3E, 0b00000000, // LD A, value
        0x0F, // RRCA
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs[A], 0b00000000);
    assert_flags_eq(&cpu, false, false, false, false);
}

#[test]
fn test_RRA() {
    let cartridge = build_cartridge(vec![
        0x3E, 0b00101010, // LD A, value
        0x1F, // RRA
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.regs[Flag::C] = true;

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs[A], 0b10010101);
    assert_flags_eq(&cpu, false, false, false, false);
}

macro_rules! rotate_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (opcode, c, reg, expected, expected_z, expected_c) = $value;

            let cartridge = build_cartridge(vec![
                0x3E, 0xF0, // LD A, $F0
                0x06, 0xE1, // LD B, $E1
                0x0E, 0xD2, // LD C, $D2
                0x16, 0xC3, // LD D, $C3
                0x1E, 0xB4, // LD E, $B4
                0x26, 0xA5, // LD H, $A5
                0x2E, 0x96, // LD L, $96
                0xCB, opcode, // <op> x
                0x10, 0x00, // STOP
            ]);

            let mut cpu = cpu::init_cpu();
            let mut mmu = Mmu::new(cartridge);

            cpu.regs.flags.c = c;

            cpu.run(&mut mmu);

            assert_eq!(expected, cpu.regs[reg]);
            assert_eq!(expected_z, cpu.regs.flags.z);
            assert_eq!(expected_c, cpu.regs.flags.c);
        }
    )*
    }
}

rotate_tests! {
    test_RLC_A: (0x07, false, A, 0xE1, false, true),
    test_RLC_B: (0x00, false, B, 0xC3, false, true),
    test_RLC_C: (0x01, false, C, 0xA5, false, true),
    test_RLC_D: (0x02, false, D, 0x87, false, true),
    test_RLC_E: (0x03, false, E, 0x69, false, true),
    test_RLC_H: (0x04, false, H, 0x4B, false, true),
    test_RLC_L: (0x05, false, L, 0x2D, false, true),
}

#[test]
fn test_RLC_HL() {
    let cartridge = build_cartridge(vec![
        0x3E, 0xF0, // LD A, $F0
        0x06, 0xE1, // LD B, $E1
        0x0E, 0xD2, // LD C, $D2
        0x16, 0xC3, // LD D, $C3
        0x1E, 0xB4, // LD E, $B4
        0x26, 0xA5, // LD H, $A5
        0x2E, 0x96, // LD L, $96
        0xCB, 0x06, // RLC (HL)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    mmu[0xA596] = 0x0F;

    cpu.run(&mut mmu);

    assert_eq!(0x1E, mmu[0xA596]);
    assert_eq!(false, cpu.regs.flags.z);
    assert_eq!(false, cpu.regs.flags.c);
}

rotate_tests! {
    test_RL_A: (0x17, false, A, 0xE0, false, true),
    test_RL_B: (0x10, true, B, 0xC3, false, true),
    test_RL_C: (0x11, false, C, 0xA4, false, true),
    test_RL_D: (0x12, true, D, 0x87, false, true),
    test_RL_E: (0x13, false, E, 0x68, false, true),
    test_RL_H: (0x14, true, H, 0x4B, false, true),
    test_RL_L: (0x15, false, L, 0x2C, false, true),
}

#[test]
fn test_RL_HL() {
    let cartridge = build_cartridge(vec![
        0x3E, 0xF0, // LD A, $F0
        0x06, 0xE1, // LD B, $E1
        0x0E, 0xD2, // LD C, $D2
        0x16, 0xC3, // LD D, $C3
        0x1E, 0xB4, // LD E, $B4
        0x26, 0xA5, // LD H, $A5
        0x2E, 0x96, // LD L, $96
        0xCB, 0x16, // RL (HL)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    mmu[0xA596] = 0x0F;
    cpu.regs.flags.c = true;

    cpu.run(&mut mmu);

    assert_eq!(0x1F, mmu[0xA596]);
    assert_eq!(false, cpu.regs.flags.z);
    assert_eq!(false, cpu.regs.flags.c);
}

rotate_tests! {
    test_RRC_A: (0x0F, false, A, 0x78, false, false),
    test_RRC_B: (0x08, false, B, 0xF0, false, true),
    test_RRC_C: (0x09, false, C, 0x69, false, false),
    test_RRC_D: (0x0A, false, D, 0xE1, false, true),
    test_RRC_E: (0x0B, false, E, 0x5A, false, false),
    test_RRC_H: (0x0C, false, H, 0xD2, false, true),
    test_RRC_L: (0x0D, false, L, 0x4B, false, false),
}

#[test]
fn test_RRC_HL() {
    let cartridge = build_cartridge(vec![
        0x3E, 0xF0, // LD A, $F0
        0x06, 0xE1, // LD B, $E1
        0x0E, 0xD2, // LD C, $D2
        0x16, 0xC3, // LD D, $C3
        0x1E, 0xB4, // LD E, $B4
        0x26, 0xA5, // LD H, $A5
        0x2E, 0x96, // LD L, $96
        0xCB, 0x0E, // RRC (HL)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    mmu[0xA596] = 0x0F;
    cpu.regs.flags.c = true;

    cpu.run(&mut mmu);

    assert_eq!(0x87, mmu[0xA596]);
    assert_eq!(false, cpu.regs.flags.z);
    assert_eq!(true, cpu.regs.flags.c);
}

rotate_tests! {
    test_RR_A: (0x1F, true, A, 0xF8, false, false),
    test_RR_B: (0x18, false, B, 0x70, false, true),
    test_RR_C: (0x19, true, C, 0xE9, false, false),
    test_RR_D: (0x1A, false, D, 0x61, false, true),
    test_RR_E: (0x1B, true, E, 0xDA, false, false),
    test_RR_H: (0x1C, false, H, 0x52, false, true),
    test_RR_L: (0x1D, true, L, 0xCB, false, false),
}

#[test]
fn test_RR_HL() {
    let cartridge = build_cartridge(vec![
        0x3E, 0xF0, // LD A, $F0
        0x06, 0xE1, // LD B, $E1
        0x0E, 0xD2, // LD C, $D2
        0x16, 0xC3, // LD D, $C3
        0x1E, 0xB4, // LD E, $B4
        0x26, 0xA5, // LD H, $A5
        0x2E, 0x96, // LD L, $96
        0xCB, 0x1E, // RR (HL)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    mmu[0xA596] = 0x0E;
    cpu.regs.flags.c = true;

    cpu.run(&mut mmu);

    assert_eq!(0x87, mmu[0xA596]);
    assert_eq!(false, cpu.regs.flags.z);
    assert_eq!(false, cpu.regs.flags.c);
}

rotate_tests! {
    test_SLA_A: (0x27, false, A, 0xE0, false, true),
    test_SLA_B: (0x20, false, B, 0xC2, false, true),
    test_SLA_C: (0x21, false, C, 0xA4, false, true),
    test_SLA_D: (0x22, false, D, 0x86, false, true),
    test_SLA_E: (0x23, false, E, 0x68, false, true),
    test_SLA_H: (0x24, false, H, 0x4A, false, true),
    test_SLA_L: (0x25, false, L, 0x2C, false, true),
}

#[test]
fn test_SLA_HL() {
    let cartridge = build_cartridge(vec![
        0x3E, 0xF0, // LD A, $F0
        0x06, 0xE1, // LD B, $E1
        0x0E, 0xD2, // LD C, $D2
        0x16, 0xC3, // LD D, $C3
        0x1E, 0xB4, // LD E, $B4
        0x26, 0xA5, // LD H, $A5
        0x2E, 0x96, // LD L, $96
        0xCB, 0x26, // SLA (HL)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    mmu[0xA596] = 0x0E;
    cpu.regs.flags.c = true;

    cpu.run(&mut mmu);

    assert_eq!(0x1C, mmu[0xA596]);
    assert_eq!(false, cpu.regs.flags.z);
    assert_eq!(false, cpu.regs.flags.c);
}

rotate_tests! {
    test_SRA_A: (0x2F, false, A, 0xF8, false, false),
    test_SRA_B: (0x28, false, B, 0xF0, false, true),
    test_SRA_C: (0x29, false, C, 0xE9, false, false),
    test_SRA_D: (0x2A, false, D, 0xE1, false, true),
    test_SRA_E: (0x2B, false, E, 0xDA, false, false),
    test_SRA_H: (0x2C, false, H, 0xD2, false, true),
    test_SRA_L: (0x2D, false, L, 0xCB, false, false),
}

#[test]
fn test_SRA_HL() {
    let cartridge = build_cartridge(vec![
        0x3E, 0xF0, // LD A, $F0
        0x06, 0xE1, // LD B, $E1
        0x0E, 0xD2, // LD C, $D2
        0x16, 0xC3, // LD D, $C3
        0x1E, 0xB4, // LD E, $B4
        0x26, 0xA5, // LD H, $A5
        0x2E, 0x96, // LD L, $96
        0xCB, 0x2E, // SRA (HL)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    mmu[0xA596] = 0x0E;
    cpu.regs.flags.c = true;

    cpu.run(&mut mmu);

    assert_eq!(0x07, mmu[0xA596]);
    assert_eq!(false, cpu.regs.flags.z);
    assert_eq!(false, cpu.regs.flags.c);
}

rotate_tests! {
    test_SRL_A: (0x3F, false, A, 0x78, false, false),
    test_SRL_B: (0x38, false, B, 0x70, false, true),
    test_SRL_C: (0x39, false, C, 0x69, false, false),
    test_SRL_D: (0x3A, false, D, 0x61, false, true),
    test_SRL_E: (0x3B, false, E, 0x5A, false, false),
    test_SRL_H: (0x3C, false, H, 0x52, false, true),
    test_SRL_L: (0x3D, false, L, 0x4B, false, false),
}

#[test]
fn test_SRL_HL() {
    let cartridge = build_cartridge(vec![
        0x3E, 0xF0, // LD A, $F0
        0x06, 0xE1, // LD B, $E1
        0x0E, 0xD2, // LD C, $D2
        0x16, 0xC3, // LD D, $C3
        0x1E, 0xB4, // LD E, $B4
        0x26, 0xA5, // LD H, $A5
        0x2E, 0x96, // LD L, $96
        0xCB, 0x3E, // SRA (HL)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    mmu[0xA596] = 0x0E;
    cpu.regs.flags.c = true;

    cpu.run(&mut mmu);

    assert_eq!(0x07, mmu[0xA596]);
    assert_eq!(false, cpu.regs.flags.z);
    assert_eq!(false, cpu.regs.flags.c);
}
