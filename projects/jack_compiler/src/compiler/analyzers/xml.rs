use std::{
    fs::File,
    io::{self, BufWriter},
    path::Path,
};

use crate::compiler::{
    analyzer::Analyzer,
    syntax::{
        ClassNode, ClassVarDec, DoStmt, Expression, IfStmt, LetStmt, Param, ReturnStmt, Statement,
        SubroutineBody, SubroutineCall, SubroutineDec, SubroutineType, SyntaxTree, Term, TermId,
        Type, VarDec, WhileStmt,
    },
};
use xmlwriter::Options;

#[macro_export]
macro_rules! write_element {
    ($w:ident,$type:expr,$val:expr) => {
        $w.start_element($type)?;
        $w.set_preserve_whitespaces(true);
        $w.write_text($val)?;
        $w.end_element()?;
        $w.set_preserve_whitespaces(false);
    };
}

type XmlWriter<'a> = xmlwriter::XmlWriter<'a, BufWriter<File>>;

///
/// Output syntax tree as XML in a file named
/// `tree.filename` + ".xml"
///
pub struct XMLAnalyzer {
    dir: String,
}

impl Analyzer for XMLAnalyzer {
    type Output = ();

    fn analyze(&self, tree: &SyntaxTree) {
        let filename = tree.filename.clone() + ".xml";
        let path = Path::new(&self.dir).join(filename);
        let fw = BufWriter::new(File::create(path).unwrap());

        let opts = Options {
            enable_self_closing: false,
            ..Options::default()
        };
        let mut w = XmlWriter::new(fw, opts);
        if let Err(e) = self.write_root(&mut w, &tree.root, &tree.terms) {
            println!("Failed to output XML. Reason: {e:?}");
        }
    }
}

impl XMLAnalyzer {
    pub fn new(dir: &str) -> XMLAnalyzer {
        XMLAnalyzer {
            dir: dir.to_string(),
        }
    }

    fn write_root(&self, w: &mut XmlWriter, root: &ClassNode, terms: &[Term]) -> io::Result<()> {
        w.start_element("class")?;

        write_element!(w, "keyword", "class");
        write_element!(w, "identifier", root.name);
        write_element!(w, "symbol", "{");

        self.write_classvardec(w, &root.fields, terms)?;
        self.write_subroutinedec(w, &root.subroutines, terms)?;

        write_element!(w, "symbol", "}");

        w.end_element()?;
        Ok(())
    }

    fn write_classvardec(
        &self,
        w: &mut XmlWriter,
        fields: &[ClassVarDec],
        _terms: &[Term],
    ) -> io::Result<()> {
        if fields.is_empty() {
            return Ok(());
        }

        let mut i = 0;
        let mut kind = fields[0].kind;
        let mut var_type = fields[0].var_dec.var_type;
        w.start_element("classVarDec")?;
        loop {
            if i >= fields.len() {
                break;
            }

            let same_line = if i > 0 {
                let res = kind == fields[i].kind && var_type == fields[i].var_dec.var_type;

                kind = fields[i].kind;
                var_type = fields[i].var_dec.var_type;
                res
            } else {
                false
            };

            if same_line {
                write_element!(w, "symbol", ",");
                write_element!(w, "identifier", fields[i].var_dec.name);
            } else {
                if i > 0 {
                    write_element!(w, "symbol", ";");
                    w.end_element()?;
                    w.start_element("classVarDec")?;
                }
                write_element!(w, "keyword", &kind.to_string());
                let tag = match var_type {
                    Type::Int | Type::Char | Type::Boolean => "keyword",
                    Type::ClassName(_) => "identifier",
                };
                write_element!(w, tag, &var_type.to_string());
                write_element!(w, "identifier", fields[i].var_dec.name);
            }

            i += 1;
        }
        write_element!(w, "symbol", ";");
        w.end_element()?;

        Ok(())
    }

    fn write_subroutinedec(
        &self,
        w: &mut XmlWriter,
        subroutine_decs: &[SubroutineDec],
        terms: &[Term],
    ) -> io::Result<()> {
        for sd in subroutine_decs {
            w.start_element("subroutineDec")?;

            write_element!(w, "keyword", &sd.kind.to_string());
            let tag = match sd.f_type {
                SubroutineType::Void => "keyword",
                SubroutineType::Type(_) => "identifier",
            };
            write_element!(w, tag, &sd.f_type.to_string());
            write_element!(w, "identifier", &sd.name);
            write_element!(w, "symbol", "(");
            self.write_param_list(w, &sd.params, terms)?;
            write_element!(w, "symbol", ")");
            self.write_subroutine_body(w, &sd.body, terms)?;

            w.end_element()?;
        }

        Ok(())
    }

