
use std::fs;
//Here we will learn about handling errors in rust, like in python we have - try, except, finally

pub fn demo() {
    println!("========  Learning Error Handling  =============");
    //Result and Option are enums in rust
    //Result is for recoverable errors
    //Option is for non-recoverable errors
    println!("--------  Option Demo Examples ---------- ");
    option_demo();
    println!("--------  Result Demo Examples ---------- ");
    //Result<T,E> is an enum in rust
    //T is the type of value returned on success
    //E is the type of error returned on failure
    result_demo("Success");
    //result_demo(123); - Error expected string passed int
    println!("--------  Handle File Error Example --------- ");
    handle_file_error();
}

fn option_demo() {
    let some_number = Some(5);
    let some_string = Some("A String");
    let absent_number: Option<i32> = None;
    println!("Here is option demo - some_number = {:?} , some_string = {:?} , absent_number = {:?}", some_number, some_string, absent_number);
    let vec1 = vec![10,20,30,40];

    match vec1.get(10) {
        Some(x) => println!("Value at index is {}", x),
        None => println!("Given Index is out of bounds"),
    }
    println!("Error handling i.e. Option without using panics");

}

fn result_demo(input: &str) {
    let parsed: Result<i32, _> = input.parse();
    match parsed {
        Ok(x) => println!("Parsed value is {}", x),
        Err(e) => println!("Error: {}", e),
    }
    //Here program returns Error if invalid input given i.e string
    //Instead of panicking error is returned

}

fn handle_file_error() {
    let path = "input.txt";
    let result: Result<String, std::io::Error> = fs::read_to_string(path);
    match result {
        Ok(contents) => {
            println!("File read successful. Contents: \n{}", contents);

        }
        Err(e) => {
            println!("Current Working Directory = {:?}", std::env::current_dir().unwrap());
            println!("Could not read the file in path = '{}'", path);
            println!("Error: {}", e);
        }
    }

}