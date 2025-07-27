fn even_or_odd(number: i32) -> String {
    if number % 2 == 0 {
        "even".to_string()
    } else {
        "odd".to_string()
    }
}

fn main() {
    println!("{}", even_or_odd(10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_or_odd() {
        assert_eq!(even_or_odd(10), "even");
    }
}
