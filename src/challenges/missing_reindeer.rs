pub fn missing_reindeer(mut ids: Vec<u8>) -> u8 {
    ids.sort_unstable();
    match ids[0] {
        1 => 0,
        _ => {
            ids.iter()
                .enumerate()
                .find(|&(i, &n)| n + 1 != ids[i + 1])
                .unwrap()
                .1
                + 1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn missing_first_reindeer_test() {
        assert_eq!(missing_reindeer(vec![1, 2, 3]), 0);
    }

    #[test]
    fn missing_second_reindeer_test() {
        assert_eq!(missing_reindeer(vec![3, 5, 4, 0, 2]), 1);
    }

    #[test]
    fn missing_reindeer_test() {
        assert_eq!(missing_reindeer(vec![5, 6, 1, 2, 3, 7, 0]), 4);
    }
}