    fn write_param_list(
        &self,
        w: &mut XmlWriter,
        params: &[Param],
        _terms: &[Term],
    ) -> io::Result<()> {
        w.start_element("parameterList")?;

        for i in 0..params.len() {
            let ptype = &params[i].p_type;
            let tag = match ptype {
                Type::Int | Type::Char | Type::Boolean => "keyword",
                Type::ClassName(_) => "identifier",
            };
            write_element!(w, tag, &ptype.to_string());
            write_element!(w, "identifier", params[i].name);
            if i < params.len() - 1 {
                write_element!(w, "symbol", ",");
            } else {
                write_element!(w, "symbol", ";");
            }
        }
        w.end_element()?;

        w.set_preserve_whitespaces(false);
        Ok(())
    }

    fn write_subroutine_body(
        &self,
        w: &mut XmlWriter,
        body: &SubroutineBody,
        terms: &[Term],
    ) -> io::Result<()> {
        w.start_element("subroutineBody")?;
        write_element!(w, "symbol", "{");

        self.write_vardecs(w, &body.var_decs, terms)?;
        self.write_stmts(w, &body.stmts, terms)?;

        write_element!(w, "symbol", "}");
        w.end_element()?;
        Ok(())
    }

    fn write_vardecs(
        &self,
        w: &mut XmlWriter,
        vardecs: &[VarDec],
        _terms: &[Term],
    ) -> io::Result<()> {
        if vardecs.is_empty() {
            return Ok(());
        }

        let mut var_type = vardecs[0].var_type;
        w.start_element("varDec")?;
        for (i, vd) in vardecs.iter().enumerate() {
            if i > 0 && var_type == vd.var_type {
                write_element!(w, "symbol", ",");
                write_element!(w, "identifier", vd.name);
            } else {
                if i > 0 {
                    write_element!(w, "symbol", ";");
                    w.end_element()?;
                    w.start_element("varDec")?;
                }
                write_element!(w, "keyword", "var");
                let tag = match vd.var_type {
                    Type::Int | Type::Char | Type::Boolean => "keyword",
                    Type::ClassName(_) => "identifier",
                };

                write_element!(w, tag, &vd.var_type.to_string());
                write_element!(w, "identifier", vd.name);
            }
            var_type = vd.var_type;
        }

        write_element!(w, "symbol", ";");
        w.end_element()?;
        Ok(())
    }

    fn write_stmts(
        &self,
        w: &mut XmlWriter,
        stmts: &[Statement],
        terms: &[Term],
    ) -> io::Result<()> {
        w.start_element("statements")?;
        for stmt in stmts.iter() {
            self.write_stmt(w, stmt, terms)?;
        }
        w.end_element()?;
        Ok(())
    }

    fn write_stmt(&self, w: &mut XmlWriter, stmt: &Statement, terms: &[Term]) -> io::Result<()> {
        match stmt {
            Statement::Let(l) => self.write_let(w, l, terms),
            Statement::If(ify) => self.write_if(w, ify, terms),
            Statement::While(whily) => self.write_while(w, whily, terms),
            Statement::Do(d) => self.write_do(w, d, terms),
            Statement::Return(ret) => self.write_return(w, ret, terms),
        }
    }

    fn write_let(&self, w: &mut XmlWriter, l: &LetStmt, terms: &[Term]) -> io::Result<()> {
        w.start_element("letStatement")?;

        write_element!(w, "keyword", "let");
        write_element!(w, "identifier", l.name);
        if let Some(idx) = &l.idx {
            write_element!(w, "symbol", "[");
            self.write_expression(w, idx, terms)?;
            write_element!(w, "symbol", "]");
        }
        write_element!(w, "symbol", "=");
        self.write_expression(w, &l.eq_to, terms)?;
        write_element!(w, "symbol", ";");

        w.end_element()?;
        Ok(())
    }

