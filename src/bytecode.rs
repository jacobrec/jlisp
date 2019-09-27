use std::mem::transmute;

use std::cmp::Ordering;
use std::ops;

use crate::ast::List;


#[derive(Debug, Copy, Clone)]
pub enum Op {
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,

    Car,
    Cdr,
    Cons,

    Equal,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,

    Discard,
    Discard1,
    Return,

    Const1, // Uses next byte to identify constant number
    Const2, // Uses next 2 bytes to identify constant number
    Const3, // Uses next 3 bytes to identify constant number

    Load,
    Store,

    CreateFrame,
    DropFrame,
    DropFrameSaveReturn,

    Jump,
    JumpTrue,

    NoOp,
}

impl Op {
    pub fn from_lit(val: u8) -> Op {
        unsafe { transmute(val) }
    }

    pub fn to_lit(&self) -> u8 {
        unsafe { transmute(*self as u8) }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    VFloat(f64),
    VInt(isize),
    VString(String),
    VBool(bool),
    VList(List<Value>),
    VErr,
}

impl Value {
    pub fn type_of(&self) -> String {
        match self {
            VFloat(_) => String::from("Float"),
            VInt(_) => String::from("Int"),
            VString(_) => String::from("String"),
            VBool(_) => String::from("Bool"),
            VList(_) => String::from("List"),
            VErr => String::from("Error"),
        }
    }
    pub fn is_truthy(&self) -> bool {
        match self {
            VFloat(f) => !f.is_nan() && *f != 0.0,
            VInt(i) => *i != 0,
            VString(s) => s.len() > 0,
            VBool(b) => *b,
            VList(l) => l.len() == 0,
            VErr => panic!("VErr should not be used"),
        }
    }
}

use Value::*;
impl ops::Add<Value> for Value {
    type Output = Value;
    fn add(self, rhs: Value) -> Value {
        match self {
            VInt(i) => {
                match rhs {
                    VInt(i2) => VInt(i + i2),
                    VFloat(f2) => VFloat(i as f64 + f2),
                    _ => VErr,
                }
            },
            VFloat(f) => {
                match rhs {
                    VInt(i2) => VFloat(f + i2 as f64),
                    VFloat(f2) => VFloat(f + f2),
                    _ => VErr,
                }
            },
            VString(s) => {
                match rhs {
                    VString(s2) => VString(s + &s2),
                    _ => VErr,
                }
            }
            _ => VErr,
        }
    }
}
impl ops::Sub<Value> for Value {
    type Output = Value;
    fn sub(self, rhs: Value) -> Value {
        self + (-rhs)
    }
}
impl ops::Neg for Value {
    type Output = Value;
    fn neg(self) -> Value {
        match self {
            VInt(i) => VInt(-i),
            VFloat(f) => VFloat(-f),
            _ => VErr,
        }
    }
}
impl ops::Mul<Value> for Value {
    type Output = Value;
    fn mul(self, rhs: Value) -> Value {
        match self {
            VInt(i) => {
                match rhs {
                    VInt(i2) => VInt(i * i2),
                    VFloat(f2) => VFloat(i as f64 * f2),
                    _ => VErr,
                }
            },
            VFloat(f) => {
                match rhs {
                    VInt(i2) => VFloat(f * i2 as f64),
                    VFloat(f2) => VFloat(f * f2),
                    _ => VErr,
                }
            },
            VString(s) => {
                match rhs {
                    VInt(i) if i > 0 => VString(s.repeat(i as usize)),
                    _ => VErr,
                }
            }
            _ => VErr,
        }
    }
}
impl ops::Div<Value> for Value {
    type Output = Value;
    fn div(self, rhs: Value) -> Value {
        match self {
            VInt(i) => {
                match rhs {
                    VInt(i2) => VInt(i / i2),
                    VFloat(f2) => VFloat(i as f64 / f2),
                    _ => VErr,
                }
            },
            VFloat(f) => {
                match rhs {
                    VInt(i2) => VFloat(f / i2 as f64),
                    VFloat(f2) => VFloat(f / f2),
                    _ => VErr,
                }
            },
            _ => VErr,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering>{
        match self {
            VInt(s) => {
                match other {
                    VInt(o) => s.partial_cmp(o),
                    VFloat(o) => (*s as f64).partial_cmp(o),
                    _ => None,
                }
            },
            VBool(_) => None,
            VList(_) => None,
            VString(s) => {
                match other {
                    VString(o) => s.partial_cmp(o),
                    _ => None,
                }
            },
            VFloat(s) => {
                match other {
                    VInt(o) => s.partial_cmp(&(*o as f64)),
                    VFloat(o) => s.partial_cmp(o),
                    _ => None,
                }
            },
            VErr => None
        }
    }
}
impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match self {
            VInt(s) => {
                if let VInt(o) = other {
                    return o == s;
                }
            },
            VBool(s) => {
                if let VBool(o) = other {
                    return o == s;
                }
            },
            VList(s) => {
                return false;
            },
            VString(s) => {
                if let VString(o) = other {
                    return o == s;
                }
            },
            VFloat(s) => {
                if let VFloat(o) = other {
                    return o == s;
                }
            },
            VErr => return false
        };
        return false;
    }
}

