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
    syntax::{
        self, ClassNode, ClassVarKind, DoStmt, Expression, IfStmt, KeywordConstant, LetStmt,
        ReturnStmt, Statement, SubroutineBody, SubroutineCall, SubroutineDec, SubroutineKind,
        SyntaxTree, Term, Type, UnaryOp, WhileStmt,
    },
    tokens::Identifier,
};

type Label<'a> = &'a str;

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
    Mul,
    Div,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Add => write!(f, "add"),
            Op::Sub => write!(f, "sub"),
            Op::Neg => write!(f, "neg"),
            Op::Eq => write!(f, "eq"),
            Op::Gt => write!(f, "gt"),
            Op::Lt => write!(f, "lt"),
            Op::And => write!(f, "and"),
            Op::Or => write!(f, "or"),
            Op::Not => write!(f, "not"),
            Op::Mul => write!(f, "call Math.multiply 2"),
            Op::Div => write!(f, "call Math.divide 2"),
        }
    }
}

impl From<&syntax::Op> for Op {
    fn from(value: &syntax::Op) -> Self {
        match value {
            syntax::Op::Unknown => unreachable!(),
            syntax::Op::Plus => Op::Add,
            syntax::Op::Minus => Op::Sub,
            syntax::Op::Multiply => Op::Mul,
            syntax::Op::Divide => Op::Div,
            syntax::Op::And => Op::And,
            syntax::Op::Or => Op::Or,
            syntax::Op::Less => Op::Lt,
            syntax::Op::Greater => Op::Gt,
            syntax::Op::Equal => Op::Eq,
        }
    }
}

impl From<UnaryOp> for Op {
    fn from(value: UnaryOp) -> Self {
        match value {
            UnaryOp::Unknown => unreachable!(),
            UnaryOp::Minus => Op::Neg,
            UnaryOp::Not => Op::Not,
        }
    }
}

struct VMWriter {
    fw: BufWriter<File>,
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
        writeln!(&mut self.fw, "label {l}")
    }

    fn goto(&mut self, l: Label) -> io::Result<()> {
        writeln!(&mut self.fw, "goto {l}")
    }

    fn if_goto(&mut self, l: Label) -> io::Result<()> {
        writeln!(&mut self.fw, "if-goto {l}")
    }

    fn call(&mut self, caller: &str, name: &str, nargs: i32) -> io::Result<()> {
        writeln!(&mut self.fw, "call {caller}.{name} {nargs}")
    }

    fn function(&mut self, caller: &str, name: &str, nvars: i32) -> io::Result<()> {
        writeln!(&mut self.fw, "function {caller}.{name} {nvars}")
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

pub type VMGeneratorResult = Result<(), Box<dyn Error>>;

#[derive(Debug, Clone, Copy)]
struct SymbolData {
    stype: Type,
    segment: Segment,
}

struct GenData<'a> {
    global: HashMap<Identifier<'a>, SymbolData>,
    local: HashMap<Identifier<'a>, SymbolData>,
    tree: &'a SyntaxTree<'a>,
    w: VMWriter,

    label_idx: usize,
}

impl<'a> GenData<'a> {
    fn get_label(&mut self) -> String {
        self.label_idx += 1;

        format!("LABEL_{}", self.label_idx - 1)
    }

    fn get_var(&self, name: usize, tree: &SyntaxTree) -> Result<SymbolData, VMGeneratorError> {
        let name = tree.get_id(name);
        if let Some(sd) = self.local.get(name) {
            Ok(*sd)
        } else if let Some(sd) = self.global.get(name) {
            Ok(*sd)
        } else {
            Err(VMGeneratorError {
                str: format!("Name {:?} not in scope, file {}", name, self.tree.filename),
            })
        }
    }
}

pub struct VMGenerator {
    path: PathBuf,
}

impl Analyzer for VMGenerator {
    type Output = VMGeneratorResult;

    fn analyze(&self, tree: &SyntaxTree) -> VMGeneratorResult {
        let mut data = GenData {
            global: HashMap::new(),
            local: HashMap::new(),
            tree,
            w: VMWriter {
                fw: BufWriter::new(File::create(&self.path).unwrap()),
            },
            label_idx: 0,
        };
        self.generate_class(&tree.root, &mut data)?;

        Ok(())
    }
}

impl VMGenerator {
    pub fn new(dir: &str, filename: &str) -> Self {
        let fullname = filename.to_string() + ".vm";
        let path = Path::new(dir).join(fullname);

        VMGenerator { path }
    }

