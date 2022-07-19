//return left+right
pub fn fibonaci(num: i64) -> i64 {
    // if num < 0 {
    //     panic!("{} : Value of num must be greater than or equal to 0!", num);
    // }
    match num {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => fibonaci(num - 1) + fibonaci(num-2),
    }
}

#[cfg(test)]
mod fibonaci_tests {
    use super::*;
    #[test]
    fn good() {
        assert_eq!(fibonaci(0), 0);
        assert_eq!(fibonaci(1), 1);
        assert_eq!(fibonaci(2), 1);
        assert_eq!(fibonaci(3), 2);
        assert_eq!(fibonaci(13), 233);
    }
    #[test]
    fn notgood(){
        assert_ne!(fibonaci(2), 2);
        assert_ne!(fibonaci(0), 1);
    }
    // #[test]
    // #[should_panic]
    // fn not_negative(){
    //     fibonaci(-1);
    // }
}