/// ```rust
/// assert_eq!(2, add(2,3));
/// ```

extern crate rand;

use rand::Rng;

struct Stack {
    stack: [u16; 16],
    pointer: u16,
}

fn merge_opcodes(first: u8, second: u8) -> u16 {
    let f = first as u16;
    let s = second as u16;
    let r = (f << 8) | s;

    //println!("0x{:x} 0b{:b}", r, r);

    r
}

fn main() {
    let opcode: u16;
    let memory: [u8; 4096];
    let cpu_register: [u8; 16];
    let index: u16;
    let program_counter: u16;

    // Stack stores the program counter when calling a subroutine
    let stack: [u16; 16];
    let stack_pointer: u16;
}

pub struct CPU {
    pub memory: [u8; 4096],
    pub program_counter: u16,
    pub stack: [u16; 16],
    pub stack_pointer: u16,
    pub cpu_register: [u8; 16],
    pub opcode: u16,
    pub index: u16,
    pub pixels: [u8; 2048],
    pub key: [bool; 16],

    pub draw_flag: bool,
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: [0; 4096],
            program_counter: 0x200,
            stack: [0; 16],
            stack_pointer: 0,
            cpu_register: [0; 16],
            opcode: 0,
            index: 0,
            pixels: [0; 2048],
            key: [false; 16],
            draw_flag: false,
            delay_timer: 0,
            sound_timer: 0,


        }
    }

    pub fn emulate_cycle(&mut self) {
        self.draw_flag = false;
        // Fetch opcode
        // pc will point to memory, where to pick up opcode
        println!("Program counter: {}", self.program_counter);
        let opcode = CPU::fetch_opcode(self);

        // Decode opcode
        // Execute opcode
        CPU::decode_opcode(self, opcode);




        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
//        println!("Delay timer is {}", self.delay_timer);

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn fetch_opcode(&self) -> u16 {
        // Inputs: pc, memory
        // Output: opcode
        //
        // Use pc and pc + 1, merge
        let counter = self.program_counter as usize;

        let r = merge_opcodes(self.memory[counter], self.memory[counter + 1]);

        r
    }

    // Input is a 2-byte opcode
    // We do everything in hexadecimal -> 0xffff
    pub fn decode_opcode(&mut self, opcode: u16) {
        println!("Opcode is 0x{:x}", opcode);

        // Read out first half byte
        match opcode & 0xF000 {

            0x0000 => {
                match opcode & 0x00FF {
                    0x00E0 => {
                        for mut pixel in self.pixels.iter() {
                            pixel = &0;
                        }
                        self.draw_flag = true;
                        self.program_counter += 2;
                    }

                    0x00EE => {
                        self.stack_pointer -= 1;
                        self.program_counter = self.stack[self.stack_pointer as usize];
                        self.program_counter += 2;

                    }

                    // Known opcode here that should not be implemented
                    _ => println!("Unknown opcode {}", opcode)
                }
            }

            // Jump to address NNN
            0x1000 => {
                // Assuming that this means a simple GOTO, so set the PC to NNN
                self.program_counter = opcode & 0xFFF;
            }

            // Call subroutine at NNN
            0x2000 => {
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer += 1;
                self.program_counter = opcode & 0x0FFF;
            }

            // Skip the next instruction if VX == NN
            0x3000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0xFF) as u8;

                if self.cpu_register[x] == nn {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }

            // Skip the next instruction if VX != NN
            0x4000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0xFF) as u8;

                if self.cpu_register[x] != nn {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }

            // Skip the next instruction iv VX == VY
            0x5000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;

                if x == y {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }

            // 6XNN sets VX to NN
            0x6000 => {
                //println!("0x{:x}", opcode & 0x0FFF);
                let x = ((opcode & 0x0F00) >> 8) as usize;
                //println!("V: 0x{:x}", x);

                let val = (opcode & 0x00FF) as u8;
                //println!("Value: 0x{:x}", val);
                self.cpu_register[x] = val;
                self.program_counter += 2;
            },

            // Adds NN to VX, no carry flag change
            0x7000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u16;

                let sum = nn + self.cpu_register[x] as u16;

                self.cpu_register[x] = (sum & 0x00FF) as u8;
                self.program_counter += 2;
            }

            // Mathematical operator block identified on last half byte
            // 0x_XY_
            0x8000 => {
                // X and Y can easily be set higher up. Read out
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                match opcode & 0x000F {
                    // Assign VX to value of VY
                    0x0000 => {
                        self.cpu_register[x] = self.cpu_register[y];
                        self.program_counter += 2;
                    }

                    // Set VX to bit VX | VY
                    0x0001 => {
                        self.cpu_register[x] = self.cpu_register[x] | self.cpu_register[y];
                        self.program_counter += 2;
                    }

                    // Set VX to bit VX & VY
                    0x0002 => {
                        self.cpu_register[x] = self.cpu_register[x] & self.cpu_register[y];
                        self.program_counter += 2;
                    }

                    // Set VX to VX xor VY
                    0x0003 => {
                        self.cpu_register[x] = self.cpu_register[x] ^ self.cpu_register[y];
                        self.program_counter += 2;

                    }

                    // Add VY to VX, carry 1 on VF if needed
                    0x0004 => {
                        let sum = self.cpu_register[x] as u16 + self.cpu_register[y] as u16;

                        // If overflow, set V[F] to 1
                        if sum & 0xF00 == 0x100 {
                            self.cpu_register[0xF] = 1;
                        }

                        self.cpu_register[x] = (sum & 0xFF) as u8;
                        self.program_counter += 2;
                    }

                    // Subtract VY from VX, borrow 1 from VF if needed
                    0x0005 => {

                        if self.cpu_register[x] >= self.cpu_register[y] {
                            self.cpu_register[x] = self.cpu_register[x] - self.cpu_register[y];
                        } else {

                            // Does behaviour depend on if we can borrow or not? Not clear...
                            let borrowed_diff = 0x100 + self.cpu_register[x] as u16 - self.cpu_register[y] as u16;
                            // Assert that borrowed diff & 0xF00 is always 0?
                            self.cpu_register[x] = (borrowed_diff & 0xFF) as u8;
                            self.cpu_register[0xF] = 0;

                        }
                        self.program_counter += 2;

                    }

                    // Store least significant bit of VX in VF, then shift VX to right by 1
                    0x0006 => {
                        self.cpu_register[0xF] = self.cpu_register[x] & 0b1;
                        self.cpu_register[x] = self.cpu_register[x] >> 1;
                        self.program_counter += 2;
                    }

                    // Set VX = VY - VX. If borrow, VF = 0 else = 1
                    0x0007 => {
                        if self.cpu_register[y] >= self.cpu_register[x] {
                            self.cpu_register[x] = self.cpu_register[y] - self.cpu_register[x];
                        } else {

                            let borrowed_diff = 0x100 + self.cpu_register[y] as u16 - self.cpu_register[x] as u16;

                            self.cpu_register[x] = (borrowed_diff & 0xFF) as u8;
                            self.cpu_register[0xF] = 0;
                        }
                        self.program_counter += 2;

                    }

                    // Store most significant bit of VX in VF, then shift VX to left by 1
                    0x000E => {
                        self.cpu_register[0xF] = (self.cpu_register[x] & 0b10000000) >> 7;
                        self.cpu_register[x] = self.cpu_register[x] << 1; // Is this ok though? Bit shift smart enough to not overflow?
                        self.program_counter += 2;
                    }


                    _ => println!("abort"),

                }

            }

            // Skip the next instruction if VX != VY
            0x9000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;

                if x != y {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }

            // Set I to address NNN
            0xA000 => {
                self.index = opcode & 0x0FFF;
                println!("Index is now {}", self.index);
                self.program_counter += 2;
            }

            // Jump to address NNN + V0
            0xB000 => {
                // GOTO with flavour
                self.program_counter = (opcode & 0x0FFF) + self.cpu_register[0] as u16;
            }

            // Set VX to random(0-255) & NN (0xCXNN)
            0xC000 => {
                let mut rng = rand::thread_rng();
                let r = rng.gen_range(0, 256) as u8;
                let x = ((opcode & 0x0F00) >> 8) as usize;

                self.cpu_register[x] = r & (opcode & 0xFF) as u8;
                self.program_counter += 2;
            }

            // Draw sprite at VX, VY of dimension 8xN pixels.
            // Sprite details are loaded from memory at I + row count
            // If any pixel is flipped, VF is set to 1 and 0 if not flipped. (0xDXYN)
            0xD000 => {
                let x_val = self.cpu_register[((opcode & 0x0F00) >> 8) as usize] as usize;
                let y_val = self.cpu_register[((opcode & 0x00F0) >> 4) as usize] as usize;
                let height = (opcode & 0x000F) as usize;
                let width = 8 as usize;
                println!("X: {}", x_val);
                println!("Y: {}", y_val);

                self.cpu_register[0xF] = 0;
                for i in 0..height {
                    let sprite_row = self.memory[self.index as usize + i];
//                    println!("From memory {}", sprite_row);
//                    println!("Index {}", self.index);

                    // Read out each pixel in the sprite from memory and check if it's set
                    for j in 0..width {
                        let sprite_x = sprite_row & (0x80 >> j);

                        // Check if set
                        if sprite_x != 0 {
                            let index_in_gfx_array = x_val + j + (y_val + i) * 64;
                            println!("Index: {}", index_in_gfx_array);
                            println!("Yval: {}", y_val);
                            if self.pixels[index_in_gfx_array] == 1 {
                                self.cpu_register[0xF] = 1;
                            }

                            // XOR on pixel if we get this far
                            self.pixels[index_in_gfx_array] ^= 1;
                        }
                    }

                }

                self.draw_flag = true;
                self.program_counter += 2;
            }

            0xE000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;

                match opcode & 0x00FF {

                    // Skip next instruction if key stored in VX is pressed
                    0x009E => {
                        println!("Awaiting key press {}", self.cpu_register[x]);
                        println!("Now {}", self.key[self.cpu_register[x] as usize]);
                        if self.key[self.cpu_register[x] as usize] {
                            self.program_counter += 2;
                        }

                        self.program_counter += 2;
                    }

                    // Skip next instruction if the key stored in VX is not pressed
                    0x00A1 => {
                        println!("Awaiting key press {}", self.cpu_register[x]);
                        println!("Now {}", self.key[self.cpu_register[x] as usize]);
                        if !self.key[self.cpu_register[x] as usize]{
                            self.program_counter += 2;
                        }

                        self.program_counter += 2;
                    }

                    _ => println!("Unknown opcode {}", opcode)

                }

            }

            0xF000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                match opcode & 0x00FF {

                    // Set VX to value of the delay timer (0xFX07)
                    0x0007 => {
                        self.cpu_register[x] = self.delay_timer;
                        self.program_counter += 2;
                    }

                    // Await key press and store in VX (0xFX0A)
                    0x000A => {
                        println!("HIT THE KEY AWAIT, THIS SHOULD WAIT");
                        for (i, key) in self.key.iter().enumerate() {
                            if *key == true {
                                self.cpu_register[x] = i as u8;
                                self.program_counter += 2;
                            }


                        }

                    }

                    // Set delay timer to VX (0xFX15)
                    0x0015 => {
                        self.delay_timer = self.cpu_register[x];
                        self.program_counter += 2;
                    }

                    // Set sound timer to VX (0xFX18)
                    0x0018 => {
                        self.sound_timer = self.cpu_register[x];
                        self.program_counter += 2;
                    }

                    // Add VX to I (0xFX1E)
                    0x001E => {
                        self.index += self.cpu_register[x] as u16;
                        self.program_counter += 2;
                    }

                    // Set I to location of sprite for character in VX? (0xFX29)
                    0x0029 => {
                        self.index = self.cpu_register[x] as u16 * 5;
                        println!("Character access {}", self.index);
                        self.program_counter += 2;
                    }

                    // Store the binary rep of VX on I, I+1, I+2 (0xFX33)
                    0x0033 => {
                        println!("Funky binary access at {}", self.cpu_register[x]);
                        let hundred = self.cpu_register[x] / 100;
                        let ten = (self.cpu_register[x] / 10) % 10;
                        let one = (self.cpu_register[x] % 100) % 10;

                        self.memory[self.index as usize] = hundred;
                        self.memory[self.index as usize + 1] = ten;
                        self.memory[self.index as usize + 2] = one;


                        self.program_counter += 2;
                    }

                    // Store V0 to VX in memory, starting at I (0xFX55)
                    0x0055 => {
                        for i in 0..=x {
                            self.memory[self.index as usize + i] = self.cpu_register[i]
                        }
                        self.program_counter += 2;
                    }

                    // Fill V0 to VX from memory, starting at I (0xFX65)
                    0x0065 => {
                        for i in 0..=x {
                            self.cpu_register[i] = self.memory[self.index as usize + i];
                        }
                        self.program_counter += 2;

                    }

                    _ => println!("Unknown opcode {}", opcode)
                }




            }

            _ => println!("Opcode {} not implemented yet", opcode),

        }

    }

    pub fn read_rom(&mut self, path: String) {


    }

    pub fn clear_keys(&mut self) {
        for i in 0..self.key.len() {
            self.key[i] = false;
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_opcodes() {
        assert_eq!(merge_opcodes(0x1A, 0xB2), 0x1AB2);
    }

    #[test]
    fn test_execute_opcode_6XNN() {
        let mut cpu = CPU::new();

        assert_eq!(cpu.cpu_register[0x2], 0x00);

        // How did this compile
        //CPU::decode_opcode(&mut cpu,0x6211);
        cpu.decode_opcode(0x6211);
        assert_eq!(cpu.cpu_register[0x2], 0x11);
    }

    #[test]
    fn test_execute_opcode_7XNN() {
        let mut cpu = CPU::new();

        cpu.cpu_register[3] = 0x1;
        assert_eq!(cpu.cpu_register[0x3], 0x01);

        cpu.decode_opcode(0x7322);
        assert_eq!(cpu.cpu_register[0x3], 0x23);
    }

    #[test]
    fn test_execute_opcode_8XY0() {
        let mut cpu = CPU::new();

        cpu.cpu_register[3] = 0xFA;
        assert_eq!(cpu.cpu_register[0x5], 0);

        cpu.decode_opcode(0x8530);
        assert_eq!(cpu.cpu_register[0x5], 0xFA);
    }

    #[test]
    fn test_execute_opcode_8XY1() {
        let mut cpu = CPU::new();

        cpu.cpu_register[1] = 0x0F; // X
        cpu.cpu_register[2] = 0xF0; // Y

        cpu.decode_opcode(0x8121);
        assert_eq!(cpu.cpu_register[0x1], 0xFF);
    }

    #[test]
    fn test_execute_opcode_8XY2() {
        let mut cpu = CPU::new();

        cpu.cpu_register[1] = 0x0F; // X
        cpu.cpu_register[2] = 0xF1; // Y

        cpu.decode_opcode(0x8122);
        assert_eq!(cpu.cpu_register[0x1], 0x01);
    }

    #[test]
    fn test_execute_opcode_8XY3() {
        let mut cpu = CPU::new();

        cpu.cpu_register[1] = 0x0F; // X
        cpu.cpu_register[2] = 0x01; // Y

        cpu.decode_opcode(0x8123);
        assert_eq!(cpu.cpu_register[0x1], 0x0E);
    }

    #[test]
    fn test_execute_opcode_8XY4() {
        let mut cpu = CPU::new();

        assert_eq!(cpu.cpu_register[0xF], 0);

        cpu.cpu_register[1] = 0xEE; // X
        cpu.cpu_register[2] = 0x21; // Y

        cpu.decode_opcode(0x8124);
        assert_eq!(cpu.cpu_register[0x1], 0xF);
        assert_eq!(cpu.cpu_register[0xF], 0x1);
    }

    #[test]
    fn test_execute_opcode_8XY5() {
        let mut cpu = CPU::new();

        cpu.cpu_register[0xF] = 1;

        cpu.cpu_register[5] = 0xA2; // X
        cpu.cpu_register[6] = 0xCD; // Y

        cpu.decode_opcode(0x8565);
        assert_eq!(cpu.cpu_register[0x5], 0xD5);
        assert_eq!(cpu.cpu_register[0xF], 0);
    }

    #[test]
    fn test_execute_opcode_8XY6() {
        let mut cpu = CPU::new();

        assert_eq!(cpu.cpu_register[0xF], 0);
        cpu.cpu_register[0xA] = 0x19; // X

        cpu.decode_opcode(0x8A06);
        assert_eq!(cpu.cpu_register[0xA], 0xC);
        assert_eq!(cpu.cpu_register[0xF], 1);
    }

    #[test]
    fn test_execute_opcode_8XY7() {
        let mut cpu = CPU::new();

        cpu.cpu_register[0xF] = 1;
        cpu.cpu_register[0x3] = 0x51; // X
        cpu.cpu_register[0x8] = 0x1F; // Y


        cpu.decode_opcode(0x8387);
        assert_eq!(cpu.cpu_register[0x3], 0xCE);
        assert_eq!(cpu.cpu_register[0xF], 0);
    }

    #[test]
    fn test_execute_opcode_8XYE() {
        let mut cpu = CPU::new();

        assert_eq!(cpu.cpu_register[0xF], 0);
        cpu.cpu_register[0x1] = 0xBA; // X. 0b1011 1010

        cpu.decode_opcode(0x810E);
        assert_eq!(cpu.cpu_register[0x1], 0x74);  // 0b0111 0100
        assert_eq!(cpu.cpu_register[0xF], 1);

    }

}
