use proconio::input;

fn main() {

    input! {
        first_circle: (isize, isize, isize),
        second_circle: (isize, isize, isize)
    }

    let r1 = first_circle.2 as f64;
    let r2 = second_circle.2 as f64;
    let distance = f64::sqrt(((first_circle.0 - second_circle.0).pow(2) + (first_circle.1 - second_circle.1).pow(2)) as f64);

    let r_diff = f64::abs(r1 - r2);
    let r_sum = r1 + r2;

    if distance < r_diff {
        println!("{}", 1)
    }

    else if distance == r_diff {
        println!("{}", 2);
    }

    else if r_diff < distance && distance < r_sum {
        println!("{}", 3);
    }

    else if distance == r_sum {
        println!("{}", 4);
    }

    else {
        println!("{}", 5);
    }

}
