pub fn single_digit_adder(x: i8, y: i8) -> i8 {
    fn is_single_digit(x: i8) -> bool {
        x < 10 && x > -10
    }
    if !(is_single_digit(x)) || !(is_single_digit(y)) {
        panic!("Only single digits allowed");
    }{
        x + y
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_adds() {
        let sum = single_digit_adder(2, 3);
        assert_eq!(sum, 5);
    }

    // our new test function that expects `panic!` with invalid param
    #[test]
    #[should_panic]
    fn it_should_only_accept_single_digits() {
        single_digit_adder(11, 4);
    }
}
