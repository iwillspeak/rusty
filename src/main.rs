fn main() {

    for i in 1..100 {
        println!(
            "{}",
            if div_by_fifteen(i) { "FizzBuzz".to_string() }
            else if div_by_three(i) { "Fizz".to_string() }
            else if div_by_five(i) { "Buzz".to_string() }
            else { i.to_string() });
    }

}

fn div_by_three(num: u64) -> bool {
    (num % 3) == 0
}

fn div_by_five(num: u64) -> bool {
    (num % 5) == 0
}

fn div_by_fifteen(num: u64) -> bool {
    div_by_five(num) && div_by_three(num)
}

#[cfg(test)]
mod test {

    use super::div_by_three;

    #[test]
    fn test_div_by_three_returns_false() {

        assert_eq!(false, div_by_three(1));
        assert_eq!(false, div_by_three(2));

        assert_eq!(false, div_by_three(4));
        assert_eq!(false, div_by_three(5));
    }

    #[test]
    fn test_div_by_three_returns_true() {
        
        for i in 1..300 {
            assert_eq!(true, div_by_three(3 * i));
        }
    }
}
