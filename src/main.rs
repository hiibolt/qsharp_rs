#[derive(Debug, Clone)]
struct ComplexNumber {
	a: f32,
    b: f32
}
impl ComplexNumber {
    fn add ( &mut self, to_add: &ComplexNumber ) -> &Self {
        let real = self.a + to_add.a;
        let imaginary = self.b + to_add.b;
        
        self
    }
    fn conjugate ( &self ) -> Self {
        Self {
            a: self.a,
            b: self.b * -1f32
        }
    }
    fn divide ( &mut self, to_divide: ComplexNumber ) -> Option<&Self> {
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
    fn modulus ( &self ) -> f32 {
        ( self.a.powi(2) + self.b.powi(2) ).sqrt()
    }
    fn exp ( &mut self ) -> &Self {
        self.a = self.a.exp() * self.b.cos();
        self.b = self.a.exp() * self.b.sin();

        self
    }
    fn to_polar ( &self ) -> ComplexPolarNumber {
        let r = self.modulus();
        let theta = ( self.b ).atan2( self.a );
        ComplexPolarNumber {
            r,
            theta
        }
    }
    fn arbitrary_exp ( &mut self, to_exp: ComplexNumber ) -> &Self {
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

#[derive(Debug, Clone)]
struct ComplexPolarNumber {
    r: f32,
    theta: f32
}
impl ComplexPolarNumber {
    fn to_cartesian( &self ) -> ComplexNumber {
        let a = self.r * self.theta.cos();
        let b = self.r * self.theta.sin();
        ComplexNumber {
            a,
            b
        }
    }
    fn multiply ( &mut self, to_mult: ComplexPolarNumber ) -> &Self {
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



struct Matrice {
    value: Vec<Vec<ComplexNumber>>,
    rows: usize,
    cols: usize
}
impl Matrice {
    fn new(value: Vec<Vec<ComplexNumber>>) -> Self {
        if !value.iter().all(|i| i.len() == value[0].len()) {
            panic!("Matrix is not two-dimensional! Ensure all rows are equal in length.");
        }
        let rows = value.len();
        let cols = value[0].len();
        Self {
            value,
            rows,
            cols
        }
    }
    fn from_dimensions( rows: usize, cols: usize ) -> Self {
        let row: Vec<ComplexNumber> = vec![ComplexNumber { a: 0f32, b: 0f32 }; cols];
        let mut ret: Vec<Vec<ComplexNumber>> = Vec::new();
        for _ in 0..rows {
            ret.push( row.clone() );
        }
        Matrice::new(ret)
    }
    fn add( &mut self, to_add: Matrice ) {
        if self.rows != to_add.rows || self.cols != to_add.cols {
            panic!("Matrix size {}x{} doesn't match the base size {}x{}", to_add.rows, to_add.cols, self.rows, self.cols);
        }
        for row in 0..self.value.len() {
            for col in 0..self.value[row].len() {
                self.value[row][col].add( &to_add.value[row][col] );
            }
        }
    }
}
impl std::fmt::Debug for Matrice {
    fn fmt ( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
        write!(f, "{}x{}\n", self.rows, self.cols )?;
        for row in &self.value {
            write!(f, "{}\n", row.into_iter().map(|i| format!("{} +{}i", i.a, i.b)).collect::<Vec<String>>().join(", ") )?;
        }
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");

    let mut matrix = Matrice::new(vec![
        vec![ ComplexNumber { a: 0f32, b: 1f32}, ComplexNumber { a: 0f32, b: 1f32} ],
        vec![ ComplexNumber { a: 0f32, b: 1f32}, ComplexNumber { a: 0f32, b: 1f32} ]]);

    matrix.add(Matrice::new(vec![
        vec![ ComplexNumber { a: 1f32, b: 1f32}, ComplexNumber { a: 1f32, b: 1f32} ],
        vec![ ComplexNumber { a: 1f32, b: 1f32}, ComplexNumber { a: 1f32, b: 1f32} ]]));
    
    println!("{:?}", matrix);
}