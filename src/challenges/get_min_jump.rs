pub fn get_min_jump(obstacles: &[u8]) -> u8 {
    let free = get_rest_of_numbers(obstacles, *obstacles.iter().max().unwrap());
    for f in free {
        if obstacles.iter().all(|&o| o % f != 0) {
            return f;
        }
    }

    1
}

fn get_rest_of_numbers(numbers: &[u8], limit: u8) -> Vec<u8> {
    (1..limit).filter(|&x| !numbers.contains(&x)).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_min_jump_test() {
        assert_eq!(get_min_jump(&[5, 3, 6, 7, 9]), 4);
        assert_eq!(get_min_jump(&[2, 4, 6, 8, 10]), 7);
        assert_eq!(get_min_jump(&[1, 2, 3, 5]), 4);
        assert_eq!(get_min_jump(&[3, 7, 5]), 2);
        assert_eq!(get_min_jump(&[9, 5, 1]), 2);
    }
}
