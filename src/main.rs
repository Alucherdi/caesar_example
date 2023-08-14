struct Caesar {}
impl Caesar {
    pub fn encrypt(plain: &str, offset: u8) -> String {
        plain
            .chars()
            .map(|v| ((((v as u8 - 97) + offset) % 26) + 97) as char)
            .collect()
    }

    pub fn decrypt(enc: &str, offset: u8) -> String {
        enc
            .chars()
            .map(|v| {
                let u: i8 = (v as i8) - 97 - (offset as i8);
                (((u + if u < 0 { 26 } else { 0 }) + 97) as u8) as char
            }).collect()
    }
}

fn main() {
    let o: String = ('a'..='z').collect();
    let offset: u8 = 11;

    let enc = Caesar::encrypt(&o, offset);
    let dec = Caesar::decrypt(&enc, offset);

    println!("{}", enc);
    println!("{}", dec);
}
