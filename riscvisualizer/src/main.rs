#![feature(braced_empty_structs)]
#![feature(op_assign_traits)]

mod assembler;
mod interpreter;
mod isa;
mod memory;
mod types;

fn main() {
    let program = vec![
        isa::Instruction::I {
            opcode: isa::IOpcode::ADDI,
            rd: isa::T0,
            rs1: isa::Zero,
            imm: 0xFF,
        }
    ];
    let mut interpreter = interpreter::Interpreter::new(256, &program);
    interpreter.step();

    println!("{:?}", interpreter.registers());
}
