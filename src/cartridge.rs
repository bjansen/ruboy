use std::fs;

pub struct Cartridge {
    pub content: Vec<u8>,
}

impl Cartridge {
    pub fn new(path: &str) -> Cartridge {
        Cartridge {
            content: fs::read(path).expect("Couldn't read ROM")
        }
    }

    pub fn name(&self) -> String {
        String::from_utf8(self.content[0x0134..0x0142].to_vec())
            .expect("Couldn't read ROM name")
    }
}

impl Cartridge {}