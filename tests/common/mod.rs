use ruboy::cartridge::Cartridge;
use ruboy::cpu::{Cpu, Flag};

pub fn build_cartridge(program: Vec<u8>) -> Cartridge {
    let mut content = vec![0; 0x8000];

    content[0x0100..0x0100 + program.len()].copy_from_slice(program.as_slice());

    Cartridge {
        content,
    }
}

pub fn assert_flags_eq(cpu: &Cpu, z: bool, n: bool, h: bool, c: bool) {
    assert_eq!(z, cpu.regs[Flag::Z]);
    assert_eq!(n, cpu.regs[Flag::N]);
    assert_eq!(h, cpu.regs[Flag::H]);
    assert_eq!(c, cpu.regs[Flag::C]);
}