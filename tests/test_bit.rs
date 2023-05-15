#![allow(non_snake_case)]

use ruboy;
use ruboy::cpu;
use ruboy::memory::Mmu;

use crate::common::build_cartridge;

mod common;

macro_rules! bit_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (opcode, expected_z) = $value;

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

            mmu[0xA596] = 0b01010101;

            cpu.run(&mut mmu);

            assert_eq!(expected_z, cpu.regs.flags.z);
            assert_eq!(false, cpu.regs.flags.n);
            assert_eq!(true, cpu.regs.flags.h);
        }
    )*
    }
}

bit_tests! {
    test_BIT_0A: (0x47, true),
    test_BIT_1A: (0x4F, true),
    test_BIT_2A: (0x57, true),
    test_BIT_3A: (0x5F, true),
    test_BIT_4A: (0x67, false),
    test_BIT_5A: (0x6F, false),
    test_BIT_6A: (0x77, false),
    test_BIT_7A: (0x7F, false),

    test_BIT_0B: (0x40, false),
    test_BIT_1B: (0x48, true),
    test_BIT_2B: (0x50, true),
    test_BIT_3B: (0x58, true),
    test_BIT_4B: (0x60, true),
    test_BIT_5B: (0x68, false),
    test_BIT_6B: (0x70, false),
    test_BIT_7B: (0x78, false),

    test_BIT_0C: (0x41, true),
    test_BIT_1C: (0x49, false),
    test_BIT_2C: (0x51, true),
    test_BIT_3C: (0x59, true),
    test_BIT_4C: (0x61, false),
    test_BIT_5C: (0x69, true),
    test_BIT_6C: (0x71, false),
    test_BIT_7C: (0x79, false),

    test_BIT_0D: (0x42, false),
    test_BIT_1D: (0x4A, false),
    test_BIT_2D: (0x52, true),
    test_BIT_3D: (0x5A, true),
    test_BIT_4D: (0x62, true),
    test_BIT_5D: (0x6A, true),
    test_BIT_6D: (0x72, false),
    test_BIT_7D: (0x7A, false),

    test_BIT_0E: (0x43, true),
    test_BIT_1E: (0x4B, true),
    test_BIT_2E: (0x53, false),
    test_BIT_3E: (0x5B, true),
    test_BIT_4E: (0x63, false),
    test_BIT_5E: (0x6B, false),
    test_BIT_6E: (0x73, true),
    test_BIT_7E: (0x7B, false),

    test_BIT_0H: (0x44, false),
    test_BIT_1H: (0x4C, true),
    test_BIT_2H: (0x54, false),
    test_BIT_3H: (0x5C, true),
    test_BIT_4H: (0x64, true),
    test_BIT_5H: (0x6C, false),
    test_BIT_6H: (0x74, true),
    test_BIT_7H: (0x7C, false),

    test_BIT_0L: (0x45, true),
    test_BIT_1L: (0x4D, false),
    test_BIT_2L: (0x55, false),
    test_BIT_3L: (0x5D, true),
    test_BIT_4L: (0x65, false),
    test_BIT_5L: (0x6D, true),
    test_BIT_6L: (0x75, true),
    test_BIT_7L: (0x7D, false),

    test_BIT_0HL: (0x46, false),
    test_BIT_1HL: (0x4E, true),
    test_BIT_2HL: (0x56, false),
    test_BIT_3HL: (0x5E, true),
    test_BIT_4HL: (0x66, false),
    test_BIT_5HL: (0x6E, true),
    test_BIT_6HL: (0x76, false),
    test_BIT_7HL: (0x7E, true),
}
