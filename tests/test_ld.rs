#![allow(non_snake_case)]

use ruboy;
use ruboy::cpu;
use ruboy::memory::Mmu;
use ruboy::opcodes::{Register16Id::{BC, DE, HL}, RegisterId::{A, B, C, D, E, H, L}};
use ruboy::opcodes::Operand::{Byte, DirectAddress, IndirectAddress, Register};

use crate::common::build_cartridge;

mod common;

macro_rules! ld_nn_n_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (opcode, register) = $value;

            let cartridge = build_cartridge(vec![
                opcode, 0x69, // LD x, $69
                0x10, 0x00, // STOP
            ]);

            let mut cpu = cpu::init_cpu();
            let mut mmu = Mmu::new(cartridge);

            cpu.run(&mut mmu);

            match register {
                Register(reg) => assert_eq!(cpu.regs[reg], 0x69),
                _ => panic!(),
            }
        }
    )*
    }
}

ld_nn_n_tests! {
    test_LD_Bn: (0x06, Register(B)),
    test_LD_Cn: (0x0E, Register(C)),
    test_LD_Dn: (0x16, Register(D)),
    test_LD_En: (0x1E, Register(E)),
    test_LD_Hn: (0x26, Register(H)),
    test_LD_Ln: (0x2E, Register(L)),
}

macro_rules! ld_r1_r2_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (opcode, r1, r2) = $value;

            let cartridge = build_cartridge(vec![
                0x3E, 0x0A, // LD A, $0A
                0x06, 0x0B, // LD B, $0B
                0x0E, 0x0C, // LD C, $0C
                0x16, 0x0D, // LD D, $0D
                0x1E, 0x0E, // LD E, $0E
                0x26, 0xFF, // LD H, $FF
                0x2E, 0x11, // LD L, $11
                opcode, // LD r1, r2
                0x10, 0x00, // STOP or address $0010 for certain opcodes
                0x10, 0x00, // STOP
            ]);

            let mut cpu = cpu::init_cpu();
            let mut mmu = Mmu::new(cartridge);

            mmu[0xFF11] = 0x22;
            mmu[0x0B0C] = 0x33;
            mmu[0x0D0E] = 0x44;
            mmu[0x0010] = 0x55;

            cpu.run(&mut mmu);

            let val1 = match r1 {
                IndirectAddress(HL) => mmu[0xFF11],
                IndirectAddress(BC) => mmu[0x0B0C],
                IndirectAddress(DE) => mmu[0x0D0E],
                DirectAddress => mmu[0x0010],
                Register(reg) => cpu.regs[reg],
                _ => panic!(),
            };

            let val2 = match r2 {
                IndirectAddress(HL) => 0x22,
                IndirectAddress(BC) => 0x33,
                IndirectAddress(DE) => 0x44,
                DirectAddress => 0x55,
                Byte => 0x10,
                Register(reg) => cpu.regs[reg],
                _ => panic!(),
            };

            assert_eq!(val1, val2);
        }
    )*
    }
}

