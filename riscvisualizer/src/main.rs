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
    interpreter.step();
    interpreter.step();

    for action in interpreter.iter() {
        match *action {
            interpreter::Action::ReadRegister(register, value) => {
                println!("Read register {:?}: {}", register, value);
            }

            interpreter::Action::WriteRegister(register, _, after) => {
                println!("Write register {:?}: {}", register, after);
            }

            interpreter::Action::Jump(_, after) => {
                println!("PC is {}", after);
            }

            _ => {

            }
        }
    }

    println!("{:?}", interpreter.registers());
}
