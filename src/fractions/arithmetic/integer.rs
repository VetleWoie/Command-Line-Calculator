use std::ops;
use crate::fractions::Fraction;

impl<T> ops::Mul<T> for Fraction
where i64: From<T>
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs : i64 = rhs.into();
        return Self{
            numerator : self.numerator * rhs,
            denominator : self.denominator
        }.short();
    }
}

impl<T> ops::MulAssign<T> for Fraction
where i64: From<T>
{
    fn mul_assign(&mut self, rhs: T) {
        let rhs : i64 = rhs.into();

        self.numerator *= rhs;
        self.short_self();
    }
}

impl<T> ops::Div<T> for Fraction
where i64: From<T>
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let rhs : i64 = rhs.into();

        return Self{
            numerator : self.numerator,
            denominator : self.denominator * rhs
        }.short()
    }
}

impl<T> ops::DivAssign<T> for Fraction
where i64: From<T>
{
    fn div_assign(&mut self, rhs: T) {
        let rhs : i64 = rhs.into();

        self.denominator *= rhs;
        self.short_self();
    }
}

impl<T> ops::Add<T> for Fraction
where i64: From<T>
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        let rhs : i64 = rhs.into();

        return Self{
            numerator : self.numerator + rhs * self.denominator,
            denominator : self.denominator
        }.short();
    }
}

impl<T> ops::AddAssign<T> for Fraction
where i64: From<T>
{
    fn add_assign(&mut self, rhs: T) {
        let rhs : i64 = rhs.into();

        self.numerator += rhs * self.denominator;
        self.short_self();
    }
}

impl<T> ops::Sub<T> for Fraction
where i64: From<T>
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        let rhs : i64 = rhs.into();

        return Self{
            numerator : self.numerator - rhs * self.denominator,
            denominator : self.denominator
        }.short();
    }
}

impl<T> ops::SubAssign<T> for Fraction
where i64: From<T>
{
    fn sub_assign(&mut self, rhs: T) {
        let rhs : i64 = rhs.into();

        self.numerator -= rhs * self.denominator;
        self.short_self();
    }
}