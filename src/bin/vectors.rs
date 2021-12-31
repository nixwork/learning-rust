fn main() {
    // Different approaches of creating a vector.
    let v0: Vec<i32> = Vec::new();
    let v1 = vec![0, 1, 2];
    let mut v2 = Vec::new();
    v2.push(0);
    v2.push(1);

    // Reading an element
    let v3 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v3[2];
    println!("The third element is {}", third);

    // The code below will panic.
    // let does_not_exist = &v3[100];
    // let does_not_exist = v3.get(100);

    match v3.get(9) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // The code below won't compile.
    // let v4 = vec![1, 2, 3];
    // let first = &v4[0];
    // v4.push(4);
    // println!("The first element is {}", first);

    // Iterating
    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{}", i);
    }

    // Iterating with changing values
    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        *i += 50;
        println!("{}", i);
    }

    // enum in vector
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let v7 = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &v7 {
        println!("{:?}", i)
    }
}
