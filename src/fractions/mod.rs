use std::{fmt::Display, str::FromStr};
pub mod arithmetic;

pub use arithmetic::*;

#[derive(Debug, Clone, Copy)]
pub struct Fraction{
    pub numerator: i64,
    pub denominator: i64,
}

impl Fraction{
    pub fn short(self) -> Self{
        let div = gcd(self.numerator, self.denominator);
        Fraction { 
            numerator: self.numerator / div, 
            denominator: self.denominator / div 
        }
    }

    fn short_self(&mut self){
        let div = gcd(self.numerator, self.denominator);
        self.numerator /= div; 
        self.denominator /= div; 
    }
}

impl Display for Fraction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.denominator {
            1 =>write!(f,"{}",self.numerator),
            _ =>write!(f,"{}/{}",self.numerator,self.denominator) 
        }
        
    }
}

impl Into<f32> for Fraction{
    fn into(self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }
}

impl Into<f64> for Fraction{
    fn into(self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

pub struct ParseFractionError;
impl FromStr for Fraction{
    //TODO: Parse full fraction not only integers
    type Err = ParseFractionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>(){
            Ok(num) => Ok(Self{
                numerator : num,
                denominator: 1
            }),
            Err(_) => Err(ParseFractionError)
        }
    }
}

fn gcd(a: i64, b:i64) -> i64{
    if a == 0{
        return b;
    }
    return gcd(b % a, a);
}