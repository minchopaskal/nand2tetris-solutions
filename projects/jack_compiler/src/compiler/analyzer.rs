use super::syntax::SyntaxTree;

pub trait Analyzer {
    type Output;
    fn analyze(&self, tree: &SyntaxTree) -> Self::Output;
}
