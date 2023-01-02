fn main() {
    for i in 1..11 {
        for j in 1..11 {
            print!("{:3},", i * j);
        }
        println!("");
    }
}
