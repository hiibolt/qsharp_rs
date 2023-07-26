#[derive(Debug)]
enum ComplexOperationError {
    DivideByZero,
}

#[derive(Debug, Clone)]
struct ComplexNumber {
	a: f32,
    b: f32
}
#[derive(Debug, Clone)]
struct ComplexPolarNumber {
    r: f32,
    theta: f32
}

fn complex_addition ( c1: &ComplexNumber, c2: &ComplexNumber ) -> ComplexNumber {
    let real = c1.a + c2.a;
    let imaginary = c1.b + c2.b;
    
    ComplexNumber {
        a: real,
        b: imaginary
    }
}
fn complex_conjugate ( c1: &mut ComplexNumber ) {
    c1.b *= -1.;
}
fn complex_divide ( c1: ComplexNumber, c2: ComplexNumber ) -> Result<ComplexNumber, ComplexOperationError> {
    if c2.a == 0f32 && c2.b == 0f32 {
        return Err(ComplexOperationError::DivideByZero);
    }
    let a = c1.a;
    let b = c1.b;
    let c = c2.a;
    let d = c2.b;

    let real = ( a * c + b * d ) / ( c.powi(2) + d.powi(2) );
    let imaginary = ( b * c - a * d) / ( c.powi(2) + d.powi(2) );
    Ok(ComplexNumber {
        a: real,
        b: imaginary
    })
}
fn complex_modulus ( c1: &ComplexNumber ) -> f32 {
    ( c1.a.powi(2) + c1.b.powi(2) ).sqrt()
}
fn complex_exponent_e ( c1: ComplexNumber ) -> ComplexNumber {
    let real      = c1.a.exp() * c1.b.cos();
    let imaginary = c1.a.exp() * c1.b.sin();
    ComplexNumber {
        a: real,
        b: imaginary
    }
}
fn cartesian_to_polar ( c1: ComplexNumber ) -> ComplexPolarNumber {
    let r = complex_modulus( &c1 );
    let theta = ( c1.b ).atan2( c1.a );
    ComplexPolarNumber {
        r,
        theta
    }
}
fn polar_to_cartesian ( c1: ComplexPolarNumber ) -> ComplexNumber {
    let a = c1.r * c1.theta.cos();
    let b = c1.r * c1.theta.sin();
    ComplexNumber {
        a,
        b
    }
}
fn polar_complex_multiplication ( c1: ComplexPolarNumber, c2: ComplexPolarNumber ) -> ComplexPolarNumber {
    let mut r = c1.r * c2.r;
    let mut theta = c1.theta + c2.theta;

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

    ComplexPolarNumber {
        r,
        theta
    }
}
fn complex_exponent_arbitrary ( c1: ComplexNumber, c2: ComplexNumber ) -> ComplexNumber {
    let c1_polar = cartesian_to_polar(c1);

    let r = c1_polar.r;
    let theta = c1_polar.theta;

    if r == 0f32 {
        return ComplexNumber {
            a: 0f32,
            b: 0f32
        }
    }

    let c = c2.a;
    let d = c2.b;

    let real = ( c * r.ln() - d * theta ).exp() * ( c * theta + d * r.ln() ).cos();
    let imaginary = ( c * r.ln() - d * theta ).exp() * ( c * theta + d * r.ln() ).sin();

    ComplexNumber {
        a: real,
        b: imaginary
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
                self.value[row][col] = complex_addition( &self.value[row][col], &to_add.value[row][col] );
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