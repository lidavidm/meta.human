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
    {
        let action = interpreter.step();
        if let Some((register, before, after)) = action.written_register {
            println!("Register {:?} changed from {} to {}", register, before, after);
        }
    }

    println!("{:?}", interpreter.registers());
}
