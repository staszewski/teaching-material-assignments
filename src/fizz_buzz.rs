pub fn fizz_buzz(i: i32) -> String {
    if i % 3 == 0 && i % 5 == 0 {
        String::from("FizzBuzz")
    } else if i % 3 == 0 {
        String::from("Fizz")
    } else if i % 5 == 0 {
        String::from("Buzz")
    } else {
        format!("{}", i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizz_buzz(15), String::from("FizzBuzz"));
    }

    #[test]
    fn test_fizz() {
        assert_eq!(fizz_buzz(3), String::from("Fizz"));
    }

    #[test]
    fn test_buzz() {
        assert_eq!(fizz_buzz(5), String::from("Buzz"));
    }

    #[test]
    fn test_non_fizz_buzz_value() {
        assert_eq!(fizz_buzz(2), String::from("2"));
    }
}
