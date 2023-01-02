fn main() {
    let showa = 1926;
    let heisei = 1989;
    let reiwa = 2019;

    for y in 1926..2027 {
        print!("西暦{} = ", y);

        if showa <= y && y < heisei {
            println!("昭和{}", y - showa + 1);
        } else if heisei <= y && y < reiwa {
            println!("平成{}", y - heisei + 1);
        } else if reiwa <= y {
            println!("令和{}", y - reiwa + 1);
        }
    }
}
