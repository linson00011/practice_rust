pub fn num_practice() {
    let result = num_practice_add(1, 2);
    println!("1+2={}", result);

    let result = num_practice_division(1, 3);
    println!("1/3={:.20}", result);

    let result = num_practice_module(5, 3);
    println!("5 module 3:{}", result);
}

fn num_practice_add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn num_practice_division(num1: i32, num2: i32) -> f64 {
    num1 as f64 / num2 as f64
}

fn num_practice_module(num1: i32, num2: i32) -> i32 {
    num1 % num2
}
