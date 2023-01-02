fn main() {
    let mut a = 1;
    let mut b = 1;
    println!("{}", a);
    println!("{}", b);
    for _i in 0..30 {
        let tmp = a + b;
        a = b;
        b = tmp;
        println!("{}", b);
    }
}
