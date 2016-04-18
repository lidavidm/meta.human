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
            imm: 0xF0,
        },
        isa::Instruction::I {
            opcode: isa::IOpcode::ADDI,
            rd: isa::T0,
            rs1: isa::T0,
            imm: 0x0F,
        }
    ];
    let mut interpreter = interpreter::Interpreter::new(256, &program);
    {
        let step = interpreter.step();
        match step.action {
            interpreter::Action::WriteRegister(register, before, after) => {
                println!("Register {:?} changed from {} to {}", register, before, after);
            }

            _ => {

            }
        }
    }
    {
        let step = interpreter.step();
        match step.action {
            interpreter::Action::WriteRegister(register, before, after) => {
                println!("Register {:?} changed from {} to {}", register, before, after);
            }

            _ => {

            }
        }
    }

    println!("{:?}", interpreter.registers());
}
