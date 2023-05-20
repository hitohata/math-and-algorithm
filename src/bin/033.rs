use proconio::input;

fn main() {

    input! {
        a: (isize, isize),
        b: (isize, isize),
        c: (isize, isize)
    }

    let ba_vector = (a.0 - b.0, a.1 - b.1);
    let bc_vector = (c.0 - b.0, c.1 - b.1);
    let ca_vector = (a.0 - c.0, a.1 - c.1);
    let cb_vector = (b.0 - c.0, b.1 - c.1);

    // ba . bc < 90 degree
    if ba_vector.0 * bc_vector.0 + ba_vector.1 * bc_vector.1 < 0 {
        println!("{0:.12}", f64::sqrt((ba_vector.0.pow(2) + ba_vector.1.pow(2)) as f64));
        return;
    }

    // ca . cb < 90 degree
    if ca_vector.0 * cb_vector.0 + ca_vector.1 * cb_vector.1 < 0 {
        println!("{0:.12}", f64::sqrt((ca_vector.0.pow(2) + ca_vector.1.pow(2)) as f64));
        return;
    }

    // square
    let cross_product = isize::abs(ba_vector.0 * ca_vector.1 - ba_vector.1 * ca_vector.0) as f64;

    let bc_length = f64::sqrt((bc_vector.0.pow(2) + bc_vector.1.pow(2)) as f64);

    println!("{0:.12}", cross_product / bc_length);

}
