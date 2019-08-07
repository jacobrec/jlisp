use crate::chunk;
use crate::bytecode;
use crate::ast;

use std::collections::HashMap;

mod functions;

pub fn evaluate(ast: Option<ast::List>) -> Option<chunk::Chunk> {
    ast.map(|ast| {
        let mut x = Evaluator {
            chunk: chunk::new(),
            inlined: functions::get_inlines(),
        };
        x.eval(ast)
    })
}

pub struct Evaluator {
    chunk: chunk::Chunk,
    inlined: HashMap<String, functions::InlineType>,
}

impl Evaluator {
    fn eval(&mut self, ast: ast::List) -> chunk::Chunk {
        self.eval_s_expr(ast);
        self.chunk.add_op(bytecode::Op::Return, 1);
        std::mem::replace(&mut self.chunk, chunk::new())
    }

    fn eval_s_expr(&mut self, ast: ast::List) {
        let mut val = ast;
        while let Some(astlora) = val.head() {
            self.eval_s_expr_inner(astlora);
            val = val.tail();
        }
    }
    fn eval_s_expr_inner(&mut self, ast: &ast::AtomOrList) {
        match ast {
            ast::AtomOrList::List(l) => self.eval_fn(l),
            ast::AtomOrList::Atom(a) => self.eval_atom(a),
        }
    }

    fn eval_fn(&mut self, ast: &ast::List) {
        let tail_tip = ast.tail_tip();
        if let Some(ast::AtomOrList::Atom(ast::Atom::AIdentifier(cmd))) = tail_tip {
            if let Some(f) = self.inlined.get(cmd) {
                f(self, ast);
            } else {
                unimplemented!("Can't do custom functions yet")
            }
        } else {
            panic!("Function is not a function")
        }
    }

    fn eval_atom(&mut self, ast: &ast::Atom) {
        match ast {
            ast::Atom::AInteger(v) => {
                self.chunk.add_constant(bytecode::Value::VInt(*v), 0); // TODO: proper line numbers
            },
            ast::Atom::AString(v) => {
                self.chunk.add_constant(bytecode::Value::VString((*v).clone()), 0); // TODO: proper line numbers
            },
            ast::Atom::AIdentifier(_) => unimplemented!(),
            ast::Atom::ATrue => {
                self.chunk.add_constant(bytecode::Value::VBool(true), 0); // TODO: proper line numbers
            },
            ast::Atom::AFalse => {
                self.chunk.add_constant(bytecode::Value::VBool(false), 0); // TODO: proper line numbers
            },
        }
    }
}
