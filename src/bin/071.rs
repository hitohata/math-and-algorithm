use proconio::input;

fn main() {

    input! {
        n: usize,
    }

    let mut a = vec![0.0; n];
    let mut b = vec![0.0; n];
    let mut c = vec![0.0; n];

    for i in 0..n {
        input! {
            _a: f32,
            _b: f32,
            _c: f32
        }

        a[i] = _a;
        b[i] = _b;
        c[i] = _c;
    }

    let mut answer: f32 = 0.0;

    for i in 0..n {
        for j in i..n {

            if a[i] * b[j] == a[j] * b[i] { continue; };

            let px = (c[i] * b[j] - b[i] * c[j]) as f32 / (a[i] * b[j] - a[j] * b[i]) as f32;
            let py = (c[i] * a[j] - a[i] * c[j]) as f32 / (a[j] * b[i] - a[i] * b[j]) as f32;

            if check(px, py, &a, &b, &c) {
                let sum = px + py;
                if sum > answer {
                    answer = sum;
                }
            }

        }
    }

    println!("{}", answer);
}

fn check(x: f32, y: f32, a: &Vec<f32>, b: &Vec<f32>, c: &Vec<f32>) -> bool {

    for i in 0..a.len() {

        // to include
        if (x - 1e-5) * a[i] + (y - 1e-5) * b[i] > c[i] {
            return false;
        }
    }

    true

}
