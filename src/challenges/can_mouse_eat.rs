const MOUSE: &str = "m";
const FOOD: &str = "*";

pub fn can_mouse_eat(direction: &str, room: &[Vec<&str>]) -> bool {
    let mut mouse_position = None;
    room.iter().enumerate().for_each(|(y, arr)| {
        arr.iter().enumerate().for_each(|(x, &el)| {
            if el == MOUSE {
                mouse_position = Some((y, x));
            }
        })
    });

    if let Some(m) = mouse_position {
        match direction {
            "up" => {
                if m.0 > 0 {
                    return room[m.0 - 1][m.1] == FOOD;
                }
                return false;
            }
            "down" => {
                if m.0 + 1 != room.len() {
                    return room[m.0 + 1][m.1] == FOOD;
                }
                return false;
            }
            "left" => {
                if m.1 - 1 > 0 {
                    return room[m.0][m.1 - 1] == FOOD;
                }
                return false;
            }
            "right" => {
                if m.1 + 1 != room[0].len() {
                    return room[m.0][m.1 + 1] == FOOD;
                }
                return false;
            }
            _ => false,
        };
    };

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_mouse_eat_test_1() {
        let room = [
            vec![" ", " ", " "],
            vec![" ", " ", MOUSE],
            vec![" ", " ", FOOD],
        ];

        assert!(!can_mouse_eat("up", &room));
        assert!(can_mouse_eat("down", &room));
        assert!(!can_mouse_eat("right", &room));
        assert!(!can_mouse_eat("left", &room));
    }

    #[test]
    fn can_mouse_eat_test_2() {
        let room = [
            vec![FOOD, " ", " ", " "],
            vec![" ", MOUSE, FOOD, " "],
            vec![" ", " ", " ", " "],
            vec![" ", " ", " ", FOOD],
        ];

        assert!(!can_mouse_eat("up", &room));
        assert!(!can_mouse_eat("down", &room));
        assert!(can_mouse_eat("right", &room));
        assert!(!can_mouse_eat("left", &room));
    }
}
