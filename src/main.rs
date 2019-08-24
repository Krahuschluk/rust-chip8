mod cpu;

fn main() {
    println!("Hello, world!");
    let cpu = cpu::CPU {
        memory: [123; 4096],
        program_counter: 0,
        stack: [0; 16],
        stack_pointer: 0,
        cpu_register: [0; 16],
        opcode: 0,
        index: 0,
    };
}
