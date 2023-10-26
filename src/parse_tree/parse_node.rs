use std::ops;

#[derive(Debug)]
pub enum ParseNode<T>
where T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + ops::Div<Output = T>
{
    Add{ left: Box<ParseNode<T>>, right: Box<ParseNode<T>> },
    Subtract{ left: Box<ParseNode<T>>, right: Box<ParseNode<T>> },
    Multiplication{ left: Box<ParseNode<T>>, right: Box<ParseNode<T>> },
    Division{ left: Box<ParseNode<T>>, right: Box<ParseNode<T>> },
    Number(T)
}

impl<T> ParseNode<T>
where T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + ops::Div<Output = T>
{
    pub fn eval(self) -> T{
        match self {
            ParseNode::Add{ left, right } => {
                left.eval() + right.eval()
            }
            ParseNode::Subtract{ left, right } => {
                left.eval() - right.eval()
            }
            ParseNode::Multiplication{ left, right } => {
                left.eval() * right.eval()
            }
            ParseNode::Division{ left, right } => {
                left.eval() / right.eval()
            }
            ParseNode::Number(number) => {
                number
            }
        }
    }
}
