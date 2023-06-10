use std::sync::Mutex;

use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {

    input! {
        n: usize
    }

    let mut origin = Matrix::new();
    origin.d[0][0] = 2;
    origin.d[1][0] = 1;
    origin.d[0][1] = 1;
    origin.d[1][1] = 0;

    let after = mod_pow(origin, n - 1);

    println!("{}", (after.d[1][0] + after.d[1][1]) % MOD);

}

#[derive(Clone, Copy)]
struct Matrix {
    d: [[usize; 2]; 2]
}

impl Matrix {
    fn new() -> Self {
        Matrix { d: [[0, 0], [0, 0]] }
    }
}

fn mod_pow(a: Matrix, n: usize) -> Matrix {

    let mut p = a;
    let mut q = Matrix::new();
    let mut flag = false;

    for i in 0..60 {
        if (n & 1 << i) != 0 {
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

    let mut m = Matrix::new();

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                m.d[i][k] += a.d[i][j] * b.d[j][k];
                m.d[i][k] %= MOD;
            }
        }
    }

    m
}
