use proconio::input;

fn main() {

    input! {
        n: usize,
        m: usize
    }

    let mut answer = true;

    let mut visited = vec![false; n + 1];
    let mut graph = vec![vec![]; n + 1];

    (1..=m).for_each(|_| {
        input! {
            a: usize,
            b: usize
        }

        graph[a].push(b);
        graph[b].push(a);

    });

    dfs(1, &graph, &mut visited);

    for i in 1..=n {
        if !visited[i] {
            answer = false;
            break;
        }
    }

    println!("{}", if answer {"The graph is connected."} else {"The graph is not connected."} )

}

fn dfs(position: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[position] = true;

    graph[position].iter().for_each(|to| {

        if !visited[to.to_owned()] {
            dfs(to.to_owned(), graph, visited)
        }

    });

}
