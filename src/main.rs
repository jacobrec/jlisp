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
    let show_ast = true;

    let test = r###"
        (quote (1 2))
        "###;
    let ast = reader::read(test);
    if debug && show_ast { dbg!(&ast); }
    if let Some(chunk) = evaluator::evaluate(ast) {

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
