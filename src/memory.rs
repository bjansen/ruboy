use std::ops::{Index, IndexMut};

use crate::cartridge::Cartridge;

pub struct Mmu {
    internal_ram: Vec<u8>,
    io_ports: Vec<u8>,
    internal_8kb_ram: Vec<u8>,
    switchable_ram: Vec<u8>,
    video_ram: Vec<u8>,
    cartridge: Cartridge,
}

impl Mmu {
    pub fn new(cart: Cartridge) -> Mmu {
        let mut mmu = Mmu {
            internal_ram: vec![0; 0xFFFF - 0xFF80 + 1],
            io_ports: vec![0; 0xFF4C - 0xFF00],
            internal_8kb_ram: vec![0; 0xE000 - 0xC000],
            switchable_ram: vec![0; 0xC000 - 0xA000],
            video_ram: vec![0; 0xA000 - 0x8000],
            cartridge: cart,
        };

        Self::init_io_ports(&mut mmu);

        mmu
    }

    fn init_io_ports(mmu: &mut Mmu) {
        mmu[0xFF00] = 0xCF;
        mmu[0xFF01] = 0x00;
        mmu[0xFF02] = 0x7E;
        mmu[0xFF04] = 0x18;
        mmu[0xFF05] = 0x00;
        mmu[0xFF06] = 0x00;
        mmu[0xFF07] = 0xF8;
        mmu[0xFF10] = 0x80;
        mmu[0xFF11] = 0xBF;
        mmu[0xFF12] = 0xF3;
        mmu[0xFF14] = 0xBF;
        mmu[0xFF16] = 0x3F;
        mmu[0xFF17] = 0x00;
        mmu[0xFF19] = 0xBF;
        mmu[0xFF1A] = 0x7F;
        mmu[0xFF1B] = 0xFF;
        mmu[0xFF1C] = 0x9F;
        mmu[0xFF1E] = 0xBF;
        mmu[0xFF20] = 0xFF;
        mmu[0xFF21] = 0x00;
        mmu[0xFF22] = 0x00;
        mmu[0xFF23] = 0xBF;
        mmu[0xFF24] = 0x77;
        mmu[0xFF25] = 0xF3;
        mmu[0xFF26] = 0xF1;
        mmu[0xFF40] = 0x91;
        mmu[0xFF42] = 0x00;
        mmu[0xFF43] = 0x00;
        mmu[0xFF45] = 0x00;
        mmu[0xFF47] = 0xFC;
        mmu[0xFF48] = 0xFF;
        mmu[0xFF49] = 0xFF;
        mmu[0xFF4A] = 0x00;
        mmu[0xFF4B] = 0x00;
    }
}

impl Index<u16> for Mmu {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        if index < 0x8000 {
            return &self.cartridge.content[index as usize];
        } else if index < 0xA000 {
            return &self.video_ram[(index - 0x8000) as usize];
        } else if index < 0xC000 {
            return &self.switchable_ram[(index - 0xA000) as usize];
        } else if index < 0xE000 {
            return &self.internal_8kb_ram[(index - 0xC000) as usize];
        } else if index < 0xFE00 {
            // echo of internal RAM
            return &self.internal_8kb_ram[(index - 0xE000) as usize];
        }
        if index >= 0xFF00 && index < 0xFF4C {
            return &self.io_ports[(index - 0xFF00) as usize];
        }
        if index >= 0xFF80 {
            return &self.internal_ram[(index - 0xFF80) as usize];
        }

        panic!("Memory address not supported: {:#06x}", index);
    }
}

impl IndexMut<u16> for Mmu {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        if index < 0x8000 {
            return &mut self.cartridge.content[index as usize];
        } else if index < 0xA000 {
            return &mut self.video_ram[(index - 0x8000) as usize];
        } else if index < 0xC000 {
            return &mut self.switchable_ram[(index - 0xA000) as usize];
        } else if index < 0xE000 {
            return &mut self.internal_8kb_ram[(index - 0xC000) as usize];
        } else if index < 0xFE00 {
            // echo of internal RAM
            return &mut self.internal_8kb_ram[(index - 0xE000) as usize];
        }
        if index >= 0xFF00 && index < 0xFF4C {
            return &mut self.io_ports[(index - 0xFF00) as usize];
        }
        if index >= 0xFF80 {
            return &mut self.internal_ram[(index - 0xFF80) as usize];
        }

        panic!("Memory address not supported: {:#06x}", index);
    }
}