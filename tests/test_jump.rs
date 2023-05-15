#![allow(non_snake_case)]

use ruboy;
use ruboy::cpu;
use ruboy::memory::Mmu;
use ruboy::opcodes::{RegisterId::{B, D}};

use crate::common::build_cartridge;

mod common;

#[test]
fn test_JP_nn() {
    let cartridge = build_cartridge(vec![
        0xC3, 0x00, 0x20, // JP $2000
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    mmu[0x2000] = 0x16; // LD D, $42
    mmu[0x2001] = 0x42;
    mmu[0x2002] = 0x10; // STOP
    mmu[0x2003] = 0x00;

    cpu.run(&mut mmu);

    assert_eq!(0x42, cpu.regs[D]);
    assert_eq!(0x2003, cpu.regs.pc);
}

macro_rules! jp_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (opcode, z, c, expected_d, expected_pc) = $value;

            let cartridge = build_cartridge(vec![
                opcode, 0x00, 0x20, // JP cc,$2000
                0x10, 0x00, // STOP
            ]);

            let mut cpu = cpu::init_cpu();
            let mut mmu = Mmu::new(cartridge);

            mmu[0x2000] = 0x16; // LD D, $42
            mmu[0x2001] = 0x42;
            mmu[0x2002] = 0x10; // STOP
            mmu[0x2003] = 0x00;

            cpu.regs.flags.z = z;
            cpu.regs.flags.c = c;
            cpu.regs[D] = 0x00;

            cpu.run(&mut mmu);

            assert_eq!(expected_d, cpu.regs[D]);
            assert_eq!(expected_pc, cpu.regs.pc);
        }
    )*
    }
}

jp_tests! {
    test_JP_NZ_nn_ok: (0xC2, false, false, 0x42, 0x2003),
    test_JP_NZ_nn_ko: (0xC2, true, false, 0x00, 0x0104),
    test_JP_Z_nn_ok: (0xCA, true, false, 0x42, 0x2003),
    test_JP_Z_nn_ko: (0xCA, false, false, 0x00, 0x0104),
    test_JP_NC_nn_ok: (0xD2, false, false, 0x42, 0x2003),
    test_JP_NC_nn_ko: (0xD2, false, true, 0x00, 0x0104),
    test_JP_C_nn_ok: (0xDA, false, true, 0x42, 0x2003),
    test_JP_C_nn_ko: (0xDA, false, false, 0x00, 0x0104),
}

#[test]
fn test_JP_HL() {
    let cartridge = build_cartridge(vec![
        0x21, 0x00, 0x20, // LD HL, $2000
        0xE9, // JP (HL)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    mmu[0x2000] = 0x16; // LD D, $42
    mmu[0x2001] = 0x42;
    mmu[0x2002] = 0x10; // STOP
    mmu[0x2003] = 0x00;

    cpu.run(&mut mmu);

    assert_eq!(0x42, cpu.regs[D]);
    assert_eq!(0x2003, cpu.regs.pc);
}

#[test]
fn test_JR() {
    let cartridge = build_cartridge(vec![
        0x18, 0x02, // JR $032
        0x06, 0x51, // LD B, $51
        0x16, 0x42, // LD D, $42
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.regs[B] = 0x00;
    cpu.regs[D] = 0x00;

    cpu.run(&mut mmu);

    assert_eq!(0x00, cpu.regs[B]);
    assert_eq!(0x42, cpu.regs[D]);
}

macro_rules! jr_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (opcode, z, c, expected_b, expected_d) = $value;

            let cartridge = build_cartridge(vec![
                opcode, 0x02, // JR cc, $032
                0x06, 0x51, // LD B, $51
                0x16, 0x42, // LD D, $42
                0x10, 0x00, // STOP
            ]);

            let mut cpu = cpu::init_cpu();
            let mut mmu = Mmu::new(cartridge);

            cpu.regs.flags.z = z;
            cpu.regs.flags.c = c;
            cpu.regs[B] = 0x00;
            cpu.regs[D] = 0x00;

            cpu.run(&mut mmu);

            assert_eq!(expected_b, cpu.regs[B]);
            assert_eq!(expected_d, cpu.regs[D]);
        }
    )*
    }
}

jr_tests! {
    test_JR_NZ_nn_ok: (0x20, false, false, 0x00, 0x42),
    test_JR_NZ_nn_ko: (0x20, true, false, 0x51, 0x42),
    test_JR_Z_nn_ok: (0x28, true, false, 0x00, 0x42),
    test_JR_Z_nn_ko: (0x28, false, false, 0x51, 0x42),
    test_JR_NC_nn_ok: (0x30, false, false, 0x00, 0x42),
    test_JR_NC_nn_ko: (0x30, false, true, 0x51, 0x42),
    test_JR_C_nn_ok: (0x38, false, true, 0x00, 0x42),
    test_JR_C_nn_ko: (0x38, false, false, 0x51, 0x42),
}
