use proconio::input;

fn main() {

    input! {
        n: usize,
        mut a: [usize; n]
    }

    sort(& mut a, 0, n);

    a.iter().for_each(|x| print!("{} ", x))

}

fn sort(array: &mut [usize], l: usize, r: usize) {

    if r - l == 1 {
        return;
    }

    let m = (r + l) / 2;

    sort(array, l, m);
    sort(array, m, r);

    let mut c1 = l;
    let mut c2 = m;
    let mut count = 0;
    let mut c = Vec::<usize>::new();

    while c1 != m || c2 != r {
        if c1 == m {
            c.push(array[c2]);
            c2 += 1;
        }
        else if c2 == r {
            c.push(array[c1]);
            c1 += 1;
        }

        else {
            if array[c1] < array[c2] {
                c.push(array[c1]);
                c1 += 1;
            } else {
                c.push(array[c2]);
                c2 += 1;
            }
        }

        count += 1;

    }

    for i in 0..count {
        array[l + i] = c[i]
    }


}
