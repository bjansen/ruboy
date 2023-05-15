use ruboy;
use ruboy::cpu;
use ruboy::cpu::Flag;

use crate::common::assert_flags_eq;

mod common;

#[test]
fn test_flags() {
    let mut cpu = cpu::init_cpu();

    cpu.regs.flags.set_f(0);
    assert_flags_eq(&cpu, false, false, false, false);
    assert_eq!(cpu.regs.flags.get_f(), 0);

    cpu.regs.flags.set_f(0b10000000);
    assert_flags_eq(&cpu, true, false, false, false);
    assert_eq!(cpu.regs.flags.get_f(), 0b10000000);

    cpu.regs.flags.set_f(0b01000000);
    assert_flags_eq(&cpu, false, true, false, false);
    assert_eq!(cpu.regs.flags.get_f(), 0b01000000);

    cpu.regs.flags.set_f(0b00100000);
    assert_flags_eq(&cpu, false, false, true, false);
    assert_eq!(cpu.regs.flags.get_f(), 0b00100000);

    cpu.regs.flags.set_f(0b00010000);
    assert_flags_eq(&cpu, false, false, false, true);
    assert_eq!(cpu.regs.flags.get_f(), 0b00010000);

    cpu.regs.flags.set_f(0);
    cpu.regs[Flag::Z] = true;
    assert_flags_eq(&cpu, true, false, false, false);
    assert_eq!(cpu.regs.flags.get_f(), 0b10000000);

    cpu.regs.flags.set_f(0);
    cpu.regs[Flag::N] = true;
    assert_flags_eq(&cpu, false, true, false, false);
    assert_eq!(cpu.regs.flags.get_f(), 0b01000000);

    cpu.regs.flags.set_f(0);
    cpu.regs[Flag::H] = true;
    assert_flags_eq(&cpu, false, false, true, false);
    assert_eq!(cpu.regs.flags.get_f(), 0b00100000);

    cpu.regs.flags.set_f(0);
    cpu.regs[Flag::C] = true;
    assert_flags_eq(&cpu, false, false, false, true);
    assert_eq!(cpu.regs.flags.get_f(), 0b00010000);
}
