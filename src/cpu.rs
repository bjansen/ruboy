use std::ops::{BitAnd, BitXor, Index, IndexMut};

use InstructionType::*;

use crate::cpu::Flag::{C, H, N, Z};
use crate::memory::Mmu;
use crate::opcodes::{InstructionType, FlagId, Instruction, Operand, Register16Id, RegisterId};
use crate::opcodes::Register16Id::HL;

pub struct Cpu {
    /// CPU registers
    pub regs: Registers,
}

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    /// Stack Pointer (points to the top of the stack)
    pub sp: u16,

    /// Program Counter (points to the next instruction)
    pub pc: u16,

    pub flags: FlagRegister,
}

pub enum Flag {
    Z,
    N,
    H,
    C,
}

impl Index<RegisterId> for Registers {
    type Output = u8;

    fn index(&self, index: RegisterId) -> &Self::Output {
        match index {
            RegisterId::A => &self.a,
            RegisterId::B => &self.b,
            RegisterId::C => &self.c,
            RegisterId::D => &self.d,
            RegisterId::E => &self.e,
            RegisterId::H => &self.h,
            RegisterId::L => &self.l,
            _ => panic!("Unsupported register id {:?}", index)
        }
    }
}

impl IndexMut<RegisterId> for Registers {
    fn index_mut(&mut self, index: RegisterId) -> &mut Self::Output {
        match index {
            RegisterId::A => &mut self.a,
            RegisterId::B => &mut self.b,
            RegisterId::C => &mut self.c,
            RegisterId::D => &mut self.d,
            RegisterId::E => &mut self.e,
            RegisterId::H => &mut self.h,
            RegisterId::L => &mut self.l,
            _ => panic!("Unsupported register id {:?}", index)
        }
    }
}

impl Index<Flag> for Registers {
    type Output = bool;

    fn index(&self, index: Flag) -> &Self::Output {
        match index {
            Z => &self.flags.z,
            N => &self.flags.n,
            H => &self.flags.h,
            C => &self.flags.c,
        }
    }
}

impl IndexMut<Flag> for Registers {
    fn index_mut(&mut self, index: Flag) -> &mut Self::Output {
        match index {
            Z => &mut self.flags.z,
            N => &mut self.flags.n,
            H => &mut self.flags.h,
            C => &mut self.flags.c,
        }
    }
}

impl Registers {
    pub fn get(&self, reg: Register16Id) -> u16 {
        match reg {
            Register16Id::AF => u16::from_be_bytes([self.a, self.flags.get_f()]),
            Register16Id::BC => u16::from_be_bytes([self.b, self.c]),
            Register16Id::DE => u16::from_be_bytes([self.d, self.e]),
            Register16Id::HL => u16::from_be_bytes([self.h, self.l]),
            Register16Id::SP => self.sp
        }
    }

    fn set(&mut self, reg: Register16Id, val: u16) {
        match reg {
            Register16Id::AF => {
                let [a, f] = u16::to_be_bytes(val);
                self.a = a;
                self.flags.set_f(f);
            }
            Register16Id::BC => [self.b, self.c] = u16::to_be_bytes(val),
            Register16Id::DE => [self.d, self.e] = u16::to_be_bytes(val),
            Register16Id::HL => [self.h, self.l] = u16::to_be_bytes(val),
            Register16Id::SP => self.sp = val
        };
    }
}

pub struct FlagRegister {
    /// Zero Flag
    pub z: bool,
    /// Subtract Flag
    pub n: bool,
    /// Half Carry Flag
    pub h: bool,
    // Carry Flag
    pub c: bool,
}

impl FlagRegister {
    fn shift(&self, bit: bool, left: u8) -> u8 {
        (if bit { 1 } else { 0 }) << left
    }

    pub fn get_f(&self) -> u8 {
        self.shift(self.z, 7) + self.shift(self.n, 6) + self.shift(self.h, 5) + self.shift(self.c, 4)
    }

