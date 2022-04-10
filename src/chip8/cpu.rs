use super::memory::{read_from, write_into, Memory};
use byteorder::{BigEndian, ByteOrder, LittleEndian};

pub struct Cpu {
    memory: Memory,
    display: [u8; 64 * 32],
    program_counter: u16,
    index_register: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            display: [0; 64 * 32],
            program_counter: 0,
            index_register: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            va: 0,
            vb: 0,
            vc: 0,
            vd: 0,
            ve: 0,
            vf: 0,
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
