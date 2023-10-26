use std::{iter::Peekable, str::{Chars, FromStr}, ops};
use parse_node::ParseNode;

mod parse_node;

const DIGITS: &str = "1234567890.(";
const TERM: &str ="*/";
const EXPR: &str ="+-";

#[derive(Debug)]
pub struct ParseTree<T> 
where T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + ops::Div<Output = T> + FromStr
{
    root : ParseNode<T>
}

impl<T> ParseTree<T> 
where T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + ops::Div<Output = T> + FromStr
{
    pub fn from_string(expr : &str) -> Result<Self, &'static str> {
        let mut iter = expr.chars().peekable();
        let root = ParseTree { 
            root: Self::parse_expr(&mut iter)?,
        };
        return Ok(root);
    }

    pub fn eval(self) -> T{
        return self.root.eval();
    }
    
    fn parse_expr(iter: &mut Peekable<Chars<'_>>) -> Result<ParseNode<T>,&'static str> {
        let left = Self::parse_term(iter)?;
        let op: Option<char>;

        op = iter.next_if(|&x| {EXPR.contains(x)});
        match op{
            Some(op) => {
                let right = Self::parse_term(iter)?;
                match op {
                    '+' => {
                        Ok(ParseNode::Add { left: Box::new(left), right: Box::new(right) })
                    }
                    '-' => {
                        Ok(ParseNode::Subtract { left: Box::new(left), right: Box::new(right) })
                    }
                    _other => {
                        Err("Unkown operator")
                    }
                }
            }
            _other => Ok(left)
        }
    }

    fn parse_term(iter: &mut Peekable<Chars<'_>>) -> Result<ParseNode<T>,&'static str> {
        let left = Self::parse_factor(iter)?;
        let op: Option<char>;

        op = iter.next_if(|&x| {TERM.contains(x)});
        match op{
            Some(op) => {
                let right = Self::parse_factor(iter)?;
                match op {
                    '*' => {
                        Ok(ParseNode::Multiplication { left: Box::new(left), right: Box::new(right) })
                    }
                    '/' => {
                        Ok(ParseNode::Division { left: Box::new(left), right: Box::new(right) })
                    },
                    _other => Ok(left)
                }
            }
            _other => Ok(left)
        }
    }

    fn parse_factor(iter: &mut Peekable<Chars<'_>>) -> Result<ParseNode<T>,&'static str> {
        let mut digits: Vec<char> = Vec::new();
        let res : Result<ParseNode<T>,&'static str>;

        let mut digit: char = '_';
        while iter.next_if(|&x| {digit = x; DIGITS.contains(x)}).is_some() {
            if digit == '('{
                res = Self::parse_expr(iter);
                match iter.next_if(|&x| {x == ')'}){
                    Some(_) => (),
                    None => {
                        return Err("Missing closing parenthesis");
                    }
                };
                return res;
            }
            digits.push(digit);
        }

        let s: String = digits.iter().collect();
        match s.parse::<T>() {
            Ok(i) => res = Ok(ParseNode::Number(i)),
            Err(_msg) => res = Err("Could not parse float")
        }
        res
    }
}