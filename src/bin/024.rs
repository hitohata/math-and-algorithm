use proconio::input;

fn main() {

    input! {
        n: usize,
        pq: [[f64; 2]; n],
    }

    let mut expected_value_sum: f64 = 0_f64;

    for i in 0..n {
        expected_value_sum += pq[i][1] / pq[i][0];
    }

    println!("{0:.12}", expected_value_sum);

}
