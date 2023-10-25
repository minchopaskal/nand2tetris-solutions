use core::fmt;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::compiler::{
    analyzer::Analyzer,
    syntax::{ClassNode, SubroutineDec, SyntaxTree, Term},
    tokens::Identifier,
};

type Label = String;
type Value = i32;

#[derive(Debug, Copy, Clone)]
enum Segment {
    Local(i32),
    Argument(i32),
    Static(i32),
    Constant(i32),
    This(i32),
    That, // always 0
    Pointer(i32),
    Temp(i32),
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Segment::Local(i) => write!(f, "local {i}"),
            Segment::Argument(i) => write!(f, "argument {i}"),
            Segment::Static(i) => write!(f, "static {i}"),
            Segment::Constant(i) => write!(f, "constant {i}"),
            Segment::This(i) => write!(f, "this {i}"),
            Segment::That => write!(f, "that 0"),
            Segment::Pointer(i) => write!(f, "pointer {i}"),
            Segment::Temp(i) => write!(f, "temp {i}"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Add => write!(f, "add"),
            Op::Sub => write!(f, "add"),
            Op::Neg => write!(f, "add"),
            Op::Eq => write!(f, "eq"),
            Op::Gt => write!(f, "gt"),
            Op::Lt => write!(f, "gt"),
            Op::And => write!(f, "gt"),
            Op::Or => write!(f, "gt"),
            Op::Not => write!(f, "gt"),
        }
    }
}

struct VMWriter {
    fw: BufWriter<File>,
    name: String,
}

impl VMWriter {
    fn push(&mut self, s: Segment) -> io::Result<()> {
        writeln!(&mut self.fw, "push {s}")
    }

    fn pop(&mut self, s: Segment) -> io::Result<()> {
        writeln!(&mut self.fw, "pop {s}")
    }

    fn arith(&mut self, op: Op) -> io::Result<()> {
        writeln!(&mut self.fw, "{op}")
    }

    fn label(&mut self, l: Label) -> io::Result<()> {
        writeln!(&mut self.fw, "{l}")
    }

    fn goto(&mut self, l: Label) -> io::Result<()> {
        writeln!(&mut self.fw, "goto {l}")
    }

    fn if_goto(&mut self, l: Label) -> io::Result<()> {
        writeln!(&mut self.fw, "if-goto {l}")
    }

    fn call(&mut self, name: &str, nargs: i32) -> io::Result<()> {
        writeln!(&mut self.fw, "function {name} {nargs}")
    }

    fn function(&mut self, name: &str, nvars: i32) -> io::Result<()> {
        writeln!(&mut self.fw, "{name} {nvars}")
    }

    fn ret(&mut self) -> io::Result<()> {
        writeln!(&mut self.fw, "return")
    }
}

#[derive(Debug)]
pub struct VMGeneratorError {
    str: String,
}

impl fmt::Display for VMGeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str)
    }
}

impl Error for VMGeneratorError {}

pub type VMGeneratorResult = Result<(), VMGeneratorError>;

struct GenData<'a> {
    global: HashMap<Identifier<'a>, i32>,
    local: HashMap<Identifier<'a>, i32>,
    terms: &'a Vec<Term<'a>>,
    w: VMWriter,
}

pub struct VMGenerator {
    path: PathBuf,
    name: String,
}

impl Analyzer for VMGenerator {
    type Output = VMGeneratorResult;

    fn analyze(&self, tree: &SyntaxTree) -> VMGeneratorResult {
        let mut data = GenData {
            global: HashMap::new(),
            local: HashMap::new(),
            terms: &tree.terms,
            w: VMWriter {
                fw: BufWriter::new(File::create(&self.path).unwrap()),
                name: self.name.clone(),
            },
        };
        self.generate_class(&tree.root, &mut data)?;

        Ok(())
    }
}

impl VMGenerator {
    pub fn new(dir: &str, filename: &str) -> Self {
        let fullname = filename.to_string() + ".vm";
        let path = Path::new(dir).join(fullname);

        VMGenerator {
            path,
            name: filename.to_string(),
        }
    }

    fn generate_class<'a>(
        &self,
        root: &ClassNode<'a>,
        data: &mut GenData<'a>,
    ) -> VMGeneratorResult {
        for (i, var) in root.fields.iter().enumerate() {
            data.global.insert(var.var_dec.name, i as i32);
        }

        self.gen_subroutines(&root.subroutines, data)?;

        Ok(())
    }

    fn gen_subroutines(
        &self,
        subroutines: &[SubroutineDec],
        data: &mut GenData,
    ) -> VMGeneratorResult {
        todo!()
    }
}
