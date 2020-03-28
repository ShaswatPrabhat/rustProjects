fn main() {
    let s = String::from("Bubu Zuzu");
    let s1 = &s[..2];
    let s2 = &s[3..];

    println!("Slice values {} and {}", s1, s2);
}
