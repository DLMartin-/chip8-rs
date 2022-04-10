use byteorder::{BigEndian, ByteOrder, LittleEndian};
mod chip8;
use chip8::cpu::{fetch, Cpu};

fn main() {
    let bytes: [u8; 2] = [0xff, 0x00];
    let big = BigEndian::read_u16(&bytes);
    let little = LittleEndian::read_u16(&bytes);

    let mut cpu = Cpu::new();
    cpu.write_memory(&[0xFF, 0x00], 0);

    let d = fetch(&mut cpu);
    let d2 = fetch(&mut cpu);

    println!("Hello World");
    println!("Hello World: {}", d);
    println!("Hello World: {}", d2);
}
