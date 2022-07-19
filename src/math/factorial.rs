pub fn factorial(num: u64) -> u64 {
    match num {
        0 | 1 => 1,
        _ => factorial(num - 1) * num,
    }
}

#[cfg(test)]
mod factorial_tests {
    use super::*;
    #[test]
    fn good() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(20), 2432902008176640000);
    }
    #[test]
    fn notgood() {
        assert_ne!(factorial(0), 0);
        assert_ne!(factorial(2), 1);
    }
}