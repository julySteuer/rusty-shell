use std::{error::Error, fmt};

use pest::{Parser, iterators::Pairs};
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
    right: Call, 
    left: Option<Box<ShellExpr>>
}

#[derive(Debug)]
pub enum ShellExpr {
    Pipe(LeftRecursiveBlock),
    Redirect(LeftRecursiveBlock),
    Call(Call)
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

fn to_tree(expression: Pairs<'_, Rule>) -> Result<ShellExpr, ParserError> {
    call_to_tree(expression) // Fix this 
}

fn call_to_tree(calls: Pairs<'_, Rule>) -> Result<ShellExpr, ParserError> { // The pair here should be a EXPR 
    calls.fold(None, |acc, expr| 
        match expr.as_rule() {
            Rule::CALL => {
                let mut inner_rules = expr.into_inner();

                let prog = inner_rules.next().unwrap().as_str(); // Shall never fail { NAME ~ ARGUMENT* }
                let arguments: Vec<String> = inner_rules.map(|arg| String::from(arg.as_str())).collect(); // Have to check if he really makes name + 1 
                let shell_call = ShellExpr::Call(Call { prog: String::from(prog) , arguments });
                Some(shell_call)
            },
            Rule::PIPE => {
                None
            },
            Rule::REDIRECTION => { 
                None
            },
            _ => unreachable!()
        }
    ).ok_or(ParserError::CouldNotParse) // Start From none
}

fn parse_call(call: &mut Pairs<'_, Rule>) -> ShellExpr { // Maybe start using newtype pattern instead of Pairs 
    let prog = call.next().unwrap().as_str(); // Shall never fail { NAME ~ ARGUMENT* }
    let arguments: Vec<String> = call.map(|arg| String::from(arg.as_str())).collect(); // Have to check if he really makes name + 1 
    ShellExpr::Call(Call { prog: String::from(prog) , arguments })
}