use crate::parse::Ast;

pub fn eval(ast: Ast) -> isize {
    ast.eval()
}
