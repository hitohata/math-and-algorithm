use std::collections::VecDeque;

use proconio::input;

fn main() {

    input! {
        r: usize,
        c: usize,
        s: (usize, usize),
        g: (usize, usize)
    }

    let mut step = vec![vec![-1; c + 1]; r + 1];
    let mut maize = vec![vec![String::new(); c + 1]; r + 1];

    for i in 1..=r {
        input! {
            c_: String
        }
        let mut elements = c_.chars();
        for j in 1..=c {
            maize[i][j] = elements.next().unwrap().to_string();
        }
    }

    let mut queue = VecDeque::<(usize, usize)>::new();
    queue.push_back(s);
    step[s.0][s.1] = 0;

    while !queue.is_empty() {

        let current_place = queue.pop_front().unwrap();
        let current_step = step[current_place.0][current_place.1];

        // left
        if current_place.0 > 1 {
            let place = (current_place.0 - 1, current_place.1);
            if is_space(place, &maize) && is_not_visited(place, &step) {
                step[place.0][place.1] = current_step + 1;
                queue.push_back(place);
            }
        }

        // right
        if current_place.0 < r {
            let place = (current_place.0 + 1, current_place.1);
            if is_space(place, &maize) && is_not_visited(place, &step) {
                step[place.0][place.1] = current_step + 1;
                queue.push_back(place);
            }
        }

        // above
        if current_place.1 > 1 {
            let place = (current_place.0, current_place.1 - 1);
            if is_space(place, &maize) && is_not_visited(place, &step) {
                step[place.0][place.1] = current_step + 1;
                queue.push_back(place);
            }
        }

        // beneath
        if current_place.1 < c {
            let place = (current_place.0, current_place.1 + 1);
            if is_space(place, &maize) && is_not_visited(place, &step) {
                step[place.0][place.1] = current_step + 1;
                queue.push_back(place)
            }
        }

    }

    println!("{}", step[g.0][g.1])

}

fn is_space(place: (usize, usize), maize: &Vec<Vec<String>>) -> bool {
    if maize[place.0][place.1] == "." {
        return true
    }
    return false
}

fn is_not_visited(place: (usize, usize), step: &Vec<Vec<i32>>) -> bool {
    if step[place.0][place.1] == -1 {
        return true
    }
    return false
}
