use crate::ast;
use crate::bytecode;

use std::collections::HashMap;

const SAME_LINE: usize = 0;

pub type InlineType = fn (&mut super::Evaluator, &ast::List<ast::Atom>) -> ();

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

    funs.insert(String::from("car"), car_inline as InlineType);
    funs.insert(String::from("cdr"), cdr_inline as InlineType);
    funs.insert(String::from("cons"), cons_inline as InlineType);

    // Special forms
    funs.insert(String::from("if"), if_inline as InlineType);
    funs.insert(String::from("quote"), quote_inline as InlineType);
    funs.insert(String::from("do"), do_inline as InlineType);

    funs
}

fn comp_equal_inline(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>) {
    inline_helper_comp(eve, ast, bytecode::Op::Equal)
}
fn comp_less_equal_inline(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>) {
    inline_helper_comp(eve, ast, bytecode::Op::LessEqual)
}
fn comp_greater_equal_inline(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>) {
    inline_helper_comp(eve, ast, bytecode::Op::GreaterEqual)
}
fn comp_less_then_inline(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>) {
    inline_helper_comp(eve, ast, bytecode::Op::Less)
}
fn comp_greater_then_inline(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>) {
    inline_helper_comp(eve, ast, bytecode::Op::Greater)
}

fn plus_inline(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>) {
    inline_helper_binary(eve, ast, bytecode::Op::Add)
}
fn times_inline(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>) {
    inline_helper_binary(eve, ast, bytecode::Op::Multiply)
}
fn divide_inline(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>) {
    inline_helper_binary(eve, ast, bytecode::Op::Divide)
}
fn minus_inline(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>) {
    if ast.len() > 2 {
        inline_helper_binary(eve, ast, bytecode::Op::Subtract)
    } else {
        inline_helper_parse_args(eve, ast);
        eve.chunk.add_op(bytecode::Op::Negate, SAME_LINE);
    }
}

fn if_inline(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>) {
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
    eve.eval_atom(condition_arg.expect(""), SAME_LINE);
    eve.chunk.add_op(bytecode::Op::JumpTrue, SAME_LINE);
    let d1 = eve.chunk.add_op(bytecode::Op::from_lit(0), SAME_LINE);
    if let Some(arg) = false_arg {
        eve.eval_atom(arg, SAME_LINE);
    }
    eve.chunk.add_op(bytecode::Op::Jump, 0);
    let d2 = eve.chunk.add_op(bytecode::Op::from_lit(0), SAME_LINE);
    eve.chunk.replace_instruction(d1, bytecode::Op::from_lit((d2 - d1) as u8));

    eve.eval_atom(true_arg.expect(""), SAME_LINE);
    let end = eve.chunk.code.len() - 1;
    eve.chunk.replace_instruction(d2, bytecode::Op::from_lit((end - d2) as u8));
}

fn quote_helper_alist_to_vlist(ast: &ast::Atom) -> bytecode::Value {
    match ast {
        ast::Atom::AFalse => bytecode::Value::VBool(false),
        ast::Atom::ATrue => bytecode::Value::VBool(true),
        ast::Atom::AString(s) => bytecode::Value::VString(s.clone()),
        ast::Atom::AInteger(v) => bytecode::Value::VInt(*v),
        ast::Atom::AIdentifier(v) => unimplemented!(),
        ast::Atom::AList(v) => {
            let mut l: ast::List<bytecode::Value> = ast::List::new();
            for x in v.iter() {
                l = l.append(quote_helper_alist_to_vlist(x))
            }
            return bytecode::Value::VList(l)
        },
    }
}

fn quote_inline(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>) {
    if ast.len() == 2 {
        if let Some(val) = ast.head() {
            eve.chunk.add_constant(quote_helper_alist_to_vlist(val), SAME_LINE);
            return;
        }
    }
    panic!("Error, wrong number of arguments");
}

fn car_inline(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>) {
    if ast.len() == 2 {
        inline_helper_parse_args(eve, ast);
        eve.chunk.add_op(bytecode::Op::Car, SAME_LINE);
        return;
    }
    panic!("Error, wrong number of arguments");
}

fn cdr_inline(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>) {
    if ast.len() == 2 {
        inline_helper_parse_args(eve, ast);
        eve.chunk.add_op(bytecode::Op::Cdr, SAME_LINE);
        return;
    }
    panic!("Error, wrong number of arguments");
}

fn cons_inline(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>) {
    if ast.len() == 3 {
        inline_helper_parse_args(eve, ast);
        eve.chunk.add_op(bytecode::Op::Cons, SAME_LINE);
        return;
    }
    panic!("Error, wrong number of arguments");
}

fn do_inline(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>) {
    let mod_ast = ast::List::reverse(ast.append(ast::Atom::AFalse)).tail();
    let arg_count = inline_helper_parse_args(eve, &mod_ast);
    if arg_count > 255 {
        panic!("Can't have more then 255 forms in a do");
    }
    // TODO: discard the unused operations, currently they remain on the stack
    // eve.chunk.add_op(bytecode::Op::Discard, SAME_LINE);
    // eve.chunk.add_op(bytecode::Op::from_lit((arg_count - 1) as u8), SAME_LINE);
    return;
}

fn inline_helper_comp(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>, opcode: bytecode::Op) {
    let count = inline_helper_parse_args(eve, ast);
    if count > 255 {
        panic!("Can't have more then 255 values in a comparision");
    }
    eve.chunk.add_op(opcode, SAME_LINE);
    eve.chunk.add_op(bytecode::Op::from_lit(count as u8), SAME_LINE);
}

fn inline_helper_binary(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>, opcode: bytecode::Op) {
    let count = inline_helper_parse_args(eve, ast);
    for _ in 0..(count - 1) {
        eve.chunk.add_op(opcode, SAME_LINE);
    }
}

fn inline_helper_parse_args(eve: &mut super::Evaluator, ast: &ast::List<ast::Atom>) -> usize {
    let mut iter = ast.iter().peekable();
    let mut count = 0;
    loop {
        let node = iter.next().expect("");
        if let Some(_) = iter.peek() {
            eve.eval_atom(node, SAME_LINE);
            count += 1;
        } else {
            return count;
        }
    };
}
