use proconio::input;

fn main() {
    input!{
        n: isize
    };

    let mut answer_vector = Vec::<i32>::new();

    for i in 2..=n {
        if is_prime(i as i32) {
            answer_vector.push(i as i32);
        }
    }


    for i in answer_vector.iter() {
        print!("{} ", i);
    }

}

fn is_prime(target: i32) -> bool {
    for i in 2..target {
        if target % i == 0 {
            return false;
        }
    }

    return true;
}