    fn generate_class(&self, root: &ClassNode, data: &mut GenData) -> VMGeneratorResult {
        let mut si = 0;
        let mut fi = 0;
        for var in root.fields.iter() {
            match var.kind {
                ClassVarKind::Static => {
                    data.global.insert(
                        data.tree.get_id(var.var_dec.name),
                        SymbolData {
                            stype: var.var_dec.var_type,
                            segment: Segment::Static(si),
                        },
                    );
                    si += 1;
                }
                ClassVarKind::Field => {
                    data.global.insert(
                        data.tree.get_id(var.var_dec.name),
                        SymbolData {
                            stype: var.var_dec.var_type,
                            segment: Segment::This(fi),
                        },
                    );
                    fi += 1;
                }
            }
        }

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
                SubroutineKind::Constructor => self.prep_constructor(sd, data)?,
                SubroutineKind::Function => self.prep_function(sd, data, 0)?,
                SubroutineKind::Method => self.prep_method(sd, data)?,
            }

            self.gen_subroutine_body(&sd.body, data)?
        }

        Ok(())
    }

    fn prep_constructor(&self, sd: &SubroutineDec, data: &mut GenData) -> VMGeneratorResult {
        self.prep_function(sd, data, 0)?;

        data.w
            .push(Segment::Constant(data.tree.root.fields.len() as i32))?;
        data.w.call("Memory", "alloc", 1)?;
        data.w.pop(Segment::Pointer(0))?;

        Ok(())
    }

    fn prep_function(
        &self,
        sd: &SubroutineDec,
        data: &mut GenData,
        fst_arg: i32,
    ) -> VMGeneratorResult {
        let mut arg = fst_arg;
        for v in sd.params.iter() {
            data.local.insert(
                data.tree.get_id(v.name),
                SymbolData {
                    stype: v.p_type,
                    segment: Segment::Argument(arg),
                },
            );

            arg += 1;
        }

        for (i, vd) in sd.body.var_decs.iter().enumerate() {
            data.local.insert(
                data.tree.get_id(vd.name),
                SymbolData {
                    stype: vd.var_type,
                    segment: Segment::Local(i as i32),
                },
            );
        }

        data.w.function(
            &data.tree.filename,
            data.tree.get_id(sd.name),
            sd.body.var_decs.len() as i32,
        )?;

        Ok(())
    }

    fn prep_method(&self, sd: &SubroutineDec, data: &mut GenData) -> VMGeneratorResult {
        data.local.insert(
            SyntaxTree::get_this(),
            SymbolData {
                stype: data.tree.get_type(),
                segment: Segment::Argument(0),
            },
        );

        self.prep_function(sd, data, 1)?;

        data.w.push(Segment::Argument(0))?;
        data.w.pop(Segment::Pointer(0))?;

        Ok(())
    }

    fn gen_subroutine_body(&self, body: &SubroutineBody, data: &mut GenData) -> VMGeneratorResult {
        self.gen_stmts(&body.stmts, data)?;

        Ok(())
    }

    fn gen_stmts(&self, stmts: &[Statement], data: &mut GenData) -> VMGeneratorResult {
        for stmt in stmts.iter() {
            match stmt {
                Statement::Let(ls) => self.gen_let(ls, data)?,
                Statement::If(is) => self.gen_if(is, data)?,
                Statement::While(ws) => self.gen_while(ws, data)?,
                Statement::Do(ds) => self.gen_do(ds, data)?,
                Statement::Return(rs) => self.gen_ret(rs, data)?,
            }
        }

        Ok(())
    }

    fn gen_let(&self, ls: &LetStmt, data: &mut GenData) -> VMGeneratorResult {
        let seg = data.get_var(ls.name, data.tree)?.segment;
        if let Some(expr) = &ls.idx {
            data.w.push(seg)?;
            self.gen_expression(expr, data)?;
            data.w.arith(Op::Add)?;

            self.gen_expression(&ls.eq_to, data)?;
            data.w.pop(Segment::Temp(0))?;
            data.w.pop(Segment::Pointer(1))?;
            data.w.push(Segment::Temp(0))?;
            data.w.pop(Segment::That)?;
        } else {
            self.gen_expression(&ls.eq_to, data)?;
            data.w.pop(seg)?;
        }

        Ok(())
    }

    fn gen_if(&self, is: &IfStmt, data: &mut GenData) -> VMGeneratorResult {
        self.gen_expression(&is.cond, data)?;
        data.w.arith(Op::Not)?;
        let ifnot_label = data.get_label();
        let after_else = if !is.else_body.is_empty() {
            data.get_label()
        } else {
            String::new()
        };
        data.w.if_goto(&ifnot_label)?;
        self.gen_stmts(&is.body, data)?;
        if !is.else_body.is_empty() {
            data.w.goto(&after_else)?;
        }

        data.w.label(&ifnot_label)?;
        if !is.else_body.is_empty() {
            self.gen_stmts(&is.else_body, data)?;
            data.w.label(&after_else)?;
        }

        Ok(())
    }

    fn gen_while(&self, ws: &WhileStmt, data: &mut GenData) -> VMGeneratorResult {
        let loop_label = data.get_label();
        let after_loop = data.get_label();

        data.w.label(&loop_label)?;

        self.gen_expression(&ws.cond, data)?;
        data.w.arith(Op::Not)?;
        data.w.if_goto(&after_loop)?;

        self.gen_stmts(&ws.body, data)?;
        data.w.goto(&loop_label)?;

        data.w.label(&after_loop)?;

        Ok(())
    }

    fn gen_do(&self, ds: &DoStmt, data: &mut GenData) -> VMGeneratorResult {
        self.gen_subroutine_call(&ds.call, data)?;
        data.w.pop(Segment::Temp(0))?;

        Ok(())
    }

    fn gen_ret(&self, rs: &ReturnStmt, data: &mut GenData) -> VMGeneratorResult {
        if let Some(expr) = &rs.ret_val {
            self.gen_expression(expr, data)?;
        } else {
            data.w.push(Segment::Constant(0))?;
        }

        data.w.ret()?;

        Ok(())
    }

    fn gen_expression(&self, expr: &Expression, data: &mut GenData) -> VMGeneratorResult {
        self.gen_term(expr.init_term, data)?;

        for (op, term) in &expr.ops {
            self.gen_term(*term, data)?;
            data.w.arith(op.into())?;
        }

        Ok(())
    }

    fn gen_subroutine_call(&self, call: &SubroutineCall, data: &mut GenData) -> VMGeneratorResult {
        let name = data.tree.get_id(call.name);
        let (caller, mut nargs) = if let Some(c) = call.caller {
            if let Ok(sd) = data.get_var(c, data.tree) {
                match sd.stype {
                    Type::ClassName(typ) => {
                        data.w.push(sd.segment)?;
                        (data.tree.get_id(typ), 1)
                    }
                    _ => unreachable!(),
                }
            } else {
                // Caller is not a variable in scope so we just assume it's a type
                // We also don't push anything on the stack as it's a static method
                (data.tree.get_id(c), 0)
            }
        } else {
            data.w.push(Segment::Pointer(0))?;
            (data.tree.filename.as_str(), 1)
        };

        for arg in &call.args {
            self.gen_expression(arg, data)?;
        }

        nargs += call.args.len();
        data.w.call(caller, name, nargs as i32)?;

        Ok(())
    }

    fn gen_term(&self, term: usize, data: &mut GenData) -> VMGeneratorResult {
        match &data.tree.terms[term] {
            Term::Int(i) => {
                data.w.push(Segment::Constant(*i))?;
            }
            Term::String(s) => {
                let s = data.tree.get_id(*s);

                data.w.push(Segment::Constant(s.len() as i32))?;
                data.w.call("String", "new", 1)?;
                for c in s.chars() {
                    data.w.push(Segment::Constant(c as i32))?;
                    data.w.call("String", "appendChar", 2)?;
                }
            }
            Term::VarName(name) => {
                let s = data.get_var(*name, data.tree)?.segment;
                data.w.push(s)?;
            }
            Term::KeywordConstant(kw) => match kw {
                KeywordConstant::Unknown => {
                    return Err(Box::new(VMGeneratorError {
                        str: format!("Unknwon keyword in {}", data.tree.filename),
                    }))
                }
                KeywordConstant::True => {
                    data.w.push(Segment::Constant(0))?;
                    data.w.arith(Op::Not)?;
                }
                KeywordConstant::False => {
                    data.w.push(Segment::Constant(0))?;
                }
                KeywordConstant::Null => {
                    data.w.push(Segment::Constant(0))?;
                }
                KeywordConstant::This => {
                    data.w.push(Segment::Pointer(0))?;
                }
            },
            Term::ArrayAccess(arr) => {
                let s = data.get_var(arr.var, data.tree)?.segment;
                data.w.push(s)?;
                self.gen_expression(&arr.idx, data)?;
                data.w.arith(Op::Add)?;
                data.w.pop(Segment::Pointer(1))?;
                data.w.push(Segment::That)?;
            }
            Term::Call(call) => {
                self.gen_subroutine_call(call, data)?;
            }
            Term::BracketExpression(expr) => {
                self.gen_expression(expr, data)?;
            }
            Term::Unary(term) => {
                self.gen_term(term.term, data)?;
                data.w.arith(term.op.into())?;
            }
        }

        Ok(())
    }
}
