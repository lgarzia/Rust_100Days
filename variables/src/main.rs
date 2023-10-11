fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("The value of y is: {y}");
    let a = [1, 2, 3, 4, 5];
    another_function(5); 
    let first = a[0];
    let second = a[1];
    print_labeled_measurement(5, 'h');
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

fn another_function(x: i32) {
    println!("Another function.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}