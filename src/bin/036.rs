use std::f64::consts::PI;

use proconio::input;

fn main() {

    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64
    }

    let h_angle = 30.0 * h + 0.5 * m;
    let m_angle = 6.0 * m;

    let h_angle_rad = h_angle * PI / 180.0;
    let m_angle_rad = m_angle * PI / 180.0;

    let (h_x, h_y) = (a * h_angle_rad.cos(), a * h_angle_rad.sin());
    let (m_x, m_y) = (b * m_angle_rad.cos(), b * m_angle_rad.sin());

    println!("{0:.12}", f64::sqrt((h_x - m_x).powf(2.0) + (h_y - m_y).powf(2.0)))

}
