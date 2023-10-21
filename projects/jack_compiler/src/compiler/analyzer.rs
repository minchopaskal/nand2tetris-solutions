use super::syntax::SyntaxTree;

pub trait Analyzer {
    type Output;

    fn new() -> Self
        where Self: Sized;
    fn analyze(&self, parser: &SyntaxTree) -> Self::Output;
}

pub struct NoopAnalyzer;
impl Analyzer for NoopAnalyzer {
    type Output = ();

    fn new() -> NoopAnalyzer {
        NoopAnalyzer
    }

    fn analyze(&self, _: &SyntaxTree) { }
}

pub struct XMLAnalyzer;
impl Analyzer for XMLAnalyzer {
    type Output = ();

    fn new() -> XMLAnalyzer {
        XMLAnalyzer
    }

    fn analyze(&self, tree: &SyntaxTree) {

    }
}