#![allow(non_snake_case)]

use ruboy;
use ruboy::cpu;
use ruboy::memory::Mmu;
use ruboy::opcodes::RegisterId::D;

use crate::common::build_cartridge;

mod common;

#[test]
fn test_CALL_nn() {
    let cartridge = build_cartridge(vec![
        0xCD, 0x00, 0x20, // CALL $2000
        0x18, 0xF2, // JR -2 (infinite loop)
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
    assert_eq!(0x03, mmu[0xFFFC]);
    assert_eq!(0x01, mmu[0xFFFD]);
}

macro_rules! call_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (opcode, z, c, expected_d) = $value;

            let cartridge = build_cartridge(vec![
                opcode, 0x00, 0x20, // CALL $2000
                0x16, 0x69, // LD D, $69
                0x10, 0x00 // STOP
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
        }
    )*
    }
}

call_tests! {
    test_CALL_NZ_nn_ok: (0xC4, false, false, 0x42),
    test_CALL_NZ_nn_ko: (0xC4, true, false, 0x69),
    test_CALL_Z_nn_ok: (0xCC, true, false, 0x42),
    test_CALL_Z_nn_ko: (0xCC, false, false, 0x69),
    test_CALL_NC_nn_ok: (0xD4, false, false, 0x42),
    test_CALL_NC_nn_ko: (0xD4, false, true, 0x69),
    test_CALL_C_nn_ok: (0xDC, false, true, 0x42),
    test_CALL_C_nn_ko: (0xDC, false, false, 0x69),
}
