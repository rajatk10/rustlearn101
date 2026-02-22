use basicCalculator::calculator;

#[test]
fn test_addition() {
    let result = calculator::calculate(1, calculator::Operation::Add, 4);
    assert_eq!(result, Ok(5));
}

#[test]
fn test_division() {
    let result = calculator::calculate(5, calculator::Operation::Divide,2);
    assert_eq!(result, Ok(2));
}

#[test]
fn test_modulus() {
    let result = calculator::calculate(5, calculator::Operation::Modulus,2);
    assert_eq!(result, Ok(1));
}

#[test]
fn test_subtraction() {
    let result = calculator::calculate(2, calculator::Operation::Subtract,4);
    assert_eq!(result, Ok(2));
}

#[test]
fn test_multiplication() {
    let result = calculator::calculate(2, calculator::Operation::Multiply, 5);
    assert_ne!(result, Ok(9)); //Passes if result is not equal to 9 - as result should be 10
    //Covers not equal example for learning
}

#[test]
fn test_zero_division() {
    let result = calculator::calculate(5, calculator::Operation::Divide,0);
    assert!(result.is_err()); //Passes if result is error
}

#[test]
fn test_zero_modulus() {
    let result = calculator::calculate(5, calculator::Operation::Modulus, 0);
    assert!(result.is_err()); //Passes if result is error
}

#[test]
#[should_panic]
fn test_panic() {
    panic!("This test should panic");
}
#[test]
#[ignore]
fn test_ignore() {
    assert_eq!(1, 2);
}