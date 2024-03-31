

pub fn power(k: f64, n: usize) -> f64 {
    let mut p: f64 = 1.0; 
    for _i in 1..=n {
        p = p * k;
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::*;

    #[test]
    fn test_power_07() {
        assert_eq!((100000.0_f64*3.09223574768998_f64).round()/100000.0_f64, (100000.0*(power(1.09072, 13))).round()/100000.0_f64);
    }

    #[test]
    fn test_power_06() {
        assert_eq!(106993205379072.0, (power(12.0, 13)));
    }

    #[test]
    fn test_power_05() {
        assert_eq!(-27.0, (power(-3.0, 3)));
    }

    #[test]
    fn test_power_04() {
        assert_eq!(16.0, (power(-4.0, 2)));
    }

    #[test]
    fn test_power_03() {
        assert_eq!(2.0, (power(2.0, 1)));
    }

    #[test]
    fn test_power_02() {
        assert_eq!(1.0, (power(2.0, 0)));
    }

    #[test]
    fn test_power_01() {
        assert_eq!(8.0, (power(2.0, 3)));
    }
}