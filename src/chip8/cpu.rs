const FOUR_KB: usize = 4096;
const REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;

pub struct Cpu {
    memory: [u8; FOUR_KB],
    stack: [u16; STACK_SIZE],
    //pub display: [[u8; 64]; 32],
    pub display: [u8; 32 * 64],
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
            display: [0; 64 * 32],
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
            (0x0, 0x0, 0xE, 0x0) => self.clear_screen(),
            (0x0, 0x0, 0xE, 0xE) => self.ret(),
            (0x1, _, _, _) => self.jump(nnn),
            (0x2, _, _, _) => self.call(nnn),
            (0x3, _, _, _) => self.skip_if_equal(x, nn),
            (0x4, _, _, _) => self.skip_if_not_equal(x, nn),
            (0x5, _, _, 0x0) => self.skip_if_registers_equal(x, y),
            (0x6, _, _, _) => self.store(x, nn),
            (0x7, _, _, _) => self.add_to_register(x, nn),
            (0x8, _, _, _) => panic!("ALU not implemented"),
            (0x9, _, _, 0x0) => self.skip_if_registers_not_equal(x, y),
            (0xA, _, _, _) => self.set_index_register(nnn),
            (0xB, _, _, _) => panic!("Jump with Offset not implemented"),
            (0xC, _, _, _) => panic!("Random not yet implemted"),
            (0xD, _, _, _) => self.draw(x, y, n),
            (0xE, _, _, _) => panic!("Keypad not yet implemented"),
            (0xF, _, 0x0, 0x7) => panic!("Timers not implemented"),
            (0xF, _, 0x1, 0x5) => panic!("Timers not implemented"),
            (0xF, _, 0x1, 0x8) => panic!("Timers not implemented"),
            (0xF, _, 0x1, 0xE) => panic!("Add To Index not implemented"),
            (0xF, _, 0x0, 0xA) => panic!("Get Key not implemented"),
            (0xF, _, 0x2, 0x9) => panic!("Font Character not implemented"),
            (0xF, _, 0x3, 0x3) => panic!("Decimal Converstion not implemented"),
            (0xF, _, 0x5, 0x5) => panic!("Store Memory not implemented"),
            (0xF, _, 0x6, 0x5) => panic!("Load Memory not implemented"),
            _ => panic!("Invalid Operation!"),
        }
    }

    fn clear_screen(&mut self) {
        self.display.fill(0);
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
                    self.display[y as usize * 64 + x as usize] ^= 0x1;
                }
            }
        }
    }

    fn call(&mut self, nnn: u16) {
        if self.stack_pointer as usize == self.stack.len() {
            panic!("Error: Stack Pointer == maximum size of stack!");
        }

        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer += 1;
        self.program_counter = nnn;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Error: Stack is empty!")
        }

        self.program_counter = self.stack[self.stack_pointer as usize];
        self.stack_pointer -= 1;
    }

    fn skip_if_equal(&mut self, x: u8, nn: u8) {
        let vx = self.registers[x as usize];
        if (vx == nn) {
            self.program_counter += 2;
        }
    }

    fn skip_if_not_equal(&mut self, x: u8, nn: u8) {
        let vx = self.registers[x as usize];
        if (vx != nn) {
            self.program_counter += 2;
        }
    }

    fn skip_if_registers_equal(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        if (vx == vy) {
            self.program_counter += 2;
        }
    }

    fn skip_if_registers_not_equal(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        if (vx != vy) {
            self.program_counter += 2;
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
}
