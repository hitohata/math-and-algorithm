use proconio::input;

fn main() {

    input! {
        n: usize,
        m: usize
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for _ in 0..m {

        input! { a: usize, b: usize }

        graph[a].push(b);
        graph[b].push(a);

    }

    let mut answer = 0;

    for i in 1..=n {

        let mut count = 0;

        for j in 0..graph[i].len() {
            if graph[i][j] < i {
                count += 1
            }
        }

        if count == 1 { answer += 1 };

    }

    println!("{}", answer);

}
