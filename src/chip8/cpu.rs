const FOUR_KB: usize = 4096;
const REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const DISPLAY_SIZE: usize = 64 * 32;
pub struct Cpu {
    memory: [u8; FOUR_KB],
    stack: [u16; STACK_SIZE],
    display: [u8; DISPLAY_SIZE],
    registers: [u8; REGISTER_COUNT],
    index: u16,
    program_counter: u16,
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            memory: [0; FOUR_KB],
            stack: [0; STACK_SIZE],
            display: [0; DISPLAY_SIZE],
            registers: [0; REGISTER_COUNT],
            index: 0,
            program_counter: 0,
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}
