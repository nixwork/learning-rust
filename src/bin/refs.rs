fn main() {
    let mut s1 = String::from("hello");
    let r1 = &s1;
    let r2 = &s1;
    println!("r1 = {}; r2 = {}", r1, r2);
    let r3 = &mut s1;
    println!("r3 = {}", r3);
    change(&mut s1);
    let len = calculate_length(&s1);
    println!("Length of {} is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