    pub fn set_f(&mut self, val: u8) {
        self.z = val & 0b10000000 > 0;
        self.n = val & 0b01000000 > 0;
        self.h = val & 0b00100000 > 0;
        self.c = val & 0b00010000 > 0;
    }

    pub fn get(&self, id: &FlagId) -> bool {
        match id {
            FlagId::Z => self.z,
            FlagId::NZ => !self.z,
            FlagId::C => self.c,
            FlagId::NC => !self.c,
        }
    }
}

pub fn init_cpu() -> Cpu {
    Cpu {
        regs: Registers {
            a: 0x01,
            b: 0xFF,
            c: 0x13,
            d: 0x00,
            e: 0xC1,
            h: 0x84,
            l: 0x03,
            sp: 0xFFFE,
            pc: 0x0100,
            flags: FlagRegister {
                z: false,
                n: false,
                h: false,
                c: false,
            },
        },
    }
}

impl Cpu {
    pub fn run(self: &mut Cpu, mmu: &mut Mmu) {
        'execution: loop {
            let instr = Instruction::try_from((mmu[self.regs.pc], mmu[self.regs.pc + 1]))
                .unwrap_or_else(|_| {
                    if mmu[self.regs.pc] == 0xCB {
                        panic!("Unsupported opcode {:#04x} {:#04x}", mmu[self.regs.pc], mmu[self.regs.pc + 1])
                    } else {
                        panic!("Unsupported opcode {:#04x}", mmu[self.regs.pc])
                    }
                });

            if mmu[self.regs.pc] != 0x00 && false {
                println!("PC={:#06x}, SP={:#06x}, A={:#04x}, B={:#04x}, C={:#04x}, D={:#04x}, E={:#04x}, H={:#04x}, L={:#04x}, Z={}, N={}, H={}, C={}, opcode={:#04x} {:?}, ly={:#04x}",
                         self.regs.pc, self.regs.sp,
                         self.regs.a, self.regs.b, self.regs.c, self.regs.d, self.regs.e, self.regs.h, self.regs.l,
                         self.regs[Z], self.regs[N], self.regs[H], self.regs[C],
                         mmu[self.regs.pc], instr.mnemonic, mmu[0xFF44]
                );
            }

            if mmu[self.regs.pc] == 0xCB {
                self.advance_pc(2);
            } else {
                self.advance_pc(1);
            }

            match instr.kind {
                ADD => {
                    let n = self.get_operand(&instr.lhs.unwrap(), mmu);
                    let (add, carry) = calc_with_carry(vec![self.regs.a, n, 0], |a, b| a.overflowing_add(b));

                    self.regs[Z] = add == 0;
                    self.regs[N] = false;
                    self.regs[H] = half_carry_8_add(self.regs.a, n, 0);
                    self.regs[C] = carry;

                    self.regs.a = add;
                }
                ADD16 => {
                    if instr.mnemonic == "ADD SP,r8" {
                        let n = self.read_8(mmu) as i8 as i16 as u16;
                        let h = (self.regs.sp & 0x000F) + (n & 0x000F) > 0x000F;
                        let c = (self.regs.sp & 0x00FF) + (n & 0x00FF) > 0x00FF;

                        self.regs[Z] = false;
                        self.regs[N] = false;
                        self.regs[H] = h;
                        self.regs[C] = c;

                        self.regs.sp = self.regs.sp.overflowing_add(n).0;
                    } else {
                        let lhs = &instr.lhs.unwrap();
                        let rhs = &instr.rhs.unwrap();
                        let left = self.get_16bit_operand(lhs, mmu);
                        let right = self.get_16bit_operand(rhs, mmu);

                        let hc = half_carry_16_add(left, right, 0);
                        let result = left.overflowing_add(right);

                        self.regs[N] = false;
                        self.regs[H] = hc;
                        self.regs[C] = result.1;

                        self.set_16bit_value(mmu, lhs, result.0);
                    }
                }
                ADC => {
                    let carry = if self.regs[C] { 1 } else { 0 };
                    let n = self.get_operand(&instr.lhs.unwrap(), mmu);
                    let (add, new_carry) = calc_with_carry(vec![self.regs.a, n, carry], |a, b| a.overflowing_add(b));

                    self.regs[Z] = add == 0;
                    self.regs[N] = false;
                    self.regs[H] = half_carry_8_add(self.regs.a, n, carry);
                    self.regs[C] = new_carry;

                    self.regs.a = add;
                }
                AND => {
                    let n = self.get_operand(&instr.lhs.unwrap(), mmu);
                    self.regs.a = self.regs.a.bitand(n);
                    self.regs[Z] = self.regs.a == 0;
                    self.regs[N] = false;
                    self.regs[H] = true;
                    self.regs[C] = false;
                }
                BIT => {
                    let bit = self.get_operand(&instr.lhs.unwrap(), mmu);
                    let n = self.get_operand(&instr.rhs.unwrap(), mmu);

                    self.regs[Z] = n & (1 << bit) == 0;
                    self.regs[N] = false;
                    self.regs[H] = true;
                }
                CALL => {
                    let cond = match &instr.lhs.unwrap() {
                        Operand::Flag(flag) => self.regs.flags.get(flag),
                        _ => true
                    };
                    let addr = self.read_16(mmu);

                    if cond {
                        self.push_stack(self.regs.pc, mmu);
                        self.regs.pc = addr;
                    }
                }
                CCF => {
                    self.regs[C] = !self.regs[C];
                    self.regs[N] = false;
                    self.regs[H] = false;
                }
                CP => {
                    let n = self.get_operand(&instr.lhs.unwrap(), mmu);
                    self.regs[Z] = self.regs.a == n;
                    self.regs[N] = true;
                    self.regs[H] = half_carry_8_sub(self.regs.a, n, 0);
                    self.regs[C] = self.regs.a < n;
                }
                CPL => {
                    self.regs.a = !self.regs.a;
                    self.regs[N] = true;
                    self.regs[H] = true;
                }
                DAA => {
                    // Explanation at https://ehaskins.com/2018-01-30%20Z80%20DAA/
                    let lo = self.regs.a & 0x0F;
                    let mut added = 0;

                    if self.regs.flags.n {
                        // subtraction
                        if self.regs.flags.h {
                            added = 0x06;
                        }
                        if self.regs.flags.c {
                            added += 0x60;
                        }

                        self.regs.a = self.regs.a.wrapping_sub(added);
                    } else {
                        // addition
                        if self.regs.flags.h || lo > 0x9 {
                            added = 0x06;
                        }
                        if self.regs.flags.c || self.regs.a > 0x99 {
                            added += 0x60;
                        }

                        self.regs.a = self.regs.a.wrapping_add(added);
                    }

                    self.regs.flags.z = self.regs.a == 0;
                    self.regs.flags.h = false;
                    self.regs.flags.c = added >= 0x60;
                }
                DEC => {
                    let op = instr.lhs.unwrap();
                    match op {
                        Operand::Register16(reg) => self.regs.set(reg, self.regs.get(reg).wrapping_sub(1)),
                        Operand::IndirectAddress(Register16Id::HL) => {
                            let n = &mut mmu[self.regs.get(Register16Id::HL)];
                            let old = *n;
                            *n = (*n).wrapping_sub(1);

                            self.regs[Z] = *n == 0;
                            self.regs[N] = true;
                            self.regs[H] = half_carry_8_sub(old, 1, 0);
                        }
                        Operand::Register(reg) => {
                            let val = self.regs[reg];
                            self.regs[reg] = val.wrapping_sub(1);

                            self.regs[Z] = self.regs[reg] == 0;
                            self.regs[N] = true;
                            self.regs[H] = half_carry_8_sub(val, 1, 0);
                        }
                        _ => panic!("Operand not supported: {:?}", op),
                    }
                }
                DI => {} // TODO
                EI => {} // TODO
                INC => {
                    let reg = instr.lhs.unwrap();
                    match reg {
                        Operand::Register16(reg) => {
                            let n = self.regs.get(reg);
                            self.regs.set(reg, n.wrapping_add(1));
                        }
                        Operand::IndirectAddress(Register16Id::HL) => {
                            let n = &mut mmu[self.regs.get(Register16Id::HL)];
                            let old = *n;
                            *n = (*n).wrapping_add(1);

                            self.regs[Z] = *n == 0;
                            self.regs[N] = false;
                            self.regs[H] = half_carry_8_add(old, 1, 0);
                        }
                        Operand::Register(reg) => {
                            let n = self.regs[reg];
                            self.regs[reg] = n.wrapping_add(1);

                            self.regs[Z] = self.regs[reg] == 0;
                            self.regs[N] = false;
                            self.regs[H] = half_carry_8_add(n, 1, 0);
                        }
                        _ => panic!("Can't INC this register!")
                    }
                }
                JP => {
                    let lhs = &instr.lhs.unwrap();
                    let cond = match lhs {
                        Operand::Flag(flag) => self.regs.flags.get(flag),
                        _ => true
                    };

                    let addr = match lhs {
                        Operand::Flag(_) => self.get_16bit_operand( &instr.rhs.unwrap(), mmu),
                        _ => self.get_16bit_operand( lhs, mmu),
                    };
                    if cond {
                        self.set_pc(addr);
                    }
                }
                JR => {
                    let cond = match &instr.lhs.unwrap() {
                        Operand::Flag(flag) => self.regs.flags.get(flag),
                        _ => true
                    };

                    let offset = self.read_8(mmu);
                    if offset as i8 == -2 {
                        // JR loop, used by test ROMs to indicate end of tests
                        break 'execution;
                    }
                    if cond {
                        self.advance_pc(offset as i8 as i16);
                    }
                }
                LD => {
                    let value = self.get_operand(&instr.rhs.unwrap(), mmu);
                    self.set_value(mmu, &instr.lhs.unwrap(), value);
                }
                LD16 => {
                    let rhs = &instr.rhs.unwrap();


                    match rhs {
                        Operand::SpOffset => {
                            let sp = self.regs.sp;
                            let n = self.read_8(mmu) as i8 as i16 as u16;

                            self.regs[Z] = false;
                            self.regs[N] = false;
                            self.regs[H] = (sp & 0x000F) + (n & 0x000F) > 0x000F;
                            self.regs[C] = (sp & 0x00FF) + (n & 0x00FF) > 0x00FF;

                            self.set_16bit_value(mmu, &instr.lhs.unwrap(), sp.wrapping_add(n));
                        }
                        _ => {
                            let value = self.get_16bit_operand(rhs, mmu);
                            self.set_16bit_value(mmu, &instr.lhs.unwrap(), value);
                        }
                    }
                }
                LDD => {
                    let value = self.get_operand(&instr.rhs.unwrap(), mmu);
                    self.set_value(mmu, &instr.lhs.unwrap(), value);
                    self.regs.set(HL, self.regs.get(HL).wrapping_sub(1));
                }
                LDI => {
                    let value = self.get_operand(&instr.rhs.unwrap(), mmu);
                    self.set_value(mmu, &instr.lhs.unwrap(), value);
                    self.regs.set(HL, self.regs.get(HL).wrapping_add(1));
                }
                NOP => {}
                OR => {
                    let n = self.get_operand(&instr.lhs.unwrap(), mmu);
                    self.regs.a = self.regs.a | n;

                    self.regs[Z] = self.regs.a == 0;
                    self.regs[N] = false;
                    self.regs[H] = false;
                    self.regs[C] = false;
                }
                POP => {
                    let val = self.pop_stack(mmu);
                    self.set_16bit_value(mmu, &instr.lhs.unwrap(), val);
                }
                PUSH => {
                    let addr = self.get_16bit_operand(&instr.lhs.unwrap(), mmu);
                    self.push_stack(addr, mmu)
                },
                RES => {
                    let lhs = &instr.lhs.unwrap();
                    let rhs = &instr.rhs.unwrap();
                    let bit = self.get_operand(lhs, mmu);
                    let mut n = self.get_operand(rhs, mmu);
                    n = n & (0xFF ^ (1 << bit));
                    self.set_value(mmu, rhs, n);
                }
                RET => {
                    let cond = match instr.lhs {
                        Some(Operand::Flag(flag)) => self.regs.flags.get(&flag),
                        _ => true
                    };
                    if cond {
                        let addr = self.pop_stack(mmu);
                        self.set_pc(addr);
                    }
                }
                RETI => {
                    let addr = self.pop_stack(mmu);
                    self.set_pc(addr);
                    // TODO enable interrupts
                }
                RL => {
                    let lhs = &instr.lhs.unwrap();
                    let carry_bit = if self.regs[C] { 1 } else { 0 } as u8;
                    let val = self.get_operand(lhs, mmu);
                    let bit7 = val >> 7 != 0;
                    let rotated = val << 1 | carry_bit;
                    self.set_value(mmu, lhs, rotated);

                    self.regs[Z] = rotated == 0;
                    self.regs[N] = false;
                    self.regs[H] = false;
                    self.regs[C] = bit7;
                }
                RLA => {
                    let bit7 = self.regs.a >> 7 != 0;
                    let carry_bit = if self.regs[C] { 1 } else { 0 } as u8;
                    self.regs.a = self.regs.a << 1 | carry_bit;
                    self.regs[Z] = false;
                    self.regs[N] = false;
                    self.regs[H] = false;
                    self.regs[C] = bit7;
                }
                RLC => {
                    let lhs = &instr.lhs.unwrap();
                    let val = self.get_operand(lhs, mmu).rotate_left(1);
                    self.set_value(mmu, lhs, val);
                    self.regs[Z] = val == 0;
                    self.regs[N] = false;
                    self.regs[H] = false;
                    self.regs[C] = (val & 0x01) == 0x01;
                }
                RLCA => {
                    self.regs.a = self.regs.a.rotate_left(1);
                    self.regs[Z] = false;
                    self.regs[N] = false;
                    self.regs[H] = false;
                    self.regs[C] = self.regs.a & 0x01 != 0;
                }
                RR => {
                    let lhs = &instr.lhs.unwrap();
                    let carry_bit = if self.regs[C] { 1 } else { 0 } as u8;
                    let val = self.get_operand(lhs, mmu);
                    let bit0 = val & 0x01 != 0;
                    self.set_value(mmu, lhs, val >> 1 | carry_bit << 7);

                    self.regs[Z] = self.get_operand(lhs, mmu) == 0;
                    self.regs[N] = false;
                    self.regs[H] = false;
                    self.regs[C] = bit0;
                }
                RRA => {
                    let bit0 = self.regs.a & 0x01 != 0;
                    let carry_bit = if self.regs[C] { 1 } else { 0 } as u8;
                    self.regs.a = self.regs.a >> 1 | carry_bit << 7;
                    self.regs[Z] = false;
                    self.regs[N] = false;
                    self.regs[H] = false;
                    self.regs[C] = bit0;
                }
                RRC => {
                    let lhs = &instr.lhs.unwrap();
                    let val = self.get_operand(lhs, mmu).rotate_right(1);
                    self.set_value(mmu, lhs, val);
                    self.regs[Z] = val == 0;
                    self.regs[N] = false;
                    self.regs[H] = false;
                    self.regs[C] = (val & 0x80) == 0x80;
                }
                RRCA => {
                    self.regs.a = self.regs.a.rotate_right(1);
                    self.regs[Z] = false;
                    self.regs[N] = false;
                    self.regs[H] = false;
                    self.regs[C] = self.regs.a & 0x80 != 0;
                }
                RST => {
                    self.push_stack(self.regs.pc, mmu);
                    let offset = self.get_operand(&instr.lhs.unwrap(), mmu) as u16;
                    self.set_pc(0x0000 + offset);
                }
                SBC => {
                    let carry = if self.regs[C] { 1 } else { 0 };
                    let n = self.get_operand(&instr.lhs.unwrap(), mmu);

                    let (sub, new_carry) = calc_with_carry(vec![self.regs.a, n, carry], |a, b| a.overflowing_sub(b));

                    self.regs[Z] = sub == 0;
                    self.regs[N] = true;
                    self.regs[H] = half_carry_8_sub(self.regs.a, n, carry);
                    self.regs[C] = new_carry;

                    self.regs.a = sub;
                }
                SCF => {
                    self.regs[N] = false;
                    self.regs[H] = false;
                    self.regs[C] = true;
                }
                SET => {
                    let lhs = &instr.lhs.unwrap();
                    let rhs = &instr.rhs.unwrap();
                    let bit = self.get_operand(lhs, mmu);
                    let mut n = self.get_operand(rhs, mmu);
                    n = n | (1 << bit);
                    self.set_value(mmu, rhs, n);
                }
                SLA => {
                    let lhs = &instr.lhs.unwrap();
                    let val = self.get_operand(lhs, mmu);
                    let bit7 = val & 0x80 == 0x80;
                    let shifted = val << 1;

                    self.set_value(mmu, lhs, shifted);

                    self.regs[Z] = shifted == 0;
                    self.regs[N] = false;
                    self.regs[H] = false;
                    self.regs[C] = bit7;
                }
                SRA => {
                    let lhs = &instr.lhs.unwrap();
                    let val = self.get_operand(lhs, mmu);
                    let bit7 = val & 0x80;
                    let bit0 = val & 0x01 == 0x01;
                    let shifted = (val >> 1) | bit7;

                    self.set_value(mmu, lhs, shifted);

                    self.regs[Z] = shifted == 0;
                    self.regs[N] = false;
                    self.regs[H] = false;
                    self.regs[C] = bit0;
                }
                SRL => {
                    let lhs = &instr.lhs.unwrap();
                    let val = self.get_operand(lhs, mmu);
                    let bit0 = val & 0x01 == 0x01;
                    let shifted = val >> 1;

                    self.set_value(mmu, lhs, shifted);

                    self.regs[Z] = shifted == 0;
                    self.regs[N] = false;
                    self.regs[H] = false;
                    self.regs[C] = bit0;
                }
                STOP => break 'execution,
                SUB => {
                    let n = self.get_operand(&instr.lhs.unwrap(), mmu);
                    let (sub, carry) = calc_with_carry(vec![self.regs.a, n, 0], |a, b| a.overflowing_sub(b));

                    self.regs[Z] = sub == 0;
                    self.regs[N] = true;
                    self.regs[H] = half_carry_8_sub(self.regs.a, n, 0);
                    self.regs[C] = carry;

                    self.regs.a = sub;
                }
                SWAP => {
                    let lhs = &instr.lhs.unwrap();
                    let val = self.get_operand(lhs, mmu);
                    let swapped = ((val & 0x0F) << 4) | ((val & 0xF0) >> 4);

                    self.set_value(mmu, lhs, swapped);
                    self.regs[Z] = swapped == 0;
                    self.regs[N] = false;
                    self.regs[H] = false;
                    self.regs[C] = false;
                }
                XOR => {
                    let n = self.get_operand(&instr.lhs.unwrap(), mmu);
                    self.regs.a = self.regs.a.bitxor(n);
                    self.regs[Z] = self.regs.a == 0;
                    self.regs[N] = false;
                    self.regs[H] = false;
                    self.regs[C] = false;
                }
                _ => panic!("{:?}", instr.kind)
            }
        }
    }

    fn set_pc(self: &mut Cpu, addr: u16) {
        self.regs.pc = addr;
    }

    fn advance_pc(self: &mut Cpu, nb_bytes: i16) {
        self.regs.pc = (self.regs.pc as i16 + nb_bytes) as u16;
    }

    fn read_8(&mut self, mmu: &Mmu) -> u8 {
        let n = mmu[self.regs.pc];
        self.regs.pc += 1;
        n
    }

    fn read_16(&mut self, mmu: &Mmu) -> u16 {
        let n = u16::from_le_bytes([mmu[self.regs.pc], mmu[self.regs.pc + 1]]);
        self.regs.pc += 2;
        n
    }

    fn get_operand(&mut self, op: &Operand, mmu: &Mmu) -> u8 {
        match op {
            Operand::DirectAddress => {
                let addr = self.read_16(mmu);
                mmu[addr]
            }
            Operand::IndirectAddress(reg) => {
                mmu[self.regs.get(*reg)]
            }
            Operand::Byte => {
                self.read_8(mmu)
            }
            Operand::Register(reg) => self.regs[*reg],
            Operand::Value(val) => *val,
            Operand::IoPort(reg) => mmu[0xFF00 + self.regs[*reg] as u16],
            Operand::IoPortOffset => mmu[0xFF00 + self.read_8(mmu) as u16],
            _=> panic!("{:?}", op)
        }
    }

    fn get_16bit_operand(&mut self, op: &Operand, mmu: &Mmu) -> u16 {
        match op {
            Operand::Register16(reg) => self.regs.get(*reg),
            Operand::Byte => self.read_16(mmu),
            _=> panic!("{:?}", op)
        }
    }

    fn set_value(&mut self, mmu: &mut Mmu, op: &Operand, value: u8) {
        match op {
            Operand::Register(reg) => {
                self.regs[*reg] = value;
            }
            Operand::IndirectAddress(reg) => {
                mmu[self.regs.get(*reg)] = value;
            }
            Operand::DirectAddress => {
                let addr = self.read_16(mmu);
                mmu[addr] = value;
            }
            Operand::IoPort(reg) => mmu[0xFF00 + self.regs[*reg] as u16] = value,
            Operand::IoPortOffset => {
                let offset = self.read_8(mmu) as u16;
                mmu[0xFF00 + offset] = value;
            },
            _ => panic!("{:?}", op),
        }
    }

    fn set_16bit_value(&mut self, mmu: &mut Mmu, op: &Operand, value: u16) {
        match op {
            Operand::Register16(reg) => {
                self.regs.set(*reg, value);
            }
            Operand::Byte => {
                let addr = self.read_16(mmu);
                [mmu[addr], mmu[addr + 1]] = value.to_le_bytes();
            }
            _ => panic!("{:?}", op)
        }
    }

    fn push_stack(&mut self, val: u16, mmu: &mut Mmu) {
        let [lo, hi] = val.to_le_bytes();

        self.regs.sp -= 1;
        mmu[self.regs.sp] = hi;
        self.regs.sp -= 1;
        mmu[self.regs.sp] = lo;
    }
    fn pop_stack(&mut self, mmu: &mut Mmu) -> u16 {
        let lo = mmu[self.regs.sp];
        self.regs.sp += 1;
        let hi = mmu[self.regs.sp];
        self.regs.sp += 1;

        u16::from_le_bytes([lo, hi])
    }
}

fn calc_with_carry<T: Copy>(operands: Vec<T>, op: fn(T, T) -> (T, bool)) -> (T, bool) {
    let mut c = false;
    let mut acc = operands[0];
    for x in &operands[1..] {
        if !c {
            let res = op(acc, *x);
            acc = res.0;
            c = res.1;
        } else {
            acc = op(acc, *x).0
        }
    }
    (acc, c)
}

fn half_carry_8_add(a: u8, b: u8, c: u8) -> bool {
    (a & 0xF) + (b & 0xF) + c > 0xF
}

fn half_carry_8_sub(a: u8, b: u8, c: u8) -> bool {
    (a & 0x0F) < (b & 0x0F) + c
}

fn half_carry_16_add(a: u16, b: u16, c: u16) -> bool {
    (a & 0x07FF) + (b & 0x07FF) + c > 0x07FF
}
