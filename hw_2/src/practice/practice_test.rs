
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