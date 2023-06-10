use core::fmt;

use proconio::input;

const MOD: usize = 1_000_000_000;

fn main() {

    input! {
        n: usize
    }

    let mut origin = Matrix::new();
    origin.fib_init();

    let after = power(origin, n - 1);

    println!("{}", (after.get_value(1, 0) + after.get_value(1, 1)) % MOD);

}

#[derive(Clone, Copy, Debug)]
struct Matrix {
    data: [[usize; 2]; 2]
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(0, 0): {}, (0, 1): {}, (1, 0): {}, (1, 1): {}", self.get_value(0, 0), self.get_value(0, 1), self.get_value(1, 0), self.get_value(1, 1))
    }
}

impl Matrix {
    fn new() -> Self {
        Matrix { data: [[0, 0], [0, 0]] }
    }

    fn fib_init(&mut self) {
        self.data = [[1, 1], [1, 0]]
    }

    fn get_value(&self, col: usize, row: usize) -> usize {
        self.data[col][row]
    }

}

fn multiplication(a: &Matrix,  b: &Matrix) -> Matrix {

        let mut multiplied_matrix  = Matrix::new();

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    multiplied_matrix.data[i][k] += a.get_value(i, j) * b.get_value(j, k);
                    multiplied_matrix.data[i][k] %= MOD;
                }
            }
        }

        multiplied_matrix
}

fn power(a: Matrix, n: usize) -> Matrix {
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
