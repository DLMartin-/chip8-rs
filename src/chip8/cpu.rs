use rand::Rng;

const FOUR_KB: usize = 4096;
const REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const DISPLAY_SIZE: usize = 64 * 32 * 4; // 64 pixels wide x 32 pixels tall, 1 pixel = 4 bytes

pub struct Cpu {
    memory: [u8; FOUR_KB],
    stack: [u16; STACK_SIZE],
    pub display: [u32; DISPLAY_SIZE],
    registers: [u8; REGISTER_COUNT],
    index: u16,
    program_counter: u16,
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
    keys: [bool; 16],
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu {
            memory: [0; FOUR_KB],
            stack: [0; STACK_SIZE],
            display: [0; DISPLAY_SIZE],
            registers: [0; REGISTER_COUNT],
            index: 0,
            program_counter: 512,
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            keys: [false; 16],
        };

        cpu.load_ibm();
        cpu.memory[..80].copy_from_slice(&FONT_SET);

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
            (0x8, _, _, 0x0) => self.registers[x as usize] = self.registers[y as usize],
            (0x8, _, _, 0x1) => self.registers[x as usize] |= self.registers[y as usize],
            (0x8, _, _, 0x2) => self.registers[x as usize] &= self.registers[y as usize],
            (0x8, _, _, 0x3) => self.registers[x as usize] ^= self.registers[y as usize],
            (0x8, _, _, 0x4) => {
                let value: u16 =
                    self.registers[x as usize] as u16 + self.registers[y as usize] as u16;
                self.registers[0xF] = 0;
                self.registers[x as usize] = (value & 0xFF) as u8;
                if value > 0xFF {
                    self.registers[0xF] = 1;
                }
            }
            (0x8, _, _, 0x5) => {
                let x = self.registers[x as usize];
                let y = self.registers[y as usize];
                self.registers[0xF] = 1;
                if x < y {
                    self.registers[0xF] = 0;
                }

                self.registers[x as usize] = x - y;
            }
            (0x8, _, _, 0x6) => {
                let value = self.registers[y as usize];
                self.registers[0xF] = 0b0000_0001 & value;
                self.registers[x as usize] = value >> 1;
            }
            (0x8, _, _, 0x7) => {
                let x = self.registers[x as usize];
                let y = self.registers[y as usize];
                self.registers[0xF] = 1;
                if y < x {
                    self.registers[0xF] = 0;
                }

                self.registers[x as usize] = y - x;
            }
            (0x8, _, _, 0xE) => {
                let value = self.registers[y as usize];
                self.registers[0xF] = 0b1000_0000 & value;
                self.registers[x as usize] = value << 1;
            }
            (0x9, _, _, 0x0) => self.skip_if_registers_not_equal(x, y),
            (0xA, _, _, _) => self.set_index_register(nnn),
            (0xB, _, _, _) => self.jump_with_offset(nnn),
            (0xC, _, _, _) => self.randomize_register(x, nn),
            (0xD, _, _, _) => self.draw(x, y, n),
            (0xE, _, 0x9, 0xE) => {
                let x = self.registers[x as usize];
                if self.keys[x as usize] == true {
                    self.program_counter += 2;
                }
            }
            (0xE, _, 0xA, 0x1) => {
                let x = self.registers[x as usize];
                if self.keys[x as usize] == false {
                    self.program_counter += 2;
                }
            }
            (0xF, _, 0x0, 0x7) => self.set_register_to_delay_timer(x),
            (0xF, _, 0x1, 0x5) => self.set_delay_timer_to_register(x),
            (0xF, _, 0x1, 0x8) => self.set_sound_timer_to_register(x),
            (0xF, _, 0x1, 0xE) => self.add_to_index(x),
            (0xF, _, 0x0, 0xA) => self.get_key(x),
            (0xF, _, 0x2, 0x9) => self.get_font_character(x),
            (0xF, _, 0x3, 0x3) => self.binary_coded_decimal_conversion(x),
            (0xF, _, 0x5, 0x5) => self.store_registers(x),
            (0xF, _, 0x6, 0x5) => self.load_registers(x),
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
                    let idx = (y as usize * 64) + x as usize;
                    self.display[idx] = 0x00aa6600;
                    //Look into u32::to_le_bytes (or be? idk if its little endian or big endian :))
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
        if vx != nn {
            self.program_counter += 2;
        }
    }

    fn skip_if_registers_equal(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        if vx == vy {
            self.program_counter += 2;
        }
    }

    fn skip_if_registers_not_equal(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        if vx != vy {
            self.program_counter += 2;
        }
    }

    fn set_register_to_delay_timer(&mut self, x: u8) {
        self.registers[x as usize] = self.delay_timer;
    }

    fn set_delay_timer_to_register(&mut self, x: u8) {
        self.delay_timer = self.registers[x as usize];
    }

    fn set_sound_timer_to_register(&mut self, x: u8) {
        self.sound_timer = self.registers[x as usize];
    }

    fn add_to_index(&mut self, x: u8) {
        self.index += 8;
        if self.index > 0x0FFF {
            self.index = self.index & 0x0FFF;
            self.registers[0xF] = 1;
        }
    }

    fn get_key(&mut self, _x: u8) {
        //TODO: Blocks until key is pressed
        //Then puts the value of the key pressed into register VX
        self.program_counter -= 2;
    }

    fn get_font_character(&mut self, x: u8) {
        //Index register is set to the address of the characeter in VX.
        //VX can hold two hex characters, so use the last nibble (0x0F)
        let vx = self.registers[x as usize];
        let character_index = ((vx & 0x0F) * 5) as u16;
        self.index = character_index;
    }

    fn binary_coded_decimal_conversion(&mut self, x: u8) {
        /*
        It takes the number in VX (which is one byte, so it can be any number from 0 to 255)
        and converts it to three decimal digits, storing these digits in memory at the address
        in the index register I. For example, if VX contains 156 (or 9C in hexadecimal),
        it would put the number 1 at the address in I, 5 in address I + 1,
        and 6 in address I + 2.
         */

        let index = self.index as usize;
        let vx = self.registers[x as usize];
        self.memory[index] = vx / 100;
        self.memory[index + 1] = (vx / 10) % 10;
        self.memory[index + 2] = vx % 10;
    }

    fn store_registers(&mut self, x: u8) {
        let mut slice = &mut self.memory[(self.index as usize)..(self.index as usize + x as usize)];
        slice.copy_from_slice(&self.registers[0..(x as usize)]);
    }

    fn load_registers(&mut self, x: u8) {
        let mut slice = &mut self.registers[0..(x as usize)];
        slice.copy_from_slice(
            &self.memory[(self.index as usize)..(self.index as usize + x as usize)],
        );
    }

    fn randomize_register(&mut self, x: u8, nn: u8) {
        let random_number: u8 = rand::thread_rng().gen::<u8>();
        let random_value = random_number & nn;
        self.registers[x as usize] = random_value;
    }

    // Later versions of BNNN were actually BXNN,
    // Where it would add nn and the value in V[x]
    // And jump to that location.
    fn jump_with_offset(&mut self, nnn: u16) {
        let jump_location = nnn + self.registers[0] as u16;
        self.index = jump_location;
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

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
