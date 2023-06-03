use proconio::input;

fn main() {

    input! {
        n: usize,
        k: isize,
        a_input: [usize; n],
    }

    let mut a = a_input.to_vec();
    a.insert(0, 0);

    let mut first = vec![0_isize; n + 1];
    let mut second = vec![0_isize; n + 1];

    let mut current = 1;
    let mut count = 0;

    loop {

        if first[current] == 0 { first[current] = count }
        else if second[current] == 0 { second[current] = count };

        if k == count {
            println!("{}", current);
            break;
        }

        if second[current] > 0 && ((k - first[current]) % (second[current] - first[current]) == 0) {
            println!("{}", current);
            break;
        }

        current = a[current];
        count += 1;
    }

}
