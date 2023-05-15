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
                0xCB, opcode, // SET b,x
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
    test_SET_0A: (0xC7, A, 0xF1),
    test_SET_1A: (0xCF, A, 0xF2),
    test_SET_2A: (0xD7, A, 0xF4),
    test_SET_3A: (0xDF, A, 0xF8),
    test_SET_4A: (0xE7, A, 0xF0),
    test_SET_5A: (0xEF, A, 0xF0),
    test_SET_6A: (0xF7, A, 0xF0),
    test_SET_7A: (0xFF, A, 0xF0),

    test_SET_0B: (0xC0, B, 0xE1),
    test_SET_1B: (0xC8, B, 0xE3),
    test_SET_2B: (0xD0, B, 0xE5),
    test_SET_3B: (0xD8, B, 0xE9),
    test_SET_4B: (0xE0, B, 0xF1),
    test_SET_5B: (0xE8, B, 0xE1),
    test_SET_6B: (0xF0, B, 0xE1),
    test_SET_7B: (0xF8, B, 0xE1),

    test_SET_0C: (0xC1, C, 0xD3),
    test_SET_1C: (0xC9, C, 0xD2),
    test_SET_2C: (0xD1, C, 0xD6),
    test_SET_3C: (0xD9, C, 0xDA),
    test_SET_4C: (0xE1, C, 0xD2),
    test_SET_5C: (0xE9, C, 0xF2),
    test_SET_6C: (0xF1, C, 0xD2),
    test_SET_7C: (0xF9, C, 0xD2),

    test_SET_0D: (0xC2, D, 0xC3),
    test_SET_1D: (0xCA, D, 0xC3),
    test_SET_2D: (0xD2, D, 0xC7),
    test_SET_3D: (0xDA, D, 0xCB),
    test_SET_4D: (0xE2, D, 0xD3),
    test_SET_5D: (0xEA, D, 0xE3),
    test_SET_6D: (0xF2, D, 0xC3),
    test_SET_7D: (0xFA, D, 0xC3),

    test_SET_0E: (0xC3, E, 0xB5),
    test_SET_1E: (0xCB, E, 0xB6),
    test_SET_2E: (0xD3, E, 0xB4),
    test_SET_3E: (0xDB, E, 0xBC),
    test_SET_4E: (0xE3, E, 0xB4),
    test_SET_5E: (0xEB, E, 0xB4),
    test_SET_6E: (0xF3, E, 0xF4),
    test_SET_7E: (0xFB, E, 0xB4),

    test_SET_0H: (0xC4, H, 0xA5),
    test_SET_1H: (0xCC, H, 0xA7),
    test_SET_2H: (0xD4, H, 0xA5),
    test_SET_3H: (0xDC, H, 0xAD),
    test_SET_4H: (0xE4, H, 0xB5),
    test_SET_5H: (0xEC, H, 0xA5),
    test_SET_6H: (0xF4, H, 0xE5),
    test_SET_7H: (0xFC, H, 0xA5),

    test_SET_0L: (0xC5, L, 0x97),
    test_SET_1L: (0xCD, L, 0x96),
    test_SET_2L: (0xD5, L, 0x96),
    test_SET_3L: (0xDD, L, 0x9E),
    test_SET_4L: (0xE5, L, 0x96),
    test_SET_5L: (0xED, L, 0xB6),
    test_SET_6L: (0xF5, L, 0xD6),
    test_SET_7L: (0xFD, L, 0x96),

    test_SET_0HL: (0xC6, HL, 0b01010101),
    test_SET_1HL: (0xCE, HL, 0b01010111),
    test_SET_2HL: (0xD6, HL, 0b01010101),
    test_SET_3HL: (0xDE, HL, 0b01011101),
    test_SET_4HL: (0xE6, HL, 0b01010101),
    test_SET_5HL: (0xEE, HL, 0b01110101),
    test_SET_6HL: (0xF6, HL, 0b01010101),
    test_SET_7HL: (0xFE, HL, 0b11010101),
}
