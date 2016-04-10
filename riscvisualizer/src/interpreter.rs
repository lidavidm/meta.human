use isa;

struct Interpreter {
    memory_words: usize,
    cache_lines: usize,
    cache_line_words: usize,
    cache_stall_cycles: usize,
}

struct Action {
    // pc: before after
    // read registers
    // written register(s)
    // read memory
    // written memory
}

impl Interpreter {
    fn step(&mut self) {

    }

    fn step_back(&mut self) {

    }
}
