use crate::compiler::{analyzer::Analyzer, syntax::SyntaxTree};

pub struct NoopAnalyzer;

impl Analyzer for NoopAnalyzer {
    type Output = ();

    fn analyze(&self, _: &SyntaxTree) {}
}
