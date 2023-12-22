#[cfg(test)]
use crate::practice::{digit_product, fib, fizzbuzz, missing_num, uniq_digit, validate_paren};

#[test]
fn fizzbuzz_test() {
    assert_eq!(&fizzbuzz(1), "1");
    assert_eq!(&fizzbuzz(3), "Fizz");
    assert_eq!(&fizzbuzz(5), "Buzz");
    assert_eq!(&fizzbuzz(7), "7");
    assert_eq!(&fizzbuzz(9), "Fizz");
    assert_eq!(&fizzbuzz(15), "FizzBuzz");
    assert_eq!(&fizzbuzz(30), "FizzBuzz");
    assert_eq!(&fizzbuzz(49), "49");
}

#[test]
fn digit_product_test() {
    assert_eq!(digit_product(0), 0);
    assert_eq!(digit_product(9), 9);
    assert_eq!(digit_product(10), 1);
    assert_eq!(digit_product(987), 2);
    assert_eq!(digit_product(123456), 4);
    assert_eq!(digit_product(123454321), 6);
}

#[test]
fn fib_test() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 1);
    assert_eq!(fib(7), 13);
}

#[test]
fn uniq_digit_test() {
    assert_eq!(uniq_digit("3"), 3);
    assert_eq!(uniq_digit("010"), 1);
    assert_eq!(uniq_digit("47343077"), 0);
    assert_eq!(uniq_digit("123454321"), 5);
    assert_eq!(uniq_digit("0987654321234567890"), 1);
    assert_eq!(uniq_digit("4444444444424444444444444"), 2);
}

#[test]
fn missing_num_test() {
    assert_eq!(missing_num(&[1, 2]), 0);
    assert_eq!(missing_num(&[1, 0, 4, 2]), 3);
    assert_eq!(missing_num(&[0, 4, 2, 5, 3, 6]), 1);
}

#[test]
fn validate_paren_test() {
    assert_eq!(validate_paren("()"), true);
    assert_eq!(validate_paren("()[]{}"), true);
    assert_eq!(validate_paren("({[]()})"), true);
    assert_eq!(validate_paren("(}"), false);
    assert_eq!(validate_paren("()]"), false);
    assert_eq!(validate_paren("(){"), false);
}
