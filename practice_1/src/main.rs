mod practice;
mod tests;

use practice::{
    digit_product, fib, fizzbuzz, missing_num, uniq_digit, validate_paren,
};

fn main() {
    // FizzBuzz
    let res_1 = fizzbuzz(151);
    println!("{}", res_1);

    // Product of digits
    let res_2 = digit_product(123213123);
    println!("{}", res_2);

    // Fibonacci
    let res_3 = fib(7);
    println!("{}", res_3);

    // Unique number
    let res_4 = uniq_digit("12312");
    println!("{}", res_4);

    // Missing Number
    let res_5 = missing_num(&[0, 4, 2, 5, 3, 6]);
    println!("{}", res_5);

    // Validate parentheses
    let res_6 = validate_paren("{[()]}");
    println!("{}", res_6)
}