ld_r1_r2_tests! {
    test_LD_AA: (0x7F, Register(A), Register(A)),
    test_LD_AB: (0x78, Register(A), Register(B)),
    test_LD_AC: (0x79, Register(A), Register(C)),
    test_LD_AD: (0x7A, Register(A), Register(D)),
    test_LD_AE: (0x7B, Register(A), Register(E)),
    test_LD_AH: (0x7C, Register(A), Register(H)),
    test_LD_AL: (0x7D, Register(A), Register(L)),
    test_LD_ABC: (0x0A, Register(A), IndirectAddress(BC)),
    test_LD_ADE: (0x1A, Register(A), IndirectAddress(DE)),
    test_LD_AHL: (0x7E, Register(A), IndirectAddress(HL)),
    test_LD_Ann: (0xFA, Register(A), DirectAddress),
    test_LD_An: (0x3E, Register(A), Byte),

    test_LD_BA: (0x47, Register(B), Register(A)),
    test_LD_BB: (0x40, Register(B), Register(B)),
    test_LD_BC: (0x41, Register(B), Register(C)),
    test_LD_BD: (0x42, Register(B), Register(D)),
    test_LD_BE: (0x43, Register(B), Register(E)),
    test_LD_BH: (0x44, Register(B), Register(H)),
    test_LD_BL: (0x45, Register(B), Register(L)),
    test_LD_BHL: (0x46, Register(B), IndirectAddress(HL)),

    test_LD_CA: (0x4F, Register(C), Register(A)),
    test_LD_CB: (0x48, Register(C), Register(B)),
    test_LD_CC: (0x49, Register(C), Register(C)),
    test_LD_CD: (0x4A, Register(C), Register(D)),
    test_LD_CE: (0x4B, Register(C), Register(E)),
    test_LD_CH: (0x4C, Register(C), Register(H)),
    test_LD_CL: (0x4D, Register(C), Register(L)),
    test_LD_CHL: (0x4E, Register(C), IndirectAddress(HL)),

    test_LD_DA: (0x57, Register(D), Register(A)),
    test_LD_DB: (0x50, Register(D), Register(B)),
    test_LD_DC: (0x51, Register(D), Register(C)),
    test_LD_DD: (0x52, Register(D), Register(D)),
    test_LD_DE: (0x53, Register(D), Register(E)),
    test_LD_DH: (0x54, Register(D), Register(H)),
    test_LD_DL: (0x55, Register(D), Register(L)),
    test_LD_DHL: (0x56, Register(D), IndirectAddress(HL)),

    test_LD_EA: (0x5F, Register(E), Register(A)),
    test_LD_EB: (0x58, Register(E), Register(B)),
    test_LD_EC: (0x59, Register(E), Register(C)),
    test_LD_ED: (0x5A, Register(E), Register(D)),
    test_LD_EE: (0x5B, Register(E), Register(E)),
    test_LD_EH: (0x5C, Register(E), Register(H)),
    test_LD_EL: (0x5D, Register(E), Register(L)),
    test_LD_EHL: (0x5E, Register(E), IndirectAddress(HL)),

    test_LD_HA: (0x67, Register(H), Register(A)),
    test_LD_HB: (0x60, Register(H), Register(B)),
    test_LD_HC: (0x61, Register(H), Register(C)),
    test_LD_HD: (0x62, Register(H), Register(D)),
    test_LD_HE: (0x63, Register(H), Register(E)),
    test_LD_HH: (0x64, Register(H), Register(H)),
    test_LD_HL: (0x65, Register(H), Register(L)),
    test_LD_HHL: (0x66, Register(H), IndirectAddress(HL)),

    test_LD_LA: (0x6F, Register(L), Register(A)),
    test_LD_LB: (0x68, Register(L), Register(B)),
    test_LD_LC: (0x69, Register(L), Register(C)),
    test_LD_LD: (0x6A, Register(L), Register(D)),
    test_LD_LE: (0x6B, Register(L), Register(E)),
    test_LD_LH: (0x6C, Register(L), Register(H)),
    test_LD_LL: (0x6D, Register(L), Register(L)),
    test_LD_LHL: (0x6E, Register(L), IndirectAddress(HL)),

    test_LD_HLA: (0x77, IndirectAddress(HL), Register(A)),
    test_LD_HLB: (0x70, IndirectAddress(HL), Register(B)),
    test_LD_HLC: (0x71, IndirectAddress(HL), Register(C)),
    test_LD_HLD: (0x72, IndirectAddress(HL), Register(D)),
    test_LD_HLE: (0x73, IndirectAddress(HL), Register(E)),
    test_LD_HLH: (0x74, IndirectAddress(HL), Register(H)),
    test_LD_HLL: (0x75, IndirectAddress(HL), Register(L)),
    test_LD_HLn: (0x36, IndirectAddress(HL), Byte),

    test_LD_BCA: (0x02, IndirectAddress(BC), Register(A)),
    test_LD_DEA: (0x12, IndirectAddress(DE), Register(A)),
    test_LD_nnA: (0xEA, DirectAddress, Register(A)),
}

