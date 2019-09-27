use crate::chunk::Chunk;
use crate::bytecode::Op;
use crate::bytecode::Value;
use crate::bytecode::Value::*;

use std::cmp::Ordering;


pub enum VMError {
    Runtime(String),
    Compile(String),
}

pub struct VM {
    pub debug: bool,
    pub c: Chunk,
    ip: usize,
    stack: Vec<Value>,
    stack_frames: Vec<usize>,
}


pub fn new(c: Chunk) -> VM {
    VM {
        debug: false,
        c: c,
        ip: 0,
        stack: Vec::new(),
        stack_frames: Vec::new(),
    }
}

impl VM {
    fn get_data_i8(&mut self) -> i8 {
        self.get_data() as i8
    }

    fn get_data(&mut self) -> u8 {
        self.ip += 1;
        self.c.code[self.ip].to_lit()
    }
    pub fn run(&mut self) -> Result<Value, VMError>  {
        loop {
            let op = self.c.code[self.ip];
            if self.debug {
                crate::chunk::disassemble_instruction(&self.c, self.ip);
            }
            match op {
                Op::Return => {
                    let v = self.stack.pop().expect("Empty Stack");
                    return Ok(v)
                },
                Op::Discard1 => {
                    self.stack.pop().expect("Empty stack");
                },
                Op::Discard => {
                    let amount = self.get_data();
                    for _ in 0..amount {
                        self.stack.pop().expect("Empty stack");
                    }
                },

                Op::Const1 | Op::Const2 | Op::Const3 => {
                    self.stack.push(self.c.read_constant(self.ip));
                    self.ip += match op {
                        Op::Const1 => 1,
                        Op::Const2 => 2,
                        Op::Const3 => 3,
                        _ => return err("Const not a const")
                    };
                },

                Op::Negate => {
                    let val = self.stack.pop().expect("Empty stack");
                    match val {
                        VFloat(f) => self.stack.push(VFloat(-f)),
                        VInt(f) => self.stack.push(VInt(-f)),
                        _ => return err("Value is not negatable")
                    }
                },

                Op::Car | Op::Cdr => {
                    let val = self.stack.pop().expect("Empty stack");
                    match val {
                        VList(l) => {
                            match op {
                                Op::Car => self.stack.push(l.head().expect("List needs head").clone()),
                                Op::Cdr => self.stack.push(VList(l.tail())),
                                _ => panic!(""),
                            }
                        }
                        _ => return err("Value is not car-able")
                    }
                },

                Op::Cons => {
                    let elem = self.stack.pop().expect("Empty stack");
                    let list = self.stack.pop();
                    if let Some(VList(l)) = list {
                        self.stack.push(VList(l.append(elem)))
                    } else {
                        return err("not a cons-able object")
                    }
                },

                Op::Add | Op::Subtract | Op::Multiply | Op::Divide => {
                    let v1 = self.stack.pop().expect("Empty stack");
                    let v2 = self.stack.pop().expect("Empty stack");
                    let v_ans = binary_operator(op, v1, v2);
                    match v_ans {
                        Ok(v) => self.stack.push(v),
                        Err(e) => return Err(e)
                    }
                },

                Op::Equal | Op::LessEqual | Op::GreaterEqual | Op::Less | Op::Greater => {
                    let count = self.get_data();

                    let mut params = Vec::with_capacity(count as usize);
                    for _ in 0..count {
                        params.push(self.stack.pop().expect("Empty stack"));
                    }
                    let v_ans = comparison_operator(op, params);
                    match v_ans {
                        Ok(v) => self.stack.push(v),
                        Err(e) => return Err(e)
                    }
                },
                Op::Jump | Op::JumpTrue => {
                    let amount = self.get_data_i8();
                    if match op {
                        Op::Jump => true,
                        Op::JumpTrue => self.stack.pop().expect("Empty stack").is_truthy(),
                        _ => unimplemented!(),
                    } {
                        self.ip = ((self.ip as i64) + (amount as i64)) as usize;
                    }
                },

                Op::Store => {
                    let value = self.stack.pop().expect("Empty Stack");
                    self.stack.push(value.clone());
                    self.stack.push(value);
                },
                Op::Load => {
                    let loc = self.get_data();
                    let stack_back = self.get_data();
                    self.stack.push(self.stack[(loc as usize) + self.stack_frames[stack_back as usize]].clone());
                },
                Op::Set => {
                    let loc = self.get_data();
                    let stack_back = self.get_data();
                    self.stack[(loc as usize) + self.stack_frames[stack_back as usize]] = self.stack.pop().expect("Empty Stack");
                    self.stack.push(self.stack[(loc as usize) + self.stack_frames[stack_back as usize]].clone());
                },

                Op::CreateFrame => {
                    self.stack_frames.push(self.stack.len());
                },
                Op::DropFrame => {
                    let s = self.stack_frames.pop().expect("Empty stackframes");
                    self.stack.truncate(s);
                },
                Op::DropFrameSaveReturn => {
                    let v = self.stack.pop().expect("Empty stack");
                    let s = self.stack_frames.pop().expect("Empty stackframes");
                    self.stack.truncate(s);
                    self.stack.push(v);
                },

                _ => return err("Unimplemented op")
            }
            self.ip += 1;
        }
    }
}

fn comparison_operator(op: Op, vals: Vec<Value>) -> Result<Value, VMError> {
    let mut iter = vals.iter().rev();
    let cur_opt = iter.next();
    if cur_opt.is_some() {
        let mut cur = cur_opt.expect("");
        let mut val = Value::VBool(true);
        while let Some(next) = iter.next() {
            let cmp_val = next.partial_cmp(cur);
            if let Some(cmp) = cmp_val {
                val = match op {
                    Op::Equal => Value::VBool(cmp == Ordering::Equal),
                    Op::Less => Value::VBool(cmp == Ordering::Less),
                    Op::Greater => Value::VBool(cmp == Ordering::Greater),
                    Op::LessEqual => Value::VBool(cmp == Ordering::Less ||
                                                  cmp == Ordering::Equal),
                    Op::GreaterEqual => Value::VBool(cmp == Ordering::Less ||
                                                     cmp == Ordering::Equal),
                    _ => panic!(),
                };
                if let Value::VBool(x) = val {
                    if !x {
                        break
                    }
                }
                cur = next;
            } else {
                val = Value::VErr;
                break
            }
        };

        if let Value::VErr = val {
            return err("type error on comparison operator")
        }
        Ok(val)
    } else {
        err("No arguments found for comparision")
    }

}

fn binary_operator(op: Op, v1: Value, v2: Value) -> Result<Value, VMError> {
    let s1 = v1.type_of();
    let s2 = v2.type_of();
    let x = match op {
        Op::Add => v1 + v2,
        Op::Subtract => v1 - v2,
        Op::Multiply => v1 * v2,
        Op::Divide => v1 / v2,
        _ => return err("binary operator not found for type float")
    };
    if let Value::VErr = x {
        return err(format!("operator {:?} is not usable with types {} and {}",
                           op, s1, s2).as_ref())
    }
    Ok(x)
}

fn err<T>(msg: &str) -> Result<T, VMError> {
    Err(VMError::Runtime(String::from(msg)))
}
