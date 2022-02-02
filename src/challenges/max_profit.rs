pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = -1;
    let mut min_price = prices[0];
    for price in prices {
        if price < min_price {
            min_price = price;
        }
        let profit = price - min_price;
        if profit > max_profit && profit > 0 {
            max_profit = price - min_price;
        }
    }
    max_profit
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn max_profit_test() {
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 9);
    }

    #[test]
    fn no_profit_test() {
        assert_eq!(max_profit(vec![5, 4, 3, 2, 1]), -1);
    }
}
