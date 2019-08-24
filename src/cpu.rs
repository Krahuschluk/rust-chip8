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
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_apa() {
        assert_eq!(merge_opcodes(0x1A, 0xB2), 0x1AB2);
    }
}
