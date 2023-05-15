#![allow(non_snake_case)]

use ruboy;
use ruboy::cpu;
use ruboy::memory::Mmu;
use ruboy::opcodes::Register16Id;
use ruboy::opcodes::RegisterId;
use ruboy::opcodes::RegisterId::{A, B, C, D, E, H, HL, L};

use crate::common::build_cartridge;

mod common;

macro_rules! set_tests {
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
                0xCB, opcode, // RES b,x
                0x10, 0x00, // STOP
            ]);

            let mut cpu = cpu::init_cpu();
            let mut mmu = Mmu::new(cartridge);

            mmu[0xA596] = 0b01010101;

            cpu.run(&mut mmu);

            match reg {
                RegisterId::HL => {
                    assert_eq!(expected, mmu[cpu.regs.get(Register16Id::HL)]);
                }
                _ => {
                    assert_eq!(expected, cpu.regs[reg]);
                }
            }
        }
    )*
    }
}

set_tests! {
    test_RES_0A: (0x87, A, 0xF0),
    test_RES_1A: (0x8F, A, 0xF0),
    test_RES_2A: (0x97, A, 0xF0),
    test_RES_3A: (0x9F, A, 0xF0),
    test_RES_4A: (0xA7, A, 0xE0),
    test_RES_5A: (0xAF, A, 0xD0),
    test_RES_6A: (0xB7, A, 0xB0),
    test_RES_7A: (0xBF, A, 0x70),

    test_RES_0B: (0x80, B, 0xE0),
    test_RES_1B: (0x88, B, 0xE1),
    test_RES_2B: (0x90, B, 0xE1),
    test_RES_3B: (0x98, B, 0xE1),
    test_RES_4B: (0xA0, B, 0xE1),
    test_RES_5B: (0xA8, B, 0xC1),
    test_RES_6B: (0xB0, B, 0xA1),
    test_RES_7B: (0xB8, B, 0x61),

    test_RES_0C: (0x81, C, 0xD2),
    test_RES_1C: (0x89, C, 0xD0),
    test_RES_2C: (0x91, C, 0xD2),
    test_RES_3C: (0x99, C, 0xD2),
    test_RES_4C: (0xA1, C, 0xC2),
    test_RES_5C: (0xA9, C, 0xD2),
    test_RES_6C: (0xB1, C, 0x92),
    test_RES_7C: (0xB9, C, 0x52),

    test_RES_0D: (0x82, D, 0xC2),
    test_RES_1D: (0x8A, D, 0xC1),
    test_RES_2D: (0x92, D, 0xC3),
    test_RES_3D: (0x9A, D, 0xC3),
    test_RES_4D: (0xA2, D, 0xC3),
    test_RES_5D: (0xAA, D, 0xC3),
    test_RES_6D: (0xB2, D, 0x83),
    test_RES_7D: (0xBA, D, 0x43),

    test_RES_0E: (0x83, E, 0xB4),
    test_RES_1E: (0x8B, E, 0xB4),
    test_RES_2E: (0x93, E, 0xB0),
    test_RES_3E: (0x9B, E, 0xB4),
    test_RES_4E: (0xA3, E, 0xA4),
    test_RES_5E: (0xAB, E, 0x94),
    test_RES_6E: (0xB3, E, 0xB4),
    test_RES_7E: (0xBB, E, 0x34),

    test_RES_0H: (0x84, H, 0xA4),
    test_RES_1H: (0x8C, H, 0xA5),
    test_RES_2H: (0x94, H, 0xA1),
    test_RES_3H: (0x9C, H, 0xA5),
    test_RES_4H: (0xA4, H, 0xA5),
    test_RES_5H: (0xAC, H, 0x85),
    test_RES_6H: (0xB4, H, 0xA5),
    test_RES_7H: (0xBC, H, 0x25),

    test_RES_0L: (0x85, L, 0x96),
    test_RES_1L: (0x8D, L, 0x94),
    test_RES_2L: (0x95, L, 0x92),
    test_RES_3L: (0x9D, L, 0x96),
    test_RES_4L: (0xA5, L, 0x86),
    test_RES_5L: (0xAD, L, 0x96),
    test_RES_6L: (0xB5, L, 0x96),
    test_RES_7L: (0xBD, L, 0x16),

    test_RES_0HL: (0x86, HL, 0b01010100),
    test_RES_1HL: (0x8E, HL, 0b01010101),
    test_RES_2HL: (0x96, HL, 0b01010001),
    test_RES_3HL: (0x9E, HL, 0b01010101),
    test_RES_4HL: (0xA6, HL, 0b01000101),
    test_RES_5HL: (0xAE, HL, 0b01010101),
    test_RES_6HL: (0xB6, HL, 0b00010101),
    test_RES_7HL: (0xBE, HL, 0b01010101),
}
