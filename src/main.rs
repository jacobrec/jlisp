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
        (do
            (def a 2)
            (- 1 2 3)
            (- 1 2 3)
            (- 1 2 3)
            (def b 3)
            (- 1 2 3)
            (- 1 2 3)
            (def c (+ a b))
            (- 1 2 3)
            (def d (+ a b c))
            (- 1 2 3)
            (- 1 2 3)
            (- a b)
            (def e (+ c d))
            (- 1 2 3)
            (- 1 2 3)
            (def f e)
            e) ; Should be 15
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
        match v.run() {
            Err(err) => {
                match err {
                    vm::VMError::Runtime(msg) => println!("Runtime error: {}", msg),
                    vm::VMError::Compile(msg) => println!("Compile error: {}", msg),
                }
            },
            Ok(v) => {
                println!("RETURN: {:?}", v);
            }
        };
    } else {
        println!("Compile error")
    }
}

fn _test_string(test: &'static str) -> Option<crate::bytecode::Value> {
    let ast = reader::read(test);
    if let Some(chunk) = evaluator::evaluate(ast) {
        let mut v = vm::new(chunk);
        match v.run() {
            Err(err) => {
                match err {
                    vm::VMError::Runtime(msg) => println!("Runtime error: {}", msg),
                    vm::VMError::Compile(msg) => println!("Compile error: {}", msg),
                };
                None
            },
            Ok(v) => {
                dbg!(&v);
                Some(v)
            }
        }
    } else {
        None
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::bytecode::Value::*;

    #[test]
    fn test_math() {
        assert_eq!(Some(VInt(4)), _test_string("(- 6 2)"));
        assert_eq!(Some(VInt(3)), _test_string("(+ 1 2)"));
        assert_eq!(Some(VInt(10)), _test_string("(/ 30 3)"));
        assert_eq!(Some(VInt(15)), _test_string("(* 3 5)"));
        assert_eq!(Some(VInt(49)), _test_string("(- (+ 1 2 (* 3 4 (- 9 3 (/ 100 10 10)))) 14)"));
    }

    #[test]
    fn test_strings() {
        assert_eq!(Some(VString(String::from("Hello World"))), _test_string("(+ \"Hello\" \" \" \"World\")"));
    }

    #[test]
    fn test_list() {
        assert_eq!(Some(VInt(1)), _test_string("(car (quote (1 2 3)))"));
        assert_eq!(Some(VInt(1)), _test_string("(car (cons 1 (quote ()))"));
        assert_eq!(Some(VInt(1)), _test_string("(car (cons 1 (cons 2 (cons 3 (cons 4 (quote ()))))))"));
        assert_eq!(Some(VInt(2)), _test_string("(car (cdr (cons 1 (cons 2 (cons 3 (cons 4 (quote ())))))))"));
        assert_eq!(Some(VInt(3)), _test_string("(car (cdr (cdr (cons 1 (cons 2 (cons 3 (cons 4 (quote ()))))))))"));
        assert_eq!(Some(VString(String::from("Hello"))), _test_string("(car (quote (\"Hello\" \"World\")))"));
    }

    #[test]
    fn test_if() {
        assert_eq!(Some(VBool(true)), _test_string("(if true true false)"));
        assert_eq!(Some(VBool(false)), _test_string("(if false true false)"));
        assert_eq!(Some(VBool(true)), _test_string("(if (< 10 20) true false)"));
        assert_eq!(Some(VBool(true)), _test_string("(if (> (* 5 4) (* 4 4)) true false)"));
        assert_eq!(Some(VBool(false)), _test_string("(if (> (* 5 4) (* 4 4)) (> 1 2) (+ 1 2))"));
    }

    #[test]
    fn test_do() {
    let test_ans7 = r###"
        (do
            (- 1 2 3)
            (def a 2)
            (- 1 2 3)
            (set a 7)
            (- 1 2 3)
            a)
        "###;

    let test_ans15 = r###"
        (do
            (def a 2)
            (- 1 2 3)
            (- 1 2 3)
            (- 1 2 3)
            (def b 3)
            (- 1 2 3)
            (- 1 2 3)
            (def c (+ a b))
            (- 1 2 3)
            (def d (+ a b c))
            (- 1 2 3)
            (- 1 2 3)
            (- a b)
            (def e (+ c d))
            (- 1 2 3)
            (- 1 2 3)
            (def f e)
            e)
        "###;

        assert_eq!(Some(VInt(15)), _test_string(test_ans15));
        assert_eq!(Some(VInt(7)), _test_string(test_ans7));
    }
}
