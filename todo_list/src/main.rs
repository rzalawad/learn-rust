use std::{env::args, process::exit};

fn main() {
    let args: Vec<String> = args().collect();
    let usage = "

Usage: 
    ./todolist <operator> <idx> <optional todo item>
    valid opeartors are `add`, `delete`
Examples:
    ./todolist add 2 'go to dentist' // item placed in 0 since 2 is not present
    ./todolist delete 0

        ";

    if args.get(1).unwrap() == "--help" {
        println!("{}", usage);
        exit(0);
    }

    println!("raw args: {:?}", args);

    if args.len() < 2 {
        panic!("Insufficient arguments were given: \n{}", usage);
    }

    let command: String = args
        .get(1)
        .unwrap_or_else(|| panic!("The command argument is not present!"))
        .to_string();

    if args.len() > 2 && command == "delete" {
        panic!(
            "command is delete and extra arguments were given: \n{}",
            usage
        );
    }

    let _idx: u64 = match args
        .get(2)
        .unwrap_or_else(|| panic!("The idx argument is not present!"))
        .parse::<u64>()
    {
        Ok(v) => v,
        Err(e) => panic!("unable to parse num: {:?}", e),
    };

    match command.as_ref() {
        "add" => perform_add(args, _idx),
        "delete" => perform_delete(_idx),
        _ => panic!("invalid command {}", command),
    };

    // print("cmd: {}", command);
    // print("_idx: {}", command);
    // print("msg: {}", command);
}

fn perform_add(_args: Vec<String>, _idx: u64) {
    // args.get(3)
    //     .unwrap_or_else(|| panic!("The message argument is not present!"))
}

fn perform_delete(_idx: u64) {}
