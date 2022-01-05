pub fn sum_pairs(numbers: &[isize], result: isize) -> Option<(isize, isize)> {
    for (i, x) in numbers.iter().enumerate() {
        for (j, y) in numbers.iter().enumerate() {
            if i != j && x + y == result {
                return Some((*x, *y));
            }
        }
    }
    return None;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_pairs() {
        assert_eq!(sum_pairs(&[3, 5, 7, 2], 10), Some((3, 7)));
        assert_eq!(sum_pairs(&[6, 7, 1, 2], 8), Some((6, 2)));
        assert_eq!(sum_pairs(&[1, 4, 8, 7, 3, 15], 8), Some((1, 7)));
    }

    #[test]
    fn test_sum_pairs_repeated_numbers_test() {
        assert_eq!(sum_pairs(&[1, 2, 3, 4, 1, 0], 2), Some((1, 1)));
        assert_eq!(sum_pairs(&[10, 5, 2, 3, 7, 5], 10), Some((5, 5)));
        assert_eq!(sum_pairs(&[0, 2, 0], 0), Some((0, 0)));
    }

    #[test]
    fn sum_pairs_negative_test() {
        assert_eq!(sum_pairs(&[5, 9, 13, -3], 10), Some((13, -3)));
        assert_eq!(sum_pairs(&[4, -2, 3, 3, 4], 8), Some((4, 4)));
        assert_eq!(sum_pairs(&[1, -2, 3, 0, -6, 1], -6), Some((0, -6)));
    }

    #[test]
    fn sum_pairs_none_test() {
        assert_eq!(sum_pairs(&[-3, -2, 7, -5], 10), None);
        assert_eq!(sum_pairs(&[20, -13, 40], -7), None);
    }
}
