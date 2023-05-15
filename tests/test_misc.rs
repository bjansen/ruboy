#![allow(non_snake_case)]

use ruboy;
use ruboy::cpu;
use ruboy::memory::Mmu;
use ruboy::opcodes::RegisterId::{A, B, C, D, E, H, L};

use crate::common::build_cartridge;

mod common;

macro_rules! swap_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (opcode, reg, expected) = $value;

            let cartridge = build_cartridge(vec![
                0x3E, 0xF0, // LD A, $F0
                0x06, 0xE1, // LD B, $E1
                0x0E, 0xD2, // LD C, $D2
                0x16, 0xC3, // LD D, $C3
                0x1E, 0xB4, // LD E, $B4
                0x26, 0xA5, // LD H, $A5
                0x2E, 0x96, // LD L, $96
                0xCB, opcode, // SWAP x
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

swap_tests! {
    test_SWAP_A: (0x37, A, 0x0F),
    test_SWAP_B: (0x30, B, 0x1E),
    test_SWAP_C: (0x31, C, 0x2D),
    test_SWAP_D: (0x32, D, 0x3C),
    test_SWAP_E: (0x33, E, 0x4B),
    test_SWAP_H: (0x34, H, 0x5A),
    test_SWAP_L: (0x35, L, 0x69),
}

#[test]
fn test_SWAP_HL() {
    let cartridge = build_cartridge(vec![
        0x26, 0xA5, // LD H, $A5
        0x2E, 0x96, // LD L, $96
        0xCB, 0x36, // SWAP (HL)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    mmu[0xA596] = 0x87;

    cpu.run(&mut mmu);

    assert_eq!(0x78, mmu[0xA596]);
}

#[test]
fn test_CPL() {
    let cartridge = build_cartridge(vec![
        0x3E, 0b10101100, // LD A, $AC
        0x2F, // CPL
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(0b01010011, cpu.regs[A]);
}

#[test]
fn test_CCF_false() {
    let cartridge = build_cartridge(vec![
        0x3F, // CCF
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.regs.flags.z = true;
    cpu.regs.flags.n = true;
    cpu.regs.flags.h = true;
    cpu.regs.flags.c = false;

    cpu.run(&mut mmu);

    assert_eq!(true, cpu.regs.flags.z);
    assert_eq!(false, cpu.regs.flags.n);
    assert_eq!(false, cpu.regs.flags.h);
    assert_eq!(true, cpu.regs.flags.c);
}

#[test]
fn test_CCF_true() {
    let cartridge = build_cartridge(vec![
        0x3F, // CCF
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.regs.flags.z = true;
    cpu.regs.flags.n = true;
    cpu.regs.flags.h = true;
    cpu.regs.flags.c = true;

    cpu.run(&mut mmu);

    assert_eq!(true, cpu.regs.flags.z);
    assert_eq!(false, cpu.regs.flags.n);
    assert_eq!(false, cpu.regs.flags.h);
    assert_eq!(false, cpu.regs.flags.c);
}

#[test]
fn test_SCF() {
    let cartridge = build_cartridge(vec![
        0x37, // SCF
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.regs.flags.z = true;
    cpu.regs.flags.n = true;
    cpu.regs.flags.h = true;
    cpu.regs.flags.c = false;

    cpu.run(&mut mmu);

    assert_eq!(true, cpu.regs.flags.z);
    assert_eq!(false, cpu.regs.flags.n);
    assert_eq!(false, cpu.regs.flags.h);
    assert_eq!(true, cpu.regs.flags.c);
}

#[test]
fn test_NOP() {
    let cartridge = build_cartridge(vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x3E, 0xAA, // LD A, $AA
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(0xAA, cpu.regs[A]);
}

