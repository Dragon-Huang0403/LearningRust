use std::io;

fn main() {
    degree_f_to_degree_c();

    println!("Please enter a integer for fib function");
    let num = get_stdio().trim().parse().unwrap();
    let result = fib(num);
    println!("{result}")
}

fn get_stdio() -> String {
    let mut result = String::new();

    io::stdin()
        .read_line(&mut result)
        .expect("Failed to read line");

    result
}

fn parse_string_to_f64(str: String) -> f64 {
    let result: f64 = str.trim().parse().unwrap();
    result
}

fn degree_f_to_degree_c() {
    println!("Please input temperature in fahrenheit");

    let fahrenheit: f64 = parse_string_to_f64(get_stdio());

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("The temperature in Celsius is: {celsius}");
}

fn fib(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
}
