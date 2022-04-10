use super::memory::{read_from, write_into, Memory};
use byteorder::{BigEndian, ByteOrder, LittleEndian};

pub struct Cpu {
    memory: Memory,
    program_counter: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            program_counter: 0,
        }
    }

    pub fn write_memory(&mut self, data: &[u8], location: u16) {
        write_into(&data, &mut self.memory, location);
    }

    pub fn read_memory(&self, location: u16) -> u8 {
        read_from(&self.memory, location)
    }
}

pub fn fetch(cpu: &mut Cpu) -> u16 {
    let begin = cpu.program_counter as usize;
    let end = begin + 2;
    let buf = &cpu.memory[begin..end];

    cpu.program_counter += 2;

    BigEndian::read_u16(buf)
}
