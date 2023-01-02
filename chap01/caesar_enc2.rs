fn encrypt(text: &str, shift: i16) -> String {
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;
    let diff: i16 = code_z - code_a + 1;
    let conv = |x| (x - code_a + shift + 10 * diff) % diff + code_a;
    let enc = |x| {
        if code_a <= (x as i16) && (x as i16) <= code_z {
            (conv(x as i16) as u8) as char
        } else {
            x
        }
    };
    return text.chars().map(|c| enc(c)).collect();
}

fn main() {
    let enc = encrypt("I LOVE YOU", 3);
    let dec = encrypt(&enc, -3);
    println!("{}", enc);
    println!("{}", dec);
}
