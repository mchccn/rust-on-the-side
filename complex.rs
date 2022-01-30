#![allow(dead_code, unused)]

mod complex {
    use core::ops::{Add, Sub, Mul, Div, Neg};
    
    #[derive(Debug, Clone, Copy)]
    pub struct Complex {
        imag: f64,
        real: f64,
    }

    impl Complex {
        pub fn real(real: f64) -> Self {
            return Complex { real, imag: 0.0 };
        }

        pub fn imag(imag: f64) -> Self {
            return Complex { imag, real: 0.0 };
        }

        pub fn zero() -> Self {
            return Complex { real: 0.0, imag: 0.0 };
        }

        pub fn new(real: f64, imag: f64) -> Self {
            return Complex { real, imag };
        }
    }

    impl Complex {
        pub fn conjugate(&self) -> Self {
            return Complex { real: self.real, imag: -self.imag };
        }

        pub fn inverse(&self) -> Self {
            let d = self.real * self.real + self.imag * self.imag;

            return Complex { real: self.real / d, imag: self.imag / d };
        }

        pub fn magnitude(&self) -> f64 {
            return (self.real * self.real + self.imag * self.imag).sqrt();
        }

        pub fn to_tuple(&self) -> (f64, f64) {
            return (self.real, self.imag);
        }
    }

    impl Add for Complex {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            return Complex::new(self.real + rhs.real, self.imag + rhs.imag);
        }
    }

    impl Sub for Complex {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self {
            return Complex::new(self.real - rhs.real, self.imag - rhs.imag);
        }
    }

    impl Mul for Complex {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self {
            return Complex::new(self.real * rhs.real - self.imag * rhs.imag, self.imag * rhs.real + self.real * rhs.imag);
        }
    }

    impl Div for Complex {
        type Output = Self;

        fn div(self, rhs: Self) -> Self {
            let d = (rhs * rhs.conjugate()).real;

            let n = self * rhs.conjugate();

            return Complex::new(n.real / d, n.imag / d);
        }
    }

    impl Neg for Complex {
        type Output = Self;

        fn neg(self) -> Self {
            return Complex::new(-self.real, -self.imag);
        }
    }

    impl PartialEq for Complex {
        fn eq(&self, other: &Self) -> bool {
            return self.real == other.real && self.imag == other.imag;
        }

        fn ne(&self, other: &Self) -> bool {
            return !self.eq(other);
        }
    }

    impl Eq for Complex {}

    impl Default for Complex {
        fn default() -> Self {
            return Complex::zero();
        }
    }

    impl ToString for Complex {
        fn to_string(&self) -> String {
            if self.real == 0.0 && self.imag == 0.0 {
                return format!("{}", 0);
            }

            if self.real == 0.0 {
                return format!("{}i", self.imag);
            }

            if self.imag == 0.0 {
                return format!("{}", self.real);
            }

            return if self.imag > 0.0 {
                format!("{} + {}i", self.real, self.imag)
            } else {
                format!("{} - {}i", self.real, self.imag.abs())
            };
        }
    }
}

fn main() {
    use complex::*;

    let a = Complex::new(2.0, 1.0);

    let b = Complex::new(1.0, 2.0);

    println!("Basic operations");
    println!("Add {:?}", a + b);
    println!("Sub {:?}", a - b);
    println!("Mul {:?}", a * b);
    println!("Div {:?}", a / b);
    println!("Neg {:?}", -a);
    println!("Neg {:?}", -b);
    println!("Properties");
    println!("Conjugate {:?}", a.conjugate());
    println!("Conjugate {:?}", b.conjugate());
    println!("Inverse {:?}", a.inverse());
    println!("Inverse {:?}", b.inverse());
    println!("Magnitude {:?}", a.magnitude());
    println!("Magnitude {:?}", b.magnitude());
    println!("To Tuple {:?}", a.to_tuple());
    println!("To Tuple {:?}", b.to_tuple());
    println!("To String {:?}", a.to_string());
    println!("To String {:?}", b.to_string());

    // 
}
