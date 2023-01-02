fn encrypt(text: &str, shift: i16) -> String {
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;
    let diff: i16 = code_z - code_a + 1;
    let mut res = String::new();
    for ch in text.chars() {
        let mut code = ch as i16;
        if code_a <= code && code <= code_z {
            code = (code - code_a + shift + 10 * diff) % diff + code_a;
        }
        res.push((code as u8) as char);
    }
    return res;
}

fn main() {
    let enc = encrypt("I LOVE YOU", 3);
    let dec = encrypt(&enc, -3);
    println!("{}", enc);
    println!("{}", dec);
}
