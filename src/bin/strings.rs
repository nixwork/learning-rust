fn main() {
    // Creating a string with different methods.
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Updating a string.
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 = {}, s2 = {}", s1, s2);
    s1.push('s');
    println!("s1 = {}", s1);

    // Concatenation with '+'.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);

    // Concatenation with format! macro.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s = {}", s);

    // This code will not compile.
    // let s1 = String::from("hello");
    // let h = s1[0];

    // Slicing strings.
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // The code below will panic.
    // let s = &hello[0..1];
    println!("s = {}", s);


    // Iterating over strings.
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

}
