fn main() {
    let price = 3950;
    for a in 0..11 {
        for b in 0..4 {
            for c in 0..11 {
                let p = 500 * a + 100 * b + 50 * c;
                if price == p {
                    println!("{},{},{}", a, b, c);
                }
            }
        }
    }
}
