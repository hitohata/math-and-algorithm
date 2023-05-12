use proconio::input;

fn main() {
    input! {
        n: isize
    }

    let mut answer = Vec::<isize>::new();
    let mut divided = n;

    for i in (2..=n).take_while(|&j| j * j <=n) {
        while divided % i == 0 {
            answer.push(i);
            divided /= i;
        }
    }

    if divided != 1 {
        answer.push(divided);
    }

    for i in answer {
        print!("{} ", i);
    }

}
