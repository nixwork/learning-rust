fn main() {
    another_function();
    another_function_param(5);
    another_function_params(5, 6);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function() {
    println!("Another function.");
}

fn another_function_param(x: i32) {
    println!("The value of x is {}", x);
}

fn another_function_params(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}
