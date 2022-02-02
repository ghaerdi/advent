pub fn wrap_gifts(gifts: &[&str]) -> Vec<String> {
    match gifts.len() {
        0 => vec![],
        _ => {
            let len = gifts[0].len() / 2 + 2;
            let border_x = "*".repeat(len);

            let mut result = vec![border_x.clone()];
            gifts.iter().for_each(|&g| {
                result.push(format!("*{}*", g));
            });
            result.push(border_x);

            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wrap_camera_test() {
        assert_eq!(wrap_gifts(&["📷"]), vec!["****", "*📷*", "****"]);
    }

    #[test]
    fn wrap_balon_and_camera_test() {
        assert_eq!(
            wrap_gifts(&["🎈", "📷"]),
            vec!["****", "*🎈*", "*📷*", "****"]
        );
    }

    #[test]
    fn wrap_balon_guitar_and_controller_rabbit_test() {
        assert_eq!(
            wrap_gifts(&["🎈🎸", "🎮🐰"]),
            vec!["******", "*🎈🎸*", "*🎮🐰*", "******"]
        );
    }

    #[test]
    fn wrap_gifs_nothing_test() {
        assert_eq!(wrap_gifts(&[]), vec![] as Vec<String>);
    }
}
