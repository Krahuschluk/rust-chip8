/// ```rust
/// assert_eq!(2, add(2,3));
/// ```
fn add(a: i32, b: i32) -> i32{
    a + b
}

//struct CPU {
//    opcode: u16,
//
//    cpu_register: [u8; 16],
//
//    index: u16,
//
//
//}
//
//struct Memory {
//    memory: [u8; 4096],
//    program_counter: u16,
//}
//
//impl Memory {
//    fn store(&mut self, v: u8) {
//        self.memory[1] = v;
//    }
//
//    fn fetch(&mut self, val: usize) {
//        self.memory[val];
//    }
//
//    fn counter_up(&mut self) {
//        self.program_counter += 1;
//    }
//}


struct Stack {
    stack: [u16; 16],
    pointer: u16,

}

fn merge_opcodes(first: u8, second: u8) -> u16 {
    let f = first as u16;
    let s = second as u16;
    let r = (f << 8) | s;

    println!("0x{:x} 0b{:b}", r, r);

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
    // Maybe inject these from the outside when initializing?
    pub memory: [u8; 4096],
    pub program_counter: u16,
    pub stack: [u16; 16],
    pub stack_pointer: u16,
    pub cpu_register: [u8; 16],
    pub opcode: u16,
    pub index: u16,
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
        }
    }

    pub fn emulate_cycle(&self) {
        // Fetch opcode
        // pc will point to memory, where to pick up opcode
        let opcode = CPU::fetch_opcode(self.program_counter, self.memory);

        // Decode opcode
        // Execute opcode
        //
        // Update timers
    }

    fn fetch_opcode(pc: u16, mem: [u8; 4096]) {
        // Inputs: pc, memory
        // Output: opcode
        //
        // Use pc and pc + 1, merge
        let counter = pc as usize;

        merge_opcodes(mem[counter], mem[counter + 1]);
    }

    // Input is a 2-byte opcode
    // We do everything in hexadecimal -> 0xffff
    pub fn decode_opcode(&mut self, opcode: u16) {
        println!("Opcode is 0x{:x}", opcode);

        // Read out first half byte
        match opcode & 0xF000 {
            // 6XNN sets VX to NN
            0x6000 => {
                println!("0x{:x}", opcode & 0x0FFF);
                let x = ((opcode & 0x0F00) >> 8) as usize;
                println!("V: 0x{:x}", x);

                let val = (opcode & 0x00FF) as u8;
                println!("Value: 0x{:x}", val);
                self.cpu_register[x] = val;
            },

            // Mathematical operator block identified on last half byte
            // 0x_XY_
            0x8000 => {
                match opcode & 0x000F {
                    // Assign VX to value of VY
                    0x0000 => {
                        // X and Y can easily be set higher up
                        let x = ((opcode & 0x0F00) >> 8) as usize;
                        let y = ((opcode & 0x00F0) >> 4) as usize;

                        self.cpu_register[x] = self.cpu_register[y];
                    }

                    // Set VX to bit VX | VY
                    0x0001 => {

                    }

                    // Set VX to bit VX & VY
                    0x0002 => {

                    }

                    // Set VX to VX xor VY
                    0x0003 => {

                    }

                    // Add VY to VX, carry 1 on VF if needed
                    0x0004 => {

                    }

                    // Subtract VY from VX, borrow 1 from VF if needed
                    0x0005 => {

                    }


                    _ => println!("abort"),

                }

            }

            _ => println!("abort"),

        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

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
    fn test_execute_opcode_8XY0() {
        let mut cpu = CPU::new();

        cpu.cpu_register[3] = 0xFA;
        assert_eq!(cpu.cpu_register[0x5], 0);

        cpu.decode_opcode(0x8530);
        assert_eq!(cpu.cpu_register[0x5], 0xFA);
    }

}
