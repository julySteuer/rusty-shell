use std::{error::Error, fmt};

use pest::{Parser, iterators::{Pair, Pairs}};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/shell_input.pest"]
pub struct ShellInput;

#[derive(Debug)]
pub enum ParserError {
    PestParseError,
    CouldNotParse
}

#[derive(Debug)]
pub struct Call {
    pub prog: String,
    pub arguments: Vec<String>
}

#[derive(Debug)]
pub struct LeftRecursiveBlock {
    pub right: Call, 
    pub left: Box<ShellExpr>
}

impl LeftRecursiveBlock {
    pub fn from_shell_expr(shell_call: Call, left: Option<ShellExpr>) -> Self {
        Self { right: shell_call, left: Box::new(left.expect("Left Recursive branch is None")) }
    }
}

#[derive(Debug)]
pub struct Pipe(pub LeftRecursiveBlock);

#[derive(Debug)]
pub struct Redirect(pub LeftRecursiveBlock);

#[derive(Debug)]
pub enum ShellExpr {
    Pipe(Pipe),
    Redirect(Redirect),
    Call(Call) // Add Empty Expression (If the user just presses enter, this also removes the option in the recursive blocks)
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse Error [{:?}] Occured", self)
    }
}

impl Error for ParserError {}

pub fn parse_input(input: &str) -> Result<ShellExpr, ParserError> {
    ShellInput::parse(Rule::EXPR, input)
        .map_err(|_| ParserError::PestParseError)
        .and_then(|expr| to_tree(expr)) // Maybe to_tree -> expr_to_tree to abstract 
}

fn to_tree(mut expression: Pairs<'_, Rule>) -> Result<ShellExpr, ParserError> {
    let mut ex = expression.next().unwrap().into_inner();
    call_to_tree(&mut ex) 
}

fn call_to_tree(calls: &mut Pairs<'_, Rule>) -> Result<ShellExpr, ParserError> { // The pair here should be a EXPR 
    calls.fold(None, |acc, expr| // This fold can may be started without the None. Would clean things up 
        match expr.as_rule() { 
            Rule::CALL => {
                let shell_call = parse_call(expr.into_inner());
                Some(ShellExpr::Call(shell_call))
            },
            Rule::PIPE => { // { CALL ~ whitespaces_opt ~ "|" }
                let mut iterator = expr.into_inner();
                let shell_call = parse_call(iterator.next().unwrap().into_inner());
                let pipe = ShellExpr::Pipe(Pipe(LeftRecursiveBlock::from_shell_expr(shell_call, acc)));
                Some(pipe)
            },
            Rule::REDIRECTION => { // { CALL ~ whitespaces_opt ~ ">" }
                let mut iterator = expr.into_inner();
                let shell_call = parse_call(iterator.next().unwrap().into_inner());
                let redirect = ShellExpr::Redirect(Redirect(LeftRecursiveBlock::from_shell_expr(shell_call, acc)));
                Some(redirect)
            },
            _ => unreachable!()
        }
    ).ok_or(ParserError::CouldNotParse)
}

// Error for mangeld arguments is in this method 
fn parse_call(mut call: Pairs<'_, Rule>) -> Call { // Maybe start using newtype pattern instead of Pairs
    let prog = call.next().unwrap(); // Shall never fail { NAME ~ ARGUMENT* }
    let arguments: Vec<String> = call.map(|arg| String::from(arg.as_str().trim())).collect(); // Have to check if he really makes name + 1 
    Call { prog: String::from(prog.as_str()), arguments }
}