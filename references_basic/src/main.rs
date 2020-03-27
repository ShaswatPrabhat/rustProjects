fn main() {
    let mut s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("Length of {} is {}", s1, len);

    let s2 = &s1;
    let s3 = &s2;

    println!("Duplicate references {} and {}", s2, s3);

    let s4 = &mut s1;

    s4.push_str("Bubu");
    println!("Mutated string {}", s4)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
