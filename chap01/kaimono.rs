fn main() {
    let pc = 98000.0;
    let a_ship = 1200.0;
    let a_rate = 0.8;
    let b_ship = 0.0;
    let b_rate = 0.9;
    println!("{}", pc * a_rate + a_ship);
    println!("{}", pc * b_rate + b_ship);
}
