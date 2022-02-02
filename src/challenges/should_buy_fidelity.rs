pub fn should_buy_fidelity(times: u32) -> bool {
    let times = times as f32;
    250.0 - calculate_discount(times, 12.0) < times * 12.0
}

fn calculate_discount(times: f32, price: f32) -> f32 {
    let discount = price * 0.75;
    match times <= 1.0 {
        true => discount,
        false => discount + calculate_discount(times - 1.0, discount),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_buy_fidelity_test() {
        assert!(!should_buy_fidelity(1));
        assert!(!should_buy_fidelity(3));
        assert!(should_buy_fidelity(100));
    }
}
