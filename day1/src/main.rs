use std::ops::Deref;
use std::str::FromStr;

fn main() {
    println!("AOC day 1.");
    let list = read_list();
    println!("computing sum...");
    let result = compute_sum(list);
    println!("result is: {}", result);
    assert_eq!(result, 142);
    println!("correct!")
}

fn read_list<'a>() -> Vec<&'a str> {
    vec!["1abc2",
         "pqr3stu8vwx",
         "a1b2c3d4e5f",
         "treb7uchet"]
}

fn compute_sum(values: Vec<&str>) -> u32 {
    return values.iter()
        .map(|str| filter_2digit(str))
        .map(|str| u32::from_str(str.as_str()).unwrap())
        .sum();
}

fn filter_2digit(str: &str) -> String {
    let decomposed_numbers: Vec<&str> = str
        .matches(char::is_numeric)
        .collect();
    decomposed_numbers.first().unwrap().deref().to_owned() + decomposed_numbers.last().unwrap().deref()
}

#[test]
fn test_compute_sum() {
    let list = vec!["1abc2", "pqr3stu8vwx"];
    let result = compute_sum(list);
    assert_eq!(result, 50);
}

#[test]
fn given_string_with_2digits_when_filter_then_return_2digits() {
    let str = "pqr3stu8vwx";
    let result = filter_2digit(str);
    assert_eq!(result, "38");
}

#[test]
fn given_string_with_3digits_when_filter_then_return_2digits() {
    let str = "pqr3s6tu8vwx";
    let result = filter_2digit(str);
    assert_eq!(result, "38");
}

#[test]
fn given_string_with_1digit_when_filter_then_return_2digits() {
    let str = "pqrs8vwx";
    let result = filter_2digit(str);
    assert_eq!(result, "88");
}
