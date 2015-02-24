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
    use super::div_by_five;
    use super::div_by_fifteen;

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

    #[test]
    fn test_div_by_five_returns_false() {
        
        assert_eq!(false, div_by_five(3));
        assert_eq!(false, div_by_five(4));
        assert_eq!(false, div_by_five(1));
        assert_eq!(false, div_by_five(11));
        assert_eq!(false, div_by_five(19));
    }

    #[test]
    fn test_div_by_five_returns_true() {
        
        for i in 1..100 {
            assert_eq!(true, div_by_five(5 * i));
        }
    }

    #[test]
    fn test_div_by_fifteen_returns_false() {

        assert_eq!(false, div_by_fifteen(5));
        assert_eq!(false, div_by_fifteen(3));

        assert_eq!(false, div_by_fifteen(14));
        assert_eq!(false, div_by_fifteen(31));
    }

    #[test]
    fn test_div_by_fifteen_returns_true() {
        
        for i in 1..300 {
            assert_eq!(true, div_by_fifteen(15 * i));
        }
    }
}
