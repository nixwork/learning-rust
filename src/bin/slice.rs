fn main() {
    let s = String::from("Hello world");
    let s_literal = "Hello world";

    println!("The first word is {}", first_word(&s));
    println!("The first word is {}", first_word(&s[..]));
    println!("The first word is {}", first_word(s_literal));
    println!("The first word is {}", first_word(&s_literal[..]));

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &a[2..3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
