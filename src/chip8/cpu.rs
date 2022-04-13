const FOUR_KB: usize = 4096;
const REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;

pub struct Cpu {
    memory: [u8; FOUR_KB],
    stack: [u16; STACK_SIZE],
    pub display: [[u8; 64]; 32],
    registers: [u8; REGISTER_COUNT],
    index: u16,
    program_counter: u16,
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu {
            memory: [0; FOUR_KB],
            stack: [0; STACK_SIZE],
            display: [[0; 64]; 32],
            registers: [0; REGISTER_COUNT],
            index: 0,
            program_counter: 512,
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
        };

        cpu.load_ibm();

        cpu
    }

    pub fn cycle(&mut self) {
        let opcode = ((self.memory[self.program_counter as usize] as u16) << 8)
            | (self.memory[(self.program_counter as usize) + 1] as u16);
        self.program_counter += 2;

        let op: u8 = ((opcode & 0xF000) >> 12) as u8;
        let x: u8 = ((opcode & 0x0F00) >> 8) as u8;
        let y: u8 = ((opcode & 0x00F0) >> 4) as u8;
        let n: u8 = ((opcode & 0x000F) >> 0) as u8;

        let nnn: u16 = opcode & 0x0FFF;
        let nn: u8 = (opcode & 0x00FF) as u8;

        match (op, x, y, n) {
            (0x0, 0, 0xE, 0) => self.clear_screen(),
            (0x1, _, _, _) => self.jump(nnn),
            (0x6, _, _, _) => self.store(x, nn),
            (0x7, _, _, _) => self.add_to_register(x, nn),
            (0xA, _, _, _) => self.set_index_register(nnn),
            (0xD, _, _, _) => self.draw(x, y, n),
            _ => println!("No match"),
        }
    }

    fn clear_screen(&mut self) {
        //TODO
    }

    fn jump(&mut self, nnn: u16) {
        self.program_counter = nnn;
    }

    fn store(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] = nn;
    }

    fn add_to_register(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] += nn;
    }

    fn set_index_register(&mut self, nnn: u16) {
        self.index = nnn;
    }

    fn draw(&mut self, x: u8, y: u8, n: u8) {
        let ypos = self.registers[y as usize];
        let xpos = self.registers[x as usize];
        let sprite = &self.memory[self.index as usize..];
        for j in 0..n {
            for i in 0..8 {
                // screen wrap if necessary
                let y = (ypos + j) & 31;
                let x = (xpos + i) & 63;

                // draw each sprite pixel with a XOR operation
                // i.e. toggle the pixel
                // 0x80 = 1000 0000 : allows to check each pixel in the sprite
                if (sprite[j as usize] & (0x80 >> i)) != 0x00 {
                    self.display[y as usize][x as usize] ^= 0xAA;
                }
            }
        }
    }

    fn load_ibm(&mut self) {
        let ibm_logo: [u8; 132] = [
            0x00, 0xE0, 0xA2, 0x2A, 0x60, 0x0C, 0x61, 0x08, 0xD0, 0x1F, 0x70, 0x09, 0xA2, 0x39,
            0xD0, 0x1F, 0xA2, 0x48, 0x70, 0x08, 0xD0, 0x1F, 0x70, 0x04, 0xA2, 0x57, 0xD0, 0x1F,
            0x70, 0x08, 0xA2, 0x66, 0xD0, 0x1F, 0x70, 0x08, 0xA2, 0x75, 0xD0, 0x1F, 0x12, 0x28,
            0xFF, 0x00, 0xFF, 0x00, 0x3C, 0x00, 0x3C, 0x00, 0x3C, 0x00, 0x3C, 0x00, 0xFF, 0x00,
            0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x38, 0x00, 0x3F, 0x00, 0x3F, 0x00, 0x38, 0x00, 0xFF,
            0x00, 0xFF, 0x80, 0x00, 0xE0, 0x00, 0xE0, 0x00, 0x80, 0x00, 0x80, 0x00, 0xE0, 0x00,
            0xE0, 0x00, 0x80, 0xF8, 0x00, 0xFC, 0x00, 0x3E, 0x00, 0x3F, 0x00, 0x3B, 0x00, 0x39,
            0x00, 0xF8, 0x00, 0xF8, 0x03, 0x00, 0x07, 0x00, 0x0F, 0x00, 0xBF, 0x00, 0xFB, 0x00,
            0xF3, 0x00, 0xE3, 0x00, 0x43, 0xE0, 0x00, 0xE0, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80,
            0x00, 0x80, 0x00, 0xE0, 0x00, 0xE0,
        ];

        self.memory[512..644].copy_from_slice(&ibm_logo);
    }

    /*
     */
}
