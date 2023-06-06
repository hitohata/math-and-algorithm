use proconio::input;

fn main() {

    input! {
        n: usize,
        _k: usize,
        xy: [(isize, isize); n]
    }

    let mut answer = std::isize::MAX;

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                for l in 0..n {

                    let xl = xy[i].0;
                    let xr = xy[j].0;
                    let yl = xy[k].1;
                    let yr = xy[l].1;

                    if count(xl, xr, yl, yr, &xy) >= _k {
                        let square = ((xr - xl) * (yl - yr)).abs();
                        answer = std::cmp::min(square, answer);
                    }
                }
            }
        }
    }

    println!("{}", answer)

}

fn count(xl: isize, xr: isize, yl: isize, yr: isize, xy: &Vec<(isize, isize)>) -> usize {

    let mut count = 0;

    xy.iter().for_each(|xy| {
        if xl <= xy.0 && xy.0 <= xr && yl <= xy.1 && xy.1 <= yr {
            count += 1;
        }
    });

    count

}
