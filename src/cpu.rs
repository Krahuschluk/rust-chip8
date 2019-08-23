/// ```rust
/// assert_eq!(2, add(2,3));
/// ```
fn add(a: i32, b: i32) -> i32{
    a + b
}

struct CPU {
    opcode: u16,
    memory: [u8; 4096],
    cpu_register: [u8; 16],

    index: u16,
    program_counter: u16,
    stack: [u16; 16],
    stack_pointer: u16,
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

fn emulate_cycle() {
    // Fetch opcode
    // pc will point to memory, where to pick up opcode
    
    // Decode opcode
    // Execute opcode
    //
    // Update timers
}

fn fetch_opcode() {
    // Inputs: pc, memory
    // Output: opcode
    //
    // Use pc and pc + 1, merge
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