    fn write_if(&self, w: &mut XmlWriter, i: &IfStmt, terms: &[Term]) -> io::Result<()> {
        w.start_element("ifStatement")?;

        write_element!(w, "keyword", "if");
        write_element!(w, "symbol", "(");
        self.write_expression(w, &i.cond, terms)?;
        write_element!(w, "symbol", ")");
        write_element!(w, "symbol", "{");
        self.write_stmts(w, &i.body, terms)?;
        write_element!(w, "symbol", "}");

        if !i.else_body.is_empty() {
            write_element!(w, "keyword", "else");
            write_element!(w, "symbol", "{");
            self.write_stmts(w, &i.else_body, terms)?;
            write_element!(w, "symbol", "}");
        }

        w.end_element()?;
        Ok(())
    }

    fn write_while(&self, w: &mut XmlWriter, wh: &WhileStmt, terms: &[Term]) -> io::Result<()> {
        w.start_element("whileStatement")?;

        write_element!(w, "keyword", "while");
        write_element!(w, "symbol", "(");
        self.write_expression(w, &wh.cond, terms)?;
        write_element!(w, "symbol", ")");
        write_element!(w, "symbol", "{");
        self.write_stmts(w, &wh.body, terms)?;
        write_element!(w, "symbol", "}");

        w.end_element()?;
        Ok(())
    }

    fn write_do(&self, w: &mut XmlWriter, d: &DoStmt, terms: &[Term]) -> io::Result<()> {
        w.start_element("doStatement")?;

        write_element!(w, "keyword", "do");
        self.write_subroutine_call(w, &d.call, terms)?;
        write_element!(w, "symbol", ";");

        w.end_element()?;
        Ok(())
    }

    fn write_return(&self, w: &mut XmlWriter, r: &ReturnStmt, terms: &[Term]) -> io::Result<()> {
        w.start_element("returnStatement")?;

        write_element!(w, "keyword", "return");
        if let Some(expr) = &r.ret_val {
            self.write_expression(w, expr, terms)?;
        }
        write_element!(w, "symbol", ";");

        w.end_element()?;
        Ok(())
    }

    fn write_expression(
        &self,
        w: &mut XmlWriter,
        e: &Expression,
        terms: &[Term],
    ) -> io::Result<()> {
        w.start_element("expression")?;
        self.write_term(w, e.init_term, terms)?;
        for (op, term) in &e.ops {
            write_element!(w, "symbol", &op.to_string());
            self.write_term(w, *term, terms)?;
        }
        w.end_element()?;

        Ok(())
    }

    fn write_subroutine_call(
        &self,
        w: &mut XmlWriter,
        call: &SubroutineCall,
        terms: &[Term],
    ) -> io::Result<()> {
        if let Some(caller) = call.caller {
            write_element!(w, "identifier", caller);
            write_element!(w, "symbol", ".");
        }
        write_element!(w, "identifier", call.name);
        write_element!(w, "symbol", "(");
        w.start_element("expressionList")?;
        for (i, expr) in call.args.iter().enumerate() {
            self.write_expression(w, expr, terms)?;
            if i < call.args.len() - 1 {
                write_element!(w, "symbol", ",");
            }
        }

        w.end_element()?;
        write_element!(w, "symbol", ")");

        Ok(())
    }

    fn write_term(&self, w: &mut XmlWriter, term: TermId, terms: &[Term]) -> io::Result<()> {
        w.start_element("term")?;
        let term = &terms[term];
        match term {
            Term::Int(i) => {
                write_element!(w, "integerConstant", &i.to_string());
            }
            Term::String(s) => {
                write_element!(w, "stringConstant", *s);
            }
            Term::VarName(v) => {
                write_element!(w, "identifier", *v);
            }
            Term::KeywordConstant(k) => {
                write_element!(w, "keyword", &k.to_string());
            }
            Term::ArrayAccess(a) => {
                write_element!(w, "identifier", a.var);
                write_element!(w, "symbol", "[");
                self.write_expression(w, &a.idx, terms)?;
                write_element!(w, "symbol", "]");
            }
            Term::Call(c) => {
                self.write_subroutine_call(w, c, terms)?;
            }
            Term::BracketExpression(b) => {
                write_element!(w, "symbol", "(");
                self.write_expression(w, b, terms)?;
                write_element!(w, "symbol", ")");
            }
            Term::Unary(u) => {
                write_element!(w, "symbol", &u.op.to_string());
                self.write_term(w, u.term, terms)?;
            }
        }
        w.end_element()?;

        Ok(())
    }
}
