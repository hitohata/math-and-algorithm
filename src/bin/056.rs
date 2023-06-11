use std::fmt::{self, write};

use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {

    input! {
        n: usize
    }

    let mut origin = Matrix::new();
    origin.init();

    let after = mod_pow(origin, n - 1);

    println!("{}", (2 * after.d[2][0] +  after.d[2][1] + after.d[2][2]) % MOD);

}

#[derive(Clone, Copy)]
struct Matrix {
    d: [[usize; 3]; 3]
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "{} {} {} \n{} {} {}\n{} {} {}", 
            self.d[0][0],
            self.d[0][1],
            self.d[0][2],
            self.d[1][0],
            self.d[1][1],
            self.d[1][2],
            self.d[2][0],
            self.d[2][1],
            self.d[2][2],
        )
    }
}

impl Matrix {
    fn new() -> Self {
        Matrix { d: [[0; 3]; 3] }
    }

    fn init(&mut self) {
        self.d[0][0] = 1;
        self.d[0][1] = 1;
        self.d[0][2] = 1;
        self.d[1][0] = 1;
        self.d[2][1] = 1;
    }
}

fn mod_pow(a: Matrix, n: usize) -> Matrix {
    let mut q = a;
    let mut p = a;
    let mut flag = false; 

    for i in 0..60 {

        if (n & (1 << i)) != 0 {
            if !flag {
                q = p;
                flag = true;
            } else {
                q = multiplication(&q, &p);
            }
        }

        p = multiplication(&p, &p);

    }

    q
}

fn multiplication(a: &Matrix, b: &Matrix) -> Matrix {

    let mut c = Matrix::new();

    c.d[0][0] = (a.d[0][0] * b.d[0][0] + a.d[0][1] * b.d[1][0] + a.d[0][2] * b.d[2][0]) % MOD;
    c.d[0][1] = (a.d[0][0] * b.d[0][1] + a.d[0][1] * b.d[1][1] + a.d[0][2] * b.d[2][1]) % MOD;
    c.d[0][2] = (a.d[0][0] * b.d[0][2] + a.d[0][1] * b.d[1][2] + a.d[0][2] * b.d[2][2]) % MOD;

    c.d[1][0] = (a.d[1][0] * b.d[0][0] + a.d[1][1] * b.d[1][0] + a.d[1][2] * b.d[2][0]) % MOD;
    c.d[1][1] = (a.d[1][0] * b.d[0][1] + a.d[1][1] * b.d[1][1] + a.d[1][2] * b.d[2][1]) % MOD;
    c.d[1][2] = (a.d[1][0] * b.d[0][2] + a.d[1][1] * b.d[1][2] + a.d[1][2] * b.d[2][2]) % MOD;

    c.d[2][0] = (a.d[2][0] * b.d[0][0] + a.d[2][1] * b.d[1][0] + a.d[2][2] * b.d[2][0]) % MOD;
    c.d[2][1] = (a.d[2][0] * b.d[0][1] + a.d[2][1] * b.d[1][1] + a.d[2][2] * b.d[2][1]) % MOD;
    c.d[2][2] = (a.d[2][0] * b.d[0][2] + a.d[2][1] * b.d[1][2] + a.d[2][2] * b.d[2][2]) % MOD;

    c

}
