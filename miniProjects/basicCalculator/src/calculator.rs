
use std::io;

pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
}
pub fn execute() {
    println!("Please enter the first number : ");
    let num1 = read_number();
    println!("Please enter the operation i.e. (+,-,*,/,%) out of these");
    let op = read_operation();
    println!("Please enter the second number : ");
    let num2 = read_number();
    // let result = calculate(num1, op, num2);
    // println!("Result : {}", result); -
    // Above error is not handled gracefully
    match calculate(num1, op, num2) {
        Ok(result) => println!("Result : {}", result),
        Err(e) => println!("Error : {}", e),
    }

}

fn read_number() -> i32 {
    loop {
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();

        match num.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input, Please enter a valid integer"),
        }
    }

}
fn read_operation() -> Operation {
    loop {
        let mut op_input = String::new();
        io::stdin().read_line(&mut op_input).unwrap();
        //unwrap causes panic - need to handle error
        match op_input.trim() {
            "+" => return Operation::Add,
            "-" => return Operation::Subtract,
            "*" => return Operation::Multiply,
            "/" => return Operation::Divide,
            "%" => return Operation::Modulus,
            _ => println!("Invalid operator. Please retry with valid operator i.e. (+,-,*,/,%)"),
        }
    }
}
pub fn calculate(num1: i32, op: Operation, num2: i32) -> Result<i32, String> {
    match op {
        Operation::Add => Ok(num1 + num2),
        Operation::Subtract => {
            if num2 > num1 {
                Ok(num2 - num1)
            } else {
                Ok(num1 - num2)
            }
        },
        Operation::Multiply => Ok(num1 * num2),
        Operation::Divide => {
            if num2 == 0 {
                //Need to return right away - cannot infer type otherwise
                return Err("ZeroDivisionError".to_string());
            }
            Ok(num1 / num2)
        },
        Operation::Modulus => {
            if num2 == 0 {
                //If not return then cannot infer type therefore use if else
                Err("ZeroDivision-ModulusError".to_string())
            }else {
                Ok(num1 % num2)
            }
        },
    }
}