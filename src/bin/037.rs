use proconio::input;

fn main() {

    input! {
        a: (isize, isize),
        b: (isize, isize),
        c: (isize, isize),
        d: (isize, isize)
    }

    let ab_vector = to_vector(b, a);
    let ac_vector = to_vector(c, a);
    let ad_vector = to_vector(d, a);

    let cd_vector = to_vector(d, c);
    let ca_vector = to_vector(a, c);
    let cb_vector = to_vector(b, c);

    let cross_cab = cross_product_value(ab_vector, ac_vector);
    let cross_dab = cross_product_value(ab_vector, ad_vector);
    let cross_acd = cross_product_value(cd_vector, ca_vector);
    let cross_bcd = cross_product_value(cd_vector, cb_vector);

    if cross_cab == 0 && cross_dab == 0 && cross_bcd == 0 && cross_acd == 0 {
        let mut a_pair = vec![0; 2];
        let mut b_pair = vec![0; 2];
        let mut c_pair = vec![0; 2];
        let mut d_pair = vec![0; 2];

        if a < b { 
            a_pair = vec![a.0, a.1]; b_pair = vec![b.0, b.1];
        } else {
            a_pair = vec![b.0, b.1]; b_pair = vec![a.0, a.1];
        };
        if c < d {
            c_pair = vec![c.0, c.1]; d_pair = vec![d.0, d.1];
        } else {
            c_pair = vec![d.0, d.1]; d_pair = vec![c.0, c.1];
        };

        if a_pair.max(c_pair) < b_pair.min(d_pair) {
            println!("Yes");
            return;
        } else {
            println!("No");
            return;
        }
    }

    let mut is_ab_divide = false;
    let mut is_cd_divide = false;

    if cross_cab >= 0 && cross_dab <= 0 { is_ab_divide = true; }
    if cross_cab <= 0 && cross_dab >= 0 { is_ab_divide = true; }
    if cross_acd >= 0 && cross_bcd <= 0 { is_cd_divide = true; }
    if cross_acd <= 0 && cross_bcd >= 0 { is_cd_divide = true; }

    if is_ab_divide && is_cd_divide {
        println!("Yes");
    } else {
        println!("No");
    }

}

fn cross_product_value(a: (isize, isize), b: (isize, isize)) -> isize {
    return a.0 * b.1 - a.1 * b.0;
}

fn to_vector(dist: (isize, isize), origin: (isize, isize)) -> (isize, isize) {
    return (dist.0 - origin.0, dist.1 - origin.1);
}
