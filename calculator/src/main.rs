use std::{env::args, process::exit};

fn main() {
    let args: Vec<String> = args().collect();

    if args.get(1).unwrap() == "--help" {
        println!("Usage: ./calculator <operator> <num1> <num2>");
        exit(0);
    }

    println!("raw args: {:?}", args);

    if args.len() != 4 {
        panic!("Usage: ./calculator <operator> num1 num2");
    }

    let op: char = args
        .get(1)
        .unwrap_or_else(|| panic!("The 1st argument is not present!"))
        .chars()
        .next()
        .unwrap();

    let num1: f64 = match args
        .get(2)
        .unwrap_or_else(|| panic!("The 2nd argument is not present!"))
        .parse::<f64>()
    {
        Ok(v) => v,
        Err(e) => panic!("unable to parse num: {:?}", e),
    };

    let num2: f64 = match args
        .get(3)
        .unwrap_or_else(|| panic!("The 3nd argument is not present!"))
        .parse::<f64>()
    {
        Ok(v) => v,
        Err(e) => panic!("unable to parse num: {:?}", e),
    };

    println!("op: {}", op);
    println!("num1: {}", num1);
    println!("num2: {}", num2);

    let result: f64 = perform_op(op, num1, num2);
    println!("Result: {}", result);
}

fn perform_op(op: char, num1: f64, num2: f64) -> f64 {
    // use https://docs.rs/float-cmp/latest/float_cmp/ to approximately compare floating points
    if num2 == 0.0 && op == '/' {
        panic!("can't divide a number with 0");
    }

    match op {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '/' => num1 / num2,
        '*' => num1 * num2,
        _ => panic!("operator doesn't match the valid operations"),
    }
}
