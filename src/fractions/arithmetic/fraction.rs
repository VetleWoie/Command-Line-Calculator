use std::ops;
use crate::fractions::Fraction;

impl ops::Mul for Fraction{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self{
            numerator : self.numerator * rhs.numerator,
            denominator : self.denominator * rhs.denominator
        }.short()
    }
}

impl ops::MulAssign for Fraction{
    fn mul_assign(&mut self, rhs: Self) {
        self.numerator *= rhs.numerator;
        self.denominator *= rhs.denominator;
        self.short_self();
    }
}

impl ops::Div for Fraction{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self{
            numerator : self.numerator * rhs.denominator,
            denominator : self.denominator * rhs.numerator
        }.short()
    }
}

impl ops::DivAssign for Fraction{
    fn div_assign(&mut self, rhs: Self) {
        self.numerator *= rhs.denominator;
        self.denominator *= rhs.numerator;
        self.short_self();
    }
}

impl ops::Add for Fraction{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            numerator : self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            denominator : self.denominator * rhs.denominator
        }.short()
    }
}

impl ops::AddAssign for Fraction{
    fn add_assign(&mut self, rhs: Self) {
        self.numerator *= self.numerator * rhs.denominator + rhs.numerator * self.denominator;
        self.denominator *= rhs.denominator;
        self.short_self();
    }
}

impl ops::Sub for Fraction{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            numerator : self.numerator * rhs.denominator - rhs.numerator * self.denominator,
            denominator : self.denominator * rhs.denominator
        }.short()
    }
}

impl ops::SubAssign for Fraction{
    fn sub_assign(&mut self, rhs: Self) {
        self.numerator *= self.numerator * rhs.denominator - rhs.numerator * self.denominator;
        self.denominator *= rhs.denominator;
        self.short_self();
    }
}

impl PartialEq for Fraction{
    fn eq(&self, other: &Self) -> bool {
        let shorted_self = self.short();
        let shorted_other = other.short();

        (shorted_self.numerator == shorted_other.numerator) && (shorted_self.denominator == shorted_other.denominator)
    }
}