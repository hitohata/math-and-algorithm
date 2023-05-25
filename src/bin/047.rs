use proconio::input;

fn main() {

    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut color = vec![0; n + 1];

    for i in 0..m {
        graph[ab[i].0].push(ab[i].1);
        graph[ab[i].1].push(ab[i].0);
    }

    for i in 1..=n {
        if color[i] == 0 {
            color[i] = 1;
            dfs(i, &graph, &mut color)
        }
    }

    let mut answer = true;

    for i in 0..m {
        if color[ab[i].0] == color[ab[i].1] {
            answer = false;
            break;
        }
    }

    if answer {
        println!("Yes")
    } else {
        println!("No")
    }

}

fn dfs(place: usize, graph: &Vec<Vec<usize>>, color: &mut Vec<usize>) {
    graph[place].iter().for_each(|i| {
        if color[i.to_owned()] == 0 {
            color[i.to_owned()] = 3 - color[place];
            dfs(place, graph, color);
        }
    })
}