#[test]
fn test_LD_BCnn() {
    let cartridge = build_cartridge(vec![
        0x01, 0xAA, 0xBB, // LD BC, 0xBBAA
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs[B], 0xBB);
    assert_eq!(cpu.regs[C], 0xAA);
}

#[test]
fn test_LD_DEnn() {
    let cartridge = build_cartridge(vec![
        0x11, 0xBB, 0xCC, // LD DE, 0xCCBB
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs[D], 0xCC);
    assert_eq!(cpu.regs[E], 0xBB);
}

#[test]
fn test_LD_HLnn() {
    let cartridge = build_cartridge(vec![
        0x21, 0xCC, 0xDD, // LD HL, 0xDDCC
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs[H], 0xDD);
    assert_eq!(cpu.regs[L], 0xCC);
}

#[test]
fn test_LD_SPnn() {
    let cartridge = build_cartridge(vec![
        0x31, 0xDD, 0xEE, // LD SP, 0xEEDD
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs.sp, 0xEEDD);
}

#[test]
fn test_LD_SPHL() {
    let cartridge = build_cartridge(vec![
        0x21, 0x12, 0x34, // LD HL, 0x3412
        0xF9, // LD SP, HL
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs.sp, 0x3412);
}

#[test]
fn test_LD_nnSP() {
    let cartridge = build_cartridge(vec![
        0x08, 0xDD, 0xEE, // LD 0xEEDD, SP
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(cpu.regs.sp, u16::from_le_bytes([mmu[0xEEDD], mmu[0xEEDE]]));
}

#[test]
fn test_LD_A_C_() {
    let cartridge = build_cartridge(vec![
        0x0E, 0x11, // LD C, $11
        0xF2, // LD A, ($FF00 + C)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    mmu[0xFF00 + 0x11] = 0x21;

    cpu.run(&mut mmu);

    assert_eq!(0x21, cpu.regs[A]);
}

#[test]
fn test_LD_C_A_() {
    let cartridge = build_cartridge(vec![
        0x3E, 0x22, // LD A, $22
        0x0E, 0x11, // LD C, $11
        0xE2, // LD ($FF00 + C), A
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(0x22, mmu[0xFF00 + 0x11]);
}

#[test]
fn test_LDD_AHL() {
    let cartridge = build_cartridge(vec![
        0x21, 0x11, 0xFF, // LD HL, $FF11
        0x3A, // LDD A, (HL)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    mmu[0xFF11] = 0x66;

    cpu.run(&mut mmu);

    assert_eq!(0x66, cpu.regs[A]);
    assert_eq!(0xFF10, cpu.regs.get(HL));
}

#[test]
fn test_LDD_HLA() {
    let cartridge = build_cartridge(vec![
        0x21, 0x11, 0xFF, // LD HL, $FF11
        0x3E, 0x66, // LD A, $66
        0x32, // LDD (HL), A
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(0x66, mmu[0xFF11]);
    assert_eq!(0xFF10, cpu.regs.get(HL));
}

#[test]
fn test_LDI_AHL() {
    let cartridge = build_cartridge(vec![
        0x21, 0x11, 0xFF, // LD HL, $FF11
        0x2A, // LDI A, (HL)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    mmu[0xFF11] = 0x66;

    cpu.run(&mut mmu);

    assert_eq!(0x66, cpu.regs[A]);
    assert_eq!(0xFF12, cpu.regs.get(HL));
}

#[test]
fn test_LDI_HLA() {
    let cartridge = build_cartridge(vec![
        0x21, 0x11, 0xFF, // LD HL, $FF11
        0x3E, 0x66, // LD A, $66
        0x22, // LDI (HL), A
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(0x66, mmu[0xFF11]);
    assert_eq!(0xFF12, cpu.regs.get(HL));
}

#[test]
fn test_LDH_nA() {
    let cartridge = build_cartridge(vec![
        0x3E, 0x66, // LD A, $66
        0xE0, 0x01, // LD ($FF00 + $01), A
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(0x66, mmu[0xFF01]);
}

#[test]
fn test_LDH_An() {
    let cartridge = build_cartridge(vec![
        0xF0, 0x01, // LD A, ($FF00 + $01)
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    mmu[0xFF01] = 0x77;

    cpu.run(&mut mmu);

    assert_eq!(0x77, cpu.regs[A]);
}

#[test]
fn test_LDHL_SPn() {
    let cartridge = build_cartridge(vec![
        0xF8, 0x02, // LDHL SP, $02
        0x10, 0x00, // STOP
    ]);

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_eq!(0x0000, cpu.regs.get(HL));
}
