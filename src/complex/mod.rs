#[derive(Clone)]
pub struct ComplexNumber {
	pub a: f32,
    pub b: f32
}
impl std::fmt::Debug for ComplexNumber {
    fn fmt ( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
        let operator: &str = if self.b.signum() == 1.0 { "+" } else { "-" };

        write!(f, "({} {} {}i)", self.a, operator, self.b )?;

        Ok(())
    }
}
impl ComplexNumber {
    pub fn add ( &mut self, to_add: ComplexNumber ) -> &Self {
        self.a += to_add.a;
        self.b += to_add.b;
        
        self
    }
    pub fn scalar_mult ( &mut self, to_mult: f32 ) -> &Self {
        self.a *= to_mult;
        self.b *= to_mult;

        self
    }
    pub fn mult ( &mut self, to_mult: ComplexNumber ) -> &Self {
        let a = self.a;
        let b = self.b;
        let c = to_mult.a;
        let d = to_mult.b;

        self.a = a * c - b * d;
        self.b = b * c + a * d;

        self
    }
    pub fn conjugate ( &self ) -> Self {
        Self {
            a: self.a,
            b: self.b * -1f32
        }
    }
    pub fn divide ( &mut self, to_divide: ComplexNumber ) -> Option<&Self> {
        if self.a == 0f32 && to_divide.b == 0f32 {
            return None;
        }
        let a = self.a;
        let b = self.b;
        let c = to_divide.a;
        let d = to_divide.b;
    
        self.a = ( a * c + b * d ) / ( c.powi(2) + d.powi(2) );
        self.b = ( b * c - a * d) / ( c.powi(2) + d.powi(2) );

        Some(self)
    }
    pub fn modulus ( &self ) -> f32 {
        ( self.a.powi(2) + self.b.powi(2) ).sqrt()
    }
    pub fn exp ( &mut self ) -> &Self {
        self.a = self.a.exp() * self.b.cos();
        self.b = self.a.exp() * self.b.sin();

        self
    }
    pub fn to_polar ( &self ) -> ComplexPolarNumber {
        let r = self.modulus();
        let theta = ( self.b ).atan2( self.a );
        ComplexPolarNumber {
            r,
            theta
        }
    }
    pub fn arbitrary_exp ( &mut self, to_exp: ComplexNumber ) -> &Self {
        let base_as_polar = self.to_polar();

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
    r: f32,
    theta: f32
}
impl std::fmt::Debug for ComplexPolarNumber {
    fn fmt ( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
        write!(f, "({} units, {} rads)", self.r, self.theta )?;

        Ok(())
    }
}
impl ComplexPolarNumber {
    pub fn to_cartesian( &self ) -> ComplexNumber {
        let a = self.r * self.theta.cos();
        let b = self.r * self.theta.sin();
        ComplexNumber {
            a,
            b
        }
    }
    pub fn mult ( &mut self, to_mult: ComplexPolarNumber ) -> &Self {
        let mut r     = self.r * to_mult.r;
        let mut theta = self.theta + to_mult.theta;
    
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

        self.r = r;
        self.theta = theta;
    
        self
    }
}