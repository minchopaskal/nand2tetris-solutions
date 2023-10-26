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
    syntax::{ClassNode, IdentifierId, SubroutineDec, SubroutineKind, SyntaxTree, Term, Type},
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

struct SymbolData<'a> {
    stype: &'a Type,
    kind: Segment,
}

impl<'a> SymbolData<'a> {
    fn new(stype: &'a Type, kind: Segment) -> Self {
        SymbolData { stype, kind }
    }
}

struct GenData<'a> {
    global: HashMap<IdentifierId, SymbolData<'a>>,
    local: HashMap<IdentifierId, SymbolData<'a>>,
    terms: &'a Vec<Term>,
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

    fn generate_class(&self, root: &ClassNode, data: &mut GenData<'_>) -> VMGeneratorResult {
        for (_i, _var) in root.fields.iter().enumerate() {}

        self.gen_subroutines(&root.subroutines, data)?;

        Ok(())
    }

    fn gen_subroutines(
        &self,
        subroutines: &[SubroutineDec],
        data: &mut GenData,
    ) -> VMGeneratorResult {
        for sd in subroutines {
            data.local.clear();
            match sd.kind {
                SubroutineKind::Constructor => todo!(),
                SubroutineKind::Function => todo!(),
                SubroutineKind::Method => todo!(),
            }
        }

        Ok(())
    }
}
