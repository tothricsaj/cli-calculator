use std::io;

fn main() {
  let mut input = String::new();
  let first_num: i128;
  let second_num: i128;
  let operation_result: i128;

  println!("Welcom the CLI calculator!");

  loop {
    first_num = get_input("Please add first number!");
    second_num = get_input("Please add first number!");

    println!("Which operation do you want? (+, -, *, /)");
    io::stdin()
      .read_line(&mut input)
      .expect("Cannot read line");

    let operation = input.trim();

    if operation == "/" && second_num == 0 {
      println!("You cannot dived by zero!");
      return;
    }

    if operation == "+" {
      operation_result = first_num + second_num;
    } else if operation == "-" {
      operation_result = first_num - second_num;
    } else if operation == "*" {
      operation_result = first_num * second_num;
    } else if operation == "/" {
      operation_result = first_num / second_num;
    } else {
      println!("Invalid operation!");
      operation_result = -1;
    }

    println!("Result is {}", operation_result);
  }
}

fn get_input(msg: &str) -> i128 {
  let mut input_var = String::new();
  println!("{}", msg);
  io::stdin()
    .read_line(&mut input_var)
    .expect("Cannot read line");

  let number = input_var.trim().parse::<i128>().expect("Cannot read the number");

  number
}