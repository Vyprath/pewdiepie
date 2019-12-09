use std::io;

fn digits(num: &str) -> Vec<u128> {
    num.chars().filter_map(|c| c.to_digit(10)).map(|d| d.into()).collect()
}

fn as_number(digits: &[u128]) -> u128 {
    digits.iter().fold(0, |acc, &d| acc * 10 + d)
}

fn sum_of_digits_to_power(digits: &[u128]) -> u128 {
    let len = digits.len() as u32;
    digits.iter().map(|d| d.pow(len)).sum()
}

fn are_digits_of_armstrong_number(digits: &[u128]) -> bool {
    as_number(digits) == sum_of_digits_to_power(digits)
}

fn read_digits() -> Vec<u128> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Couldn't read from stdin");
    digits(&input.trim())
}

fn main() {
    let digits = read_digits();
    println!("is_armstrong({:?}) == {:?}", digits, are_digits_of_armstrong_number(&digits));
}

#[test]
fn test_known_armstrong_numbers() {
    assert!(are_digits_of_armstrong_number(&[2,4,6,7,8,0,5,1]));
}
