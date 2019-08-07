pub mod bytecode;
pub mod chunk;
pub mod vm;
pub mod reader;
pub mod evaluator;
pub mod ast;

fn main() {
    let debug = true;
    let show_whole_code = true;
    let show_execution = true;

    let test = "(if (> 1 2) true false)";
    if let Some(chunk) = evaluator::evaluate(reader::read(test)) {

        if debug && show_whole_code {
            println!("PRINTING WHOLE CODE");
            chunk.disassemble();
            println!("END OF WHOLE CODE");
        }
        let mut v = vm::new(chunk);
        v.debug = debug && show_execution;
        if let Err(err) = v.run() {
            match err {
                vm::VMError::Runtime(msg) => println!("Runtime error: {}", msg),
                vm::VMError::Compile(msg) => println!("Compile error: {}", msg),
            }
        }
    } else {
        println!("Compile error")
    }
}
