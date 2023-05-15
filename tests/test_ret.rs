#![allow(non_snake_case)]

use ruboy;
use ruboy::cpu;
use ruboy::memory::Mmu;
use ruboy::opcodes::RegisterId::D;

use crate::common::build_cartridge;

mod common;

#[test]
fn test_RET() {
    let cartridge = build_cartridge(vec![
        0x21, 0x07, 0x01, // LD HL, $0107
        0xE5, // PUSH HL
        0xC9, // RET
        0x18, 0xF2, // JR -2 (infinite loop)
        0x16, 0x42, // // LD D, $42
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(0x42, cpu.regs[D]);
}

#[test]
fn test_RETI() {
    let cartridge = build_cartridge(vec![
        0x21, 0x07, 0x01, // LD HL, $0107
        0xE5, // PUSH HL
        0xD9, // RETI
        0x18, 0xF2, // JR -2 (infinite loop)
        0x16, 0x42, // // LD D, $42
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(0x42, cpu.regs[D]);
    // TODO check that interrupts are enabled
}

macro_rules! ret_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (opcode, z, c, expected_d) = $value;

            let cartridge = build_cartridge(vec![
                0x21, 0x09, 0x01, // LD HL, $0109
                0xE5, // PUSH HL
                opcode, // RET cc
                0x16, 0x69, // LD D, $69
                0x10, 0x00, // STOP
                0x16, 0x42, // LD D, $42
                0x10, 0x00 // STOP
            ]);

            let mut cpu = cpu::init_cpu();
            let mut mmu = Mmu::new(cartridge);

            cpu.regs.flags.z = z;
            cpu.regs.flags.c = c;
            cpu.regs[D] = 0x00;

            cpu.run(&mut mmu);

            assert_eq!(expected_d, cpu.regs[D]);
        }
    )*
    }
}

ret_tests! {
    test_RET_NZ_nn_ok: (0xC0, false, false, 0x42),
    test_RET_NZ_nn_ko: (0xC0, true, false, 0x69),
    test_RET_Z_nn_ok: (0xC8, true, false, 0x42),
    test_RET_Z_nn_ko: (0xC8, false, false, 0x69),
    test_RET_NC_nn_ok: (0xD0, false, false, 0x42),
    test_RET_NC_nn_ko: (0xD0, false, true, 0x69),
    test_RET_C_nn_ok: (0xD8, false, true, 0x42),
    test_RET_C_nn_ko: (0xD8, false, false, 0x69),
}
