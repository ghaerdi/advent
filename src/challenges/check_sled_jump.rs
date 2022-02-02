pub fn check_sled_jump(heights: &[u32]) -> bool {
    let mut up = true;
    let mut invalid = false;

    heights.iter().enumerate().for_each(|(i, &value)| {
        if i + 1 != heights.len() {
            up &= value < heights[i + 1];
            invalid |= !up && value <= heights[i + 1];
        }
    });

    !invalid && !up
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_sled_jump_test() {
        assert!(check_sled_jump(&[1, 2, 3, 2, 1]));
        assert!(check_sled_jump(&[0, 1, 0]));
        assert!(check_sled_jump(&[0, 3, 2, 1]));
    }

    #[test]
    fn check_sled_jump_not_valid_test() {
        assert!(!check_sled_jump(&[2, 4, 4, 6, 2]));
        assert!(!check_sled_jump(&[1, 2, 3]));
        assert!(!check_sled_jump(&[1, 2, 3, 2, 1, 2, 3]));
    }
}
