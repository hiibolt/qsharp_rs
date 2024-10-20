use std::ops::{
    Add,
    Sub,
    Mul,
    Div,

    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign
};

#[derive(Clone, PartialEq)]
pub struct ComplexNumber {
	pub a: f32,
    pub b: f32
}
impl std::fmt::Debug for ComplexNumber {
    fn fmt ( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
        let operator: &str = if self.b.signum() == 1.0 { "+" } else { "-" };

        write!(f, "({} {} {}i)", self.a, operator, self.b.abs())?;
        Ok(())
    }
}
impl Add for ComplexNumber {
    type Output = Self;

    fn add ( self, to_add: Self ) -> Self {
        Self { a: self.a + to_add.a, b: self.b + to_add.b }
    }
}
impl AddAssign for ComplexNumber {
    fn add_assign ( &mut self, to_add: Self ) {
        self.a += to_add.a;
        self.b += to_add.b;
    }
}
impl Sub for ComplexNumber {
    type Output = Self;

    fn sub ( self, to_sub: Self ) -> Self {
        Self { a: self.a - to_sub.a, b: self.b - to_sub.b }
    }
}
impl SubAssign for ComplexNumber {
    fn sub_assign ( &mut self, to_sub: Self ) {
        self.a -= to_sub.a;
        self.b -= to_sub.b;
    }
}
impl Mul<f32> for ComplexNumber {
    type Output = Self;

    fn mul ( self, to_mul: f32 ) -> Self {
        Self { a: self.a * to_mul, b: self.b * to_mul }
    }
}
impl Mul<ComplexNumber> for ComplexNumber{
    type Output = Self;

    fn mul ( self, to_mul: Self ) -> Self {
        let a = self.a;
        let b = self.b;
        let c = to_mul.a;
        let d = to_mul.b;

        Self {
            a: a * c - b * d,
            b: b * c + a * d
        }
    }
}
impl MulAssign<f32> for ComplexNumber {
    fn mul_assign ( &mut self, to_mul: f32 ) {
        self.a *= to_mul;
        self.b *= to_mul;
    }
}
impl MulAssign<ComplexNumber> for ComplexNumber {
    fn mul_assign ( &mut self, to_mul: ComplexNumber ) {
        let a = self.a;
        let b = self.b;
        let c = to_mul.a;
        let d = to_mul.b;

        self.a = a * c - b * d;
        self.b = b * c + a * d;
    }
}
impl Div<f32> for ComplexNumber {
    type Output = Self;

    fn div ( self, to_div: f32 ) -> Self {
        Self { a: self.a / to_div, b: self.b / to_div }
    }
}
impl Div<ComplexNumber> for ComplexNumber {
    type Output = Self;

    fn div ( self, to_div: ComplexNumber ) -> Self {
        if to_div.a == 0f32 && to_div.b == 0f32 {
            panic!("Denominator cannot be zero!");
        }

        let a = self.a;
        let b = self.b;
        let c = to_div.a;
        let d = to_div.b;
    
        Self {
            a: ( a * c + b * d ) / ( c.powi(2) + d.powi(2) ),
            b: ( b * c - a * d) / ( c.powi(2) + d.powi(2) )
        }
    }
}
impl DivAssign<f32> for ComplexNumber {
    fn div_assign ( &mut self, to_div: f32 ) {
        self.a /= to_div;
        self.b /= to_div;
    }
}
impl DivAssign<ComplexNumber> for ComplexNumber {
    fn div_assign ( &mut self, to_div: ComplexNumber ) {
        if to_div.a == 0f32 && to_div.b == 0f32 {
            panic!("Denominator cannot be zero!");
        }

        let a = self.a;
        let b = self.b;
        let c = to_div.a;
        let d = to_div.b;
    
        self.a = ( a * c + b * d ) / ( c.powi(2) + d.powi(2) );
        self.b = ( b * c - a * d) / ( c.powi(2) + d.powi(2) );
    }
}
impl ComplexNumber {
    pub fn conjugate ( &mut self ) -> &Self {
        self.b *= -1f32;
        self
    }
    pub fn modulus ( &self ) -> f32 {
        ( self.a.powi(2) + self.b.powi(2) ).sqrt()
    }
    pub fn exp ( &mut self ) -> &Self {
        self.a = self.a.exp() * self.b.cos();
        self.b = self.a.exp() * self.b.sin();

        self
    }
    pub fn polar ( &self ) -> ComplexPolarNumber {
        let r = self.modulus();
        let theta = ( self.b ).atan2( self.a );
        ComplexPolarNumber {
            r,
            theta
        }
    }
    pub fn arbitrary_exp ( &mut self, to_exp: ComplexNumber ) -> &Self {
        let base_as_polar = self.polar();

        let r = base_as_polar.r;
        let theta = base_as_polar.theta;

        if r == 0f32 {
            // prolly unnessecary but i'm tired
            self.a = 0f32;
            self.b = 0f32;

            return self;
        }

        let c = to_exp.a;
        let d = to_exp.b;

        self.a = ( c * r.ln() - d * theta ).exp() * ( c * theta + d * r.ln() ).cos();
        self.b = ( c * r.ln() - d * theta ).exp() * ( c * theta + d * r.ln() ).sin();

        self
    }
}

#[derive(Clone)]
pub struct ComplexPolarNumber {
    pub r: f32,
    pub theta: f32
}
impl std::fmt::Debug for ComplexPolarNumber {
    fn fmt ( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
        write!(f, "({} units, {} rads)", self.r, self.theta )?;

        Ok(())
    }
}
impl Mul<ComplexPolarNumber> for ComplexPolarNumber {
    type Output = Self;
    
    fn mul ( self, to_mul: Self ) -> Self {
        let mut r     = self.r * to_mul.r;
        let mut theta = self.theta + to_mul.theta;
    
        if r < 0f32 {
            r *= -1f32;
            theta += std::f32::consts::PI;
        }
        while theta > std::f32::consts::PI {
            theta -= std::f32::consts::TAU;
        }
        while theta < -std::f32::consts::PI {
            theta += std::f32::consts::TAU;
        }

        Self {
            r,
            theta
        }
    }
}
impl ComplexPolarNumber {
    pub fn cartesian( self ) -> ComplexNumber {
        let a = self.r * self.theta.cos();
        let b = self.r * self.theta.sin();
        ComplexNumber {
            a,
            b
        }
    }
}
