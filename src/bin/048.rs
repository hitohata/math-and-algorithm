use std::collections::VecDeque;

use proconio::input;

fn main() {

    input! {
        k: usize
    }

    let mut graph = vec![vec![]; k];

    for i in 0..k {
        for j in 0..10 {
            if i == 0 && j == 0 {
                continue;
            }
            graph[i].push((i * 10 + j, j))
        }
    }

    println!("{}", dijkstra(k, &mut graph));

}

fn dijkstra(k: usize, graph: &mut Vec<Vec<(usize, usize)>>) -> usize {
    let mut dist = vec![std::usize::MAX; k];
    let mut used = vec![false; k];

    let mut queue = VecDeque::<(usize, usize)>::new();
    queue.push_back((0, 0));

    while !queue.is_empty() {

        let position = queue.pop_front().unwrap().1;

        if used[position] {
            continue;
        } else {
            used[position] = true;
        }

        for i in graph[position].clone() {
            let to = i.0;
            let cost = if position == 0 {
                i.1
            } else {
                dist[position] + i.1
            };

            println!("{}", cost);
            println!("{}", dist[to]);

            if dist[to] > cost {
                dist[to] = cost;
                queue.push_back((dist[to], to))
            }

        }

    }

    dist[0]

}
