extern crate chrono;

use chrono::prelude::*;

pub fn days_to_xmas(date: Date<Utc>) -> i64 {
    let xmas = Utc.ymd(2021, 12, 25);
    return (xmas - date).num_days();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn days_to_xmas_test() {
        assert_eq!(days_to_xmas(Utc.ymd(2021, 12, 1)), 24);
        assert_eq!(days_to_xmas(Utc.ymd(2021, 12, 24)), 1);
    }

    #[test]
    fn is_xmas_test() {
        assert_eq!(days_to_xmas(Utc.ymd(2021, 12, 25)), 0);
    }

    #[test]
    fn days_after_xmas_test() {
        assert_eq!(days_to_xmas(Utc.ymd(2021, 12, 26)), -1);
        assert_eq!(days_to_xmas(Utc.ymd(2022, 1, 1)), -7);
    }
}
