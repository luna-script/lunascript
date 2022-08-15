#[derive(Debug)]
pub enum AST {
    BinOp(Box<AST>, String, Box<AST>),
    Number(f64),
}
