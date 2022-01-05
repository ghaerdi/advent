pub fn get_coins(mut change: u32) -> Vec<u32> {
    const COINS: [u32; 6] = [50, 20, 10, 5, 2, 1];

    let change_coins = |coin| {
        let result = {
            match change >= coin {
                true => change / coin,
                false => 0,
            }
        };

        change %= coin;
        return result;
    };

    return COINS.map(change_coins).iter().copied().rev().collect();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_coins_test() {
        assert_eq!(get_coins(0), vec![0, 0, 0, 0, 0, 0]);
        assert_eq!(get_coins(3), vec![1, 1, 0, 0, 0, 0]);
        assert_eq!(get_coins(5), vec![0, 0, 1, 0, 0, 0]);
        assert_eq!(get_coins(16), vec![1, 0, 1, 1, 0, 0]);
        assert_eq!(get_coins(51), vec![1, 0, 0, 0, 0, 1]);
        assert_eq!(get_coins(100), vec![0, 0, 0, 0, 0, 2]);
    }
}
