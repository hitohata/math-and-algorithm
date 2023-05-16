use proconio::input;

fn main() {

    input! {
        n: usize,
        a: [usize; n]
    }

    let mut answer = 0;

    for i in 0..n {
        if a[i] > 1000  { continue; };
        for j in i+1..n {
            if a[i] + a[j] > 1000  { continue; };
            for k in j+1..n {
                if a[i] + a[j] + a[k] > 1000  { continue; };
                for l in k+1..n {
                    if a[i] + a[j] + a[k] + a[l] > 1000  { continue; };
                    for m in l+1..n {
                        if a[i] + a[j] + a[k] + a[l] + a[m] == 1000 {
                            answer += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", answer);

}
