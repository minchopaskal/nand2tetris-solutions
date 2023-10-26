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
    ($d:ident,$type:expr,$val:expr) => {
        $d.w.start_element($type)?;
        $d.w.set_preserve_whitespaces(true);
        $d.w.write_text($val)?;
        $d.w.end_element()?;
        $d.w.set_preserve_whitespaces(false);
    };
}

#[macro_export]
macro_rules! write_id_elem {
    ($d:ident,$type:expr,$val:expr) => {
        let s = $d.tree.get_id($val);
        $d.w.start_element($type)?;
        $d.w.set_preserve_whitespaces(true);
        $d.w.write_text(&s)?;
        $d.w.end_element()?;
        $d.w.set_preserve_whitespaces(false);
    };
}

type XmlWriter<'a> = xmlwriter::XmlWriter<'a, BufWriter<File>>;
struct XmlWriterData<'a> {
    w: XmlWriter<'a>,
    tree: &'a SyntaxTree<'a>,
}

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
        let mut w = XmlWriterData {
            w: XmlWriter::new(fw, opts),
            tree,
        };

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

    fn write_root(
        &self,
        d: &mut XmlWriterData,
        root: &ClassNode,
        terms: &[Term],
    ) -> io::Result<()> {
        d.w.start_element("class")?;

        write_element!(d, "keyword", "class");
        write_id_elem!(d, "identifier", root.name);
        write_element!(d, "symbol", "{");

        self.write_classvardec(d, &root.fields, terms)?;
        self.write_subroutinedec(d, &root.subroutines, terms)?;

        write_element!(d, "symbol", "}");

        d.w.end_element()?;
        Ok(())
    }

    fn write_type(&self, d: &mut XmlWriterData, vtype: &Type) -> io::Result<()> {
        match vtype {
            Type::Int | Type::Char | Type::Boolean => {
                write_element!(d, "keyword", &vtype.to_string());
            }
            Type::ClassName(id) => {
                write_id_elem!(d, "identifier", *id);
            }
        }

        Ok(())
    }

    fn write_classvardec(
        &self,
        d: &mut XmlWriterData,
        fields: &[ClassVarDec],
        _terms: &[Term],
    ) -> io::Result<()> {
        if fields.is_empty() {
            return Ok(());
        }

        let mut i = 0;
        let mut kind = fields[0].kind;
        let mut var_type = &fields[0].var_dec.var_type;
        d.w.start_element("classVarDec")?;
        loop {
            if i >= fields.len() {
                break;
            }

            let same_line = if i > 0 {
                let res = kind == fields[i].kind && *var_type == fields[i].var_dec.var_type;

                kind = fields[i].kind;
                var_type = &fields[i].var_dec.var_type;
                res
            } else {
                false
            };

            if same_line {
                write_element!(d, "symbol", ",");
                write_id_elem!(d, "identifier", fields[i].var_dec.name);
            } else {
                if i > 0 {
                    write_element!(d, "symbol", ";");
                    d.w.end_element()?;
                    d.w.start_element("classVarDec")?;
                }
                write_element!(d, "keyword", &kind.to_string());
                self.write_type(d, var_type)?;
                write_id_elem!(d, "identifier", fields[i].var_dec.name);
            }

            i += 1;
        }
        write_element!(d, "symbol", ";");
        d.w.end_element()?;

        Ok(())
    }

    fn write_subroutinedec(
        &self,
        d: &mut XmlWriterData,
        subroutine_decs: &[SubroutineDec],
        terms: &[Term],
    ) -> io::Result<()> {
        for sd in subroutine_decs {
            d.w.start_element("subroutineDec")?;

            write_element!(d, "keyword", &sd.kind.to_string());
            match sd.f_type {
                SubroutineType::Void => {
                    write_element!(d, "keyword", &sd.f_type.to_string());
                }
                SubroutineType::Type(id) => {
                    self.write_type(d, &id)?;
                }
            };
            write_id_elem!(d, "identifier", sd.name);
            write_element!(d, "symbol", "(");
            self.write_param_list(d, &sd.params, terms)?;
            write_element!(d, "symbol", ")");
            self.write_subroutine_body(d, &sd.body, terms)?;

            d.w.end_element()?;
        }

        Ok(())
    }

    fn write_param_list(
        &self,
        d: &mut XmlWriterData,
        params: &[Param],
        _terms: &[Term],
    ) -> io::Result<()> {
        d.w.start_element("parameterList")?;

        for i in 0..params.len() {
            let ptype = &params[i].p_type;
            self.write_type(d, ptype)?;
            write_id_elem!(d, "identifier", params[i].name);
            if i < params.len() - 1 {
                write_element!(d, "symbol", ",");
            } else {
                write_element!(d, "symbol", ";");
            }
        }
        d.w.end_element()?;

        d.w.set_preserve_whitespaces(false);
        Ok(())
    }

    fn write_subroutine_body(
        &self,
        d: &mut XmlWriterData,
        body: &SubroutineBody,
        terms: &[Term],
    ) -> io::Result<()> {
        d.w.start_element("subroutineBody")?;
        write_element!(d, "symbol", "{");

        self.write_vardecs(d, &body.var_decs, terms)?;
        self.write_stmts(d, &body.stmts, terms)?;

        write_element!(d, "symbol", "}");
        d.w.end_element()?;
        Ok(())
    }

    fn write_vardecs(
        &self,
        d: &mut XmlWriterData,
        vardecs: &[VarDec],
        _terms: &[Term],
    ) -> io::Result<()> {
        if vardecs.is_empty() {
            return Ok(());
        }

        let mut var_type = vardecs[0].var_type;
        d.w.start_element("varDec")?;
        for (i, vd) in vardecs.iter().enumerate() {
            if i > 0 && var_type == vd.var_type {
                write_element!(d, "symbol", ",");
                write_id_elem!(d, "identifier", vd.name);
            } else {
                if i > 0 {
                    write_element!(d, "symbol", ";");
                    d.w.end_element()?;
                    d.w.start_element("varDec")?;
                }
                write_element!(d, "keyword", "var");
                self.write_type(d, &vd.var_type)?;
                write_id_elem!(d, "identifier", vd.name);
            }
            var_type = vd.var_type;
        }

        write_element!(d, "symbol", ";");
        d.w.end_element()?;
        Ok(())
    }

    fn write_stmts(
        &self,
        d: &mut XmlWriterData,
        stmts: &[Statement],
        terms: &[Term],
    ) -> io::Result<()> {
        d.w.start_element("statements")?;
        for stmt in stmts.iter() {
            self.write_stmt(d, stmt, terms)?;
        }
        d.w.end_element()?;
        Ok(())
    }

    fn write_stmt(
        &self,
        d: &mut XmlWriterData,
        stmt: &Statement,
        terms: &[Term],
    ) -> io::Result<()> {
        match stmt {
            Statement::Let(l) => self.write_let(d, l, terms),
            Statement::If(ify) => self.write_if(d, ify, terms),
            Statement::While(whily) => self.write_while(d, whily, terms),
            Statement::Do(ds) => self.write_do(d, ds, terms),
            Statement::Return(ret) => self.write_return(d, ret, terms),
        }
    }

    fn write_let(&self, d: &mut XmlWriterData, l: &LetStmt, terms: &[Term]) -> io::Result<()> {
        d.w.start_element("letStatement")?;

        write_element!(d, "keyword", "let");
        write_id_elem!(d, "identifier", l.name);
        if let Some(idx) = &l.idx {
            write_element!(d, "symbol", "[");
            self.write_expression(d, idx, terms)?;
            write_element!(d, "symbol", "]");
        }
        write_element!(d, "symbol", "=");
        self.write_expression(d, &l.eq_to, terms)?;
        write_element!(d, "symbol", ";");

        d.w.end_element()?;
        Ok(())
    }

    fn write_if(&self, d: &mut XmlWriterData, i: &IfStmt, terms: &[Term]) -> io::Result<()> {
        d.w.start_element("ifStatement")?;

        write_element!(d, "keyword", "if");
        write_element!(d, "symbol", "(");
        self.write_expression(d, &i.cond, terms)?;
        write_element!(d, "symbol", ")");
        write_element!(d, "symbol", "{");
        self.write_stmts(d, &i.body, terms)?;
        write_element!(d, "symbol", "}");

        if !i.else_body.is_empty() {
            write_element!(d, "keyword", "else");
            write_element!(d, "symbol", "{");
            self.write_stmts(d, &i.else_body, terms)?;
            write_element!(d, "symbol", "}");
        }

        d.w.end_element()?;
        Ok(())
    }

    fn write_while(&self, d: &mut XmlWriterData, wh: &WhileStmt, terms: &[Term]) -> io::Result<()> {
        d.w.start_element("whileStatement")?;

        write_element!(d, "keyword", "while");
        write_element!(d, "symbol", "(");
        self.write_expression(d, &wh.cond, terms)?;
        write_element!(d, "symbol", ")");
        write_element!(d, "symbol", "{");
        self.write_stmts(d, &wh.body, terms)?;
        write_element!(d, "symbol", "}");

        d.w.end_element()?;
        Ok(())
    }

    fn write_do(&self, d: &mut XmlWriterData, ds: &DoStmt, terms: &[Term]) -> io::Result<()> {
        d.w.start_element("doStatement")?;

        write_element!(d, "keyword", "do");
        self.write_subroutine_call(d, &ds.call, terms)?;
        write_element!(d, "symbol", ";");

        d.w.end_element()?;
        Ok(())
    }

    fn write_return(
        &self,
        d: &mut XmlWriterData,
        r: &ReturnStmt,
        terms: &[Term],
    ) -> io::Result<()> {
        d.w.start_element("returnStatement")?;

        write_element!(d, "keyword", "return");
        if let Some(expr) = &r.ret_val {
            self.write_expression(d, expr, terms)?;
        }
        write_element!(d, "symbol", ";");

        d.w.end_element()?;
        Ok(())
    }

    fn write_expression(
        &self,
        d: &mut XmlWriterData,
        e: &Expression,
        terms: &[Term],
    ) -> io::Result<()> {
        d.w.start_element("expression")?;
        self.write_term(d, e.init_term, terms)?;
        for (op, term) in &e.ops {
            write_element!(d, "symbol", &op.to_string());
            self.write_term(d, *term, terms)?;
        }
        d.w.end_element()?;

        Ok(())
    }

    fn write_subroutine_call(
        &self,
        d: &mut XmlWriterData,
        call: &SubroutineCall,
        terms: &[Term],
    ) -> io::Result<()> {
        if let Some(caller) = call.caller {
            write_id_elem!(d, "identifier", caller);
            write_element!(d, "symbol", ".");
        }
        write_id_elem!(d, "identifier", call.name);
        write_element!(d, "symbol", "(");
        d.w.start_element("expressionList")?;
        for (i, expr) in call.args.iter().enumerate() {
            self.write_expression(d, expr, terms)?;
            if i < call.args.len() - 1 {
                write_element!(d, "symbol", ",");
            }
        }

        d.w.end_element()?;
        write_element!(d, "symbol", ")");

        Ok(())
    }

    fn write_term(&self, d: &mut XmlWriterData, term: TermId, terms: &[Term]) -> io::Result<()> {
        d.w.start_element("term")?;
        let term = &terms[term];
        match term {
            Term::Int(i) => {
                write_element!(d, "integerConstant", &i.to_string());
            }
            Term::String(s) => {
                write_id_elem!(d, "stringConstant", *s);
            }
            Term::VarName(v) => {
                write_id_elem!(d, "identifier", *v);
            }
            Term::KeywordConstant(k) => {
                write_element!(d, "keyword", &k.to_string());
            }
            Term::ArrayAccess(a) => {
                write_id_elem!(d, "identifier", a.var);
                write_element!(d, "symbol", "[");
                self.write_expression(d, &a.idx, terms)?;
                write_element!(d, "symbol", "]");
            }
            Term::Call(c) => {
                self.write_subroutine_call(d, c, terms)?;
            }
            Term::BracketExpression(b) => {
                write_element!(d, "symbol", "(");
                self.write_expression(d, b, terms)?;
                write_element!(d, "symbol", ")");
            }
            Term::Unary(u) => {
                write_element!(d, "symbol", &u.op.to_string());
                self.write_term(d, u.term, terms)?;
            }
        }
        d.w.end_element()?;

        Ok(())
    }
}
