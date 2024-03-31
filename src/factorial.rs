pub fn factorial(n: usize) -> usize {
    let mut f: usize = 1;
    for i in 2..=n {
        f = f * i;
    }
    f
}

pub fn double_factorial(n: usize) -> usize {
    if n % 2 == 0 {
        match n {
            0 => 1,
            _ => {
                let mut f: usize = 2;
                for i in 2..=n / 2 {
                    f = f * i * 2;
                }
                f
            },
        }
    } else {
        let mut f: usize = 1;
            for i in 1..=(1 + n / 2) {
                f = f * (i * 2 - 1);
            }
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_factorial_11() {
        assert_eq!(645120, (double_factorial(14)));
    }

    #[test]
    fn test_double_factorial_10() {
        assert_eq!(135135, (double_factorial(13)));
    }

    #[test]
    fn test_double_factorial_09() {
        assert_eq!(1, (double_factorial(1)));
    }

    #[test]
    fn test_double_factorial_08() {
        assert_eq!(3, (double_factorial(3)));
    }

    #[test]
    fn test_double_factorial_07() {
        assert_eq!(15, (double_factorial(5)));
    }

    #[test]
    fn test_double_factorial_06() {
        assert_eq!(105, (double_factorial(7)));
    }

    #[test]
    fn test_double_factorial_05() {
        assert_eq!(1, (double_factorial(0)));
    }

    #[test]
    fn test_double_factorial_04() {
        assert_eq!(2, (double_factorial(2)));
    }

    #[test]
    fn test_double_factorial_03() {
        assert_eq!(8, (double_factorial(4)));
    }

    #[test]
    fn test_double_factorial_02() {
        assert_eq!(384, (double_factorial(8)));
    }

    #[test]
    fn test_double_factorial_01() {
        assert_eq!(48, (double_factorial(6)));
    }

    #[test]
    fn test_factorial_06() {
        assert_eq!(1, (factorial(0)));
    }

    #[test]
    fn test_factorial_05() {
        assert_eq!(1, (factorial(1)));
    }

    #[test]
    fn test_factorial_04() {
        assert_eq!(2, (factorial(2)));
    }

    #[test]
    fn test_factorial_03() {
        assert_eq!(720, (factorial(6)));
    }

    #[test]
    fn test_factorial_02() {
        assert_eq!(24, (factorial(4)));
    }

    #[test]
    fn test_factorial_01() {
        assert_eq!(6, (factorial(3)));
    }
}
