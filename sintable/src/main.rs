fn main() {
    let mut last = 0_f32;
    for n in (0..=90).step_by(5) {
        let n = n as f32;
        let sin = n.to_radians().sin();
        println!("sin({}°) = {:.3}, difference = {:.3}", n, sin, sin - last);
        last = sin;
    }
}
