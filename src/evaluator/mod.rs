use crate::chunk;
use crate::bytecode;
use crate::ast;

use std::collections::HashMap;

mod functions;

pub fn evaluate(ast: Option<ast::ASTList>) -> Option<chunk::Chunk> {
    ast.map(|ast| {
        let mut x = Evaluator {
            chunk: chunk::new(),
            inlined: functions::get_inlines(),
            var_stack: Vec::new(),
        };
        x.eval(ast)
    })
}

pub struct Evaluator {
    chunk: chunk::Chunk,
    inlined: HashMap<String, functions::InlineType>,
    var_stack: Vec<HashMap<String, usize>>,
}

impl Evaluator {
    fn eval(&mut self, ast: ast::ASTList) -> chunk::Chunk {
        self.eval_s_expr(ast);
        self.chunk.add_op(bytecode::Op::Return, 1);
        std::mem::replace(&mut self.chunk, chunk::new())
    }

    fn eval_s_expr(&mut self, ast: ast::ASTList) {
        let mut val = ast;
        while let Some((a, l)) = val.head() {
            self.eval_atom(a, *l);
            val = val.tail();
        }
    }

    fn eval_fn(&mut self, ast: &ast::List<ast::Atom>, line: usize) {
        let tail_tip = ast.tail_tip();
        if let Some(ast::Atom::AIdentifier(cmd)) = tail_tip {
            if let Some(f) = self.inlined.get(cmd) {
                f(self, ast);
            } else {
                unimplemented!("Can't do custom functions yet. Line: {}", line)
            }
        } else {
            panic!("Function is not a function")
        }
    }

    fn eval_atom(&mut self, ast: &ast::Atom, line: usize) {
        match ast {
            ast::Atom::AList(l) => {
                self.eval_fn(l, line)
            }
            ast::Atom::AInteger(v) => {
                self.chunk.add_constant(bytecode::Value::VInt(*v), line);
            },
            ast::Atom::AString(v) => {
                self.chunk.add_constant(bytecode::Value::VString((*v).clone()), line);
            },
            ast::Atom::AIdentifier(v) => {
                let (loc, stack_back) = self.get_var_stack_loc(v);
                self.chunk.add_op(bytecode::Op::Load, line);
                self.chunk.add_op(bytecode::Op::from_lit(loc), line);
                self.chunk.add_op(bytecode::Op::from_lit(stack_back), line);
            }
            ast::Atom::ATrue => {
                self.chunk.add_constant(bytecode::Value::VBool(true), line);
            },
            ast::Atom::AFalse => {
                self.chunk.add_constant(bytecode::Value::VBool(false), line);
            },
        }
    }
    fn get_var_stack_loc(&mut self, var: &String) -> (u8, u8) {
        let l = self.var_stack.len() - 1;
        let mut i: i64 = (self.var_stack.len() - 1) as i64;
        while i >= 0 {
            if let Some(x) = self.var_stack[i as usize].get(var) {
                return (*x as u8, (l - (i as usize)) as u8)
            }
            i -= 1;
        }
        panic!("Var not found: {}", var);
    }
}
