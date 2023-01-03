use std::fmt::{Display, Result};

fn main() {
    let f1 = Fp::new(23, 524269).unwrap();
    let f2 = Fp::new(123, 524269).unwrap();

    match f1.add(&f2) {
        None => {
            println!("error: invalid input");
        }
        Some(f) => {
            println!("{} + {} = {}", f1, f2, f);
        }
    }

    match f1.sub(&f2) {
        None => {
            println!("error: invalid input");
        }
        Some(f) => {
            println!("{} - {} = {}", f1, f2, f);
        }
    }

    match f1.mul(&f2) {
        None => {
            println!("error: invalid input");
        }
        Some(f) => {
            println!("{} x {} = {}", f1, f2, f);
        }
    }

    println!("inverse of {} = {}", f1, f1.inv().unwrap());
}

#[derive(Debug)]
pub struct Fp {
    value: i32,
    modulus: i32,
}

trait Field {
    fn new(v: i32, m: i32) -> Option<Self>
    where
        Self: Sized;
    fn check_input(&self, other: &Self) -> bool;
    fn add(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
    fn sub(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
    fn mul(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
    fn inv(&self) -> Option<Self>
    where
        Self: Sized;
    fn extended_euclidean(a: i32, b: i32) -> (i32, i32, i32);
}

impl Display for Fp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "{}mod{}", self.value, self.modulus)
    }
}

impl Field for Fp {
    fn new(v: i32, m: i32) -> Option<Self> {
        // check if m is prime
        if m <= 1 {
            return None;
        }

        let mut i: i32 = 2;
        while i * i <= m {
            if m % i == 0 {
                return None;
            }
            i += 1;
        }
        Some(Self {
            value: v,
            modulus: m,
        })
    }

    fn check_input(self: &Self, other: &Self) -> bool {
        if self.value < self.modulus && other.value < other.modulus && self.modulus == other.modulus
        {
            true
        } else {
            false
        }
    }

    fn add(self: &Self, other: &Self) -> Option<Self> {
        if Self::check_input(&self, &other) {
            let value = (self.value + other.value) % self.modulus;
            Some(Fp {
                value: value,
                modulus: self.modulus,
            })
        } else {
            None
        }
    }

    fn sub(self: &Self, other: &Self) -> Option<Self> {
        if Self::check_input(&self, &other) {
            let mut value = self.value - other.value;
            if value < 0 {
                value += self.modulus;
            }

            Some(Fp {
                value: value,
                modulus: self.modulus,
            })
        } else {
            None
        }
    }

    fn mul(self: &Self, other: &Self) -> Option<Self> {
        if Self::check_input(&self, &other) {
            let value = (self.value * other.value) % self.modulus;
            Some(Fp {
                value: value,
                modulus: self.modulus,
            })
        } else {
            None
        }
    }

    fn inv(self: &Self) -> Option<Self> {
        let (_gcd, mut u, _v) = Self::extended_euclidean(self.value, self.modulus);
        if u < 0 {
            u += self.modulus;
        }
        Some(Fp {
            value: u,
            modulus: self.modulus,
        })
    }

    fn extended_euclidean(a: i32, b: i32) -> (i32, i32, i32) {
        if b == 0 {
            return (a, 1, 0);
        }
        let (gcd, x, y) = Self::extended_euclidean(b, a % b);
        return (gcd, y, x - (a / b) * y);
    }
}
