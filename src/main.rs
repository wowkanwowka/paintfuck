mod interpreter;
mod tests;

fn main() {
    let mut interpreter = interpreter::Interpreter::new(
        "s*", 10, 20, 20
    );
    interpreter.print_state();
    interpreter.print_internal_state();
//    interpreter.process_next();
    interpreter.process_next().expect("Come on");
    interpreter.process_next().expect("Come on");
    interpreter.print_state();
}

