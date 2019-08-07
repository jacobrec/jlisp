use crate::ast;
use crate::bytecode;

use std::collections::HashMap;

pub type InlineType = fn (&mut super::Evaluator, &ast::List) -> ();

pub fn get_inlines() -> HashMap<String, InlineType> {
    let mut funs = HashMap::new();
    funs.insert(String::from("+"), plus_inline as InlineType);
    funs.insert(String::from("*"), times_inline as InlineType);
    funs.insert(String::from("-"), minus_inline as InlineType);
    funs.insert(String::from("/"), divide_inline as InlineType);

    funs.insert(String::from("="), comp_equal_inline as InlineType);
    funs.insert(String::from("<"), comp_less_then_inline as InlineType);
    funs.insert(String::from(">"), comp_greater_then_inline as InlineType);
    funs.insert(String::from("<="), comp_less_equal_inline as InlineType);
    funs.insert(String::from(">="), comp_greater_equal_inline as InlineType);

    funs.insert(String::from("if"), if_inline as InlineType);
    funs
}

fn comp_equal_inline(eve: &mut super::Evaluator, ast: &ast::List) {
    inline_helper_comp(eve, ast, bytecode::Op::Equal)
}
fn comp_less_equal_inline(eve: &mut super::Evaluator, ast: &ast::List) {
    inline_helper_comp(eve, ast, bytecode::Op::LessEqual)
}
fn comp_greater_equal_inline(eve: &mut super::Evaluator, ast: &ast::List) {
    inline_helper_comp(eve, ast, bytecode::Op::GreaterEqual)
}
fn comp_less_then_inline(eve: &mut super::Evaluator, ast: &ast::List) {
    inline_helper_comp(eve, ast, bytecode::Op::Less)
}
fn comp_greater_then_inline(eve: &mut super::Evaluator, ast: &ast::List) {
    inline_helper_comp(eve, ast, bytecode::Op::Greater)
}

fn plus_inline(eve: &mut super::Evaluator, ast: &ast::List) {
    inline_helper_binary(eve, ast, bytecode::Op::Add)
}
fn times_inline(eve: &mut super::Evaluator, ast: &ast::List) {
    inline_helper_binary(eve, ast, bytecode::Op::Multiply)
}
fn divide_inline(eve: &mut super::Evaluator, ast: &ast::List) {
    inline_helper_binary(eve, ast, bytecode::Op::Divide)
}
fn minus_inline(eve: &mut super::Evaluator, ast: &ast::List) {
    if ast.len() > 2 {
        inline_helper_binary(eve, ast, bytecode::Op::Subtract)
    } else {
        inline_helper_parse_args(eve, ast);
        eve.chunk.add_op(bytecode::Op::Negate, 0); // TODO: add line info
    }
}

fn if_inline(eve: &mut super::Evaluator, ast: &ast::List) {
    let t1 = ast.tail();
    let t2 = t1.tail();
    let mut false_arg = ast.head();
    let mut true_arg = t1.head();
    let mut condition_arg = t2.head();
    if ast.len() == 3 {
        condition_arg = true_arg;
        true_arg = false_arg;
        false_arg = None;
    }
    eve.eval_s_expr_inner(condition_arg.expect(""));
    eve.chunk.add_op(bytecode::Op::JumpTrue, 0); // TODO: add line info
    let d1 = eve.chunk.add_op(bytecode::Op::from_lit(0), 0); // TODO: add line info
    if let Some(arg) = false_arg {
        eve.eval_s_expr_inner(arg);
    }
    eve.chunk.add_op(bytecode::Op::Jump, 0); // TODO: add line info
    let d2 = eve.chunk.add_op(bytecode::Op::from_lit(0), 0); // TODO: add line info
    eve.chunk.replace_instruction(d1, bytecode::Op::from_lit((d2 - d1) as u8));

    eve.eval_s_expr_inner(true_arg.expect(""));
    let end = eve.chunk.code.len() - 1;
    eve.chunk.replace_instruction(d2, bytecode::Op::from_lit((end - d2) as u8));
}


fn inline_helper_comp(eve: &mut super::Evaluator, ast: &ast::List, opcode: bytecode::Op) {
    let count = inline_helper_parse_args(eve, ast);
    if count > 255 {
        panic!("Can't have more then 255 values in a comparision");
    }
    eve.chunk.add_op(opcode, 0); // TOD): add line info
    eve.chunk.add_op(bytecode::Op::from_lit(count as u8), 0); // TODO: add line info
}

fn inline_helper_binary(eve: &mut super::Evaluator, ast: &ast::List, opcode: bytecode::Op) {
    let count = inline_helper_parse_args(eve, ast);
    for _ in 0..(count - 1) {
        eve.chunk.add_op(opcode, 0); // TODO: add line info
    }
}

fn inline_helper_parse_args(eve: &mut super::Evaluator, ast: &ast::List) -> usize {
    let mut iter = ast.iter().peekable();
    let mut count = 0;
    loop {
        let node = iter.next().expect("");
        if let Some(_) = iter.peek() {
            eve.eval_s_expr_inner(node);
            count += 1;
        } else {
            return count;
        }
    };
}
