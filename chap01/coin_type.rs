fn main() {
    let price: i64 = 3950;
    let count500: i64 = 10;
    let count100: i64 = 3;
    let count50: i64 = 10;
    for a in 0..(count500 + 1) {
        for b in 0..(count100 + 1) {
            for c in 0..(count50 + 1) {
                let p = 500 * a + 100 * b + 50 * c;
                if price == p {
                    println!("{},{},{}", a, b, c);
                }
            }
        }
    }
}
