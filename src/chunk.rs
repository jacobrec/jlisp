use crate::bytecode::Op;
use crate::bytecode::Value;

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<Op>,
    lines: Vec<usize>,
    values: Vec<Value>,
}

impl Chunk {
    pub fn disassemble(&self) {
        disassemble_inner(&self, 0)
    }

    pub fn add_constant(&mut self, val: Value, line: usize) -> usize {
        let i = self.values.len();
        self.values.push(val);
        match i {
            x if x < 2usize.pow(8) => {
                self.add_op(Op::Const1, line);
                self.add_op(Op::from_lit(x as u8), line);
            },
            x if x < 2usize.pow(16) => {
                self.add_op(Op::Const2, line);
                self.add_op(Op::from_lit(((x >> 8) | 0xFF) as u8), line);
                self.add_op(Op::from_lit((x | 0xFF) as u8), line);
            },
            x if x < 2usize.pow(32) => {
                self.add_op(Op::Const3, line);
                self.add_op(Op::from_lit(((x >> 16) | 0xFF) as u8), line);
                self.add_op(Op::from_lit(((x >> 8) | 0xFF) as u8), line);
                self.add_op(Op::from_lit((x | 0xFF) as u8), line);
            },
            _ => panic!("Sorry, a program can't declare more then 4294967295 variables")

        }
        i
    }

    // i is to be the opcode refering to the constant (eg. Const1)
    pub fn read_constant(&self, i: usize) -> Value {
        match self.code[i] {
            Op::Const1 => self.read_constant_internal(i+1, 1),
            Op::Const2 => self.read_constant_internal(i+1, 2),
            Op::Const3 => self.read_constant_internal(i+1, 3),
            _ => panic!("no constant at index {}", i)
        }
    }
    fn read_constant_internal(&self, loc: usize, length: usize) -> Value {
        let mut index: usize = 0;
        for v in &self.code[loc..loc+length] {
            index *= 256;
            index += v.to_lit() as usize;
        }
        self.values[index].clone()
    }

    pub fn add_op(&mut self, op: Op, line: usize) -> usize {
        self.adding_op_line(line);
        self.code.push(op);
        self.code.len() - 1
    }

    pub fn replace_instruction(&mut self, i: usize, op: Op) {
        self.code[i] = op
    }

    pub fn get_line(&self, i: usize) -> usize {
        for (index, s) in self.lines.iter().enumerate() {
            if *s > i {
                return index
            }
        }
        self.lines.len()
    }

    fn adding_op_line(&mut self, line: usize) {
        if line > self.lines.len() {
            self.lines.push(self.code.len());
        }
    }
}


fn disassemble_inner(c: &Chunk, i: usize) {
    if i < c.code.len() {
        disassemble_inner(c, i + disassemble_instruction(c, i))
    }
}

pub fn disassemble_instruction(c: &Chunk, i: usize) -> usize{
    match c.code[i] {
        Op::Return | Op::Add | Op::Subtract | Op::Multiply | Op::Divide | Op::Negate => {
            disassemble_simple(c.code[i], c.get_line(i), i); 1
        },
        Op::Equal | Op::Less | Op::Greater | Op::GreaterEqual | Op::LessEqual => {
            disassemble_with_data1(c.code[i], c.get_line(i), i, c.code[i+1].to_lit()); 2
        },
        Op::Const1 => { disassemble_const(c, Op::Const1, i); 2 },
        Op::Const2 => { disassemble_const(c, Op::Const2, i); 3 },
        Op::Const3 => { disassemble_const(c, Op::Const3, i); 4 },
        Op::Jump | Op::JumpTrue => {
            disassemble_with_data1(c.code[i], c.get_line(i), i, c.code[i+1].to_lit()); 2
        },
        _ => { println!("Op not found {}", c.code[i].to_lit()); 0 }
    }
}

fn disassemble_const(c: &Chunk, o: Op, loc: usize){
    let line = c.get_line(loc);
    disassemble_op(o, line, loc);
    print!(" '{:?}'", c.read_constant(loc));
    println!();
}

fn disassemble_simple(o: Op, line: usize, loc: usize) {
    disassemble_op(o, line, loc);
    println!();
}

fn disassemble_with_data1(o: Op, line: usize, loc: usize, data: u8) {
    disassemble_op(o, line, loc);
    print!(": {}", data);
    println!();
}

fn disassemble_op(o: Op, line: usize, loc: usize) {
    print!("{:05X} [{:03}] Op {:?}", loc, line, o);
}

pub fn new() -> Chunk {
    Chunk{
        code: vec![],
        values: vec![],
        lines: vec![],
    }
}
