fn main() {
    for i in 1..51 {
        let a = i % 10;
        let b = i - 10 * a;
        if a == 3 {
            println!("A");
            continue;
        }
        if b == 3 {
            println!("A");
            continue;
        }
        println!("{}", i);
    }
}
