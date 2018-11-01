use luther;
use luther_derive;

#[derive(Lexer, Debug)]
pub enum Token {
    #[luther(regex = "function")]
    Function,
    #[luther(regex = "if")]
    If,
    #[luther(regex = "let")]
    Let,
    #[luther(regex = "=")]
    Tis,
    #[luther(regex = "\\(")]
    Pel,
    #[luther(regex = "\\)")]
    Per,
    #[luther(regex = "\\[")]
    Sel,
    #[luther(regex = "\\]")]
    Ser,
    #[luther(regex = "::.*$")]
    Comment,
    #[luther(regex = "[ \n]+")]
    Gap, //  No switching between "tall mode" and "wide mode"
    #[luther(regex = "[0-9]+")]
    Num(String),
    #[luther(regex = "[a-zA-Z][a-zA-Z0-9]*")]
    Ident(String),
    #[luther(regex = "\"[^\"]*\"")]
    Str(String),
}

//  Spans?
#[derive(Debug)]
pub enum Ast {
    Func { body: Box<Ast> },
    If(Box<Ast>, Box<Ast>, Box<Ast>),
    Let(String, Box<Ast>, Box<Ast>),
    Core(Vec<(String, Ast)>),
    Var(String),
    Str(String),
    Num(i32),
}
