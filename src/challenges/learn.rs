pub fn learn(time: u8, courses: &[u8]) -> Option<(usize, usize)> {
    let mut result = (0, 0);

    for (i, &n1) in courses.iter().enumerate() {
        if n1 >= time {
            continue;
        }

        for (j, &n2) in courses.iter().enumerate() {
            if n2 >= time || i == j || n1 + n2 > time {
                continue;
            }

            if courses[i] + courses[j] > courses[result.0] + courses[result.1] {
                result = (i, j);
            }
        }
    }

    if result.0 == 0 && result.1 == 0 {
        return None;
    }

    return Some(result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn learn_test() {
        assert_eq!(learn(10, &[2, 3, 8, 1, 4]), Some((0, 2)));
        assert_eq!(learn(15, &[2, 10, 4, 1]), Some((1, 2)));
        assert_eq!(learn(25, &[10, 15, 20, 5]), Some((0, 1)));

        assert_eq!(learn(4, &[10, 14, 20]), None);
        assert_eq!(learn(5, &[5, 5, 5]), None);
    }
}
