use std::fmt::{Display, Result};

fn main() {
    let f1 = Fp::new(8, 13);
    let f2 = Fp::new(9, 13);
    match f1.add(&f2) {
        None => {
            println!("error: different modulii cannot be added");
        }
        Some(f) => {
            println!("{} + {} = {}", f1, f2, f);
        }
    }
}

#[derive(Debug)]
pub struct Fp {
    value: u32,
    modulus: u32,
}

trait Field {
    fn new(v: u32, m: u32) -> Self;
    fn check_modulii(m1: &u32, m2: &u32) -> bool;
    fn add(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
    // fn mul(&self, other: &Self) -> Self;
    // fn div(&self, other: &Self) -> Self;
    // fn inv(&self) -> Self;
    // fn exp(&self, exp: u32) -> Self;
    // fn sqrt(&self) -> Self;
}

impl Display for Fp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "{}mod{}", self.value, self.modulus)
    }
}

impl Field for Fp {
    fn new(v: u32, m: u32) -> Self {
        // check if m is prime
        Self {
            value: v,
            modulus: m,
        }
    }

    fn check_modulii(m1: &u32, m2: &u32) -> bool {
        if m1 == m2 {
            true
        } else {
            false
        }
    }
    fn add(&self, other: &Self) -> Option<Self> {
        if Self::check_modulii(&self.modulus, &other.modulus) {
            let value = (self.value + other.value) % self.modulus;
            Some(Fp {
                value: value,
                modulus: self.modulus,
            })
        } else {
            None
        }
    }
}
