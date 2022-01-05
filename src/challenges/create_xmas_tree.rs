pub fn create_xmas_tree(height: usize) -> String {
    let width = {
        match height {
            1 => 1,
            _ => height * 2 - 1,
        }
    };

    let mut three = vec![];

    let mut count = 1;
    while count <= width {
        let mut leaf = (0..count).map(|_| '*').collect::<String>();
        leaf = fill_with_underscore(leaf, width);
        three.push(leaf);
        count += 2;
    }

    let trunk = fill_with_underscore("#".to_owned(), width);
    three.push(trunk.clone());
    three.push(trunk);

    return three.join("\n");
}

fn fill_with_underscore(text: String, len: usize) -> String {
    let mut underscores = String::new();
    for _ in 0..(len - text.len()) / 2 {
        underscores.push('_');
    }

    return underscores.clone() + &text + &underscores;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_xmas_tree_test_1() {
        let result = ["*", "#", "#"].join("\n");
        assert_eq!(create_xmas_tree(1), result);
    }

    #[test]
    fn create_xmas_tree_test_2() {
        let result = ["_*_", "***", "_#_", "_#_"].join("\n");
        assert_eq!(create_xmas_tree(2), result);
    }

    #[test]
    fn create_xmas_tree_test_3() {
        let result = ["__*__", "_***_", "*****", "__#__", "__#__"].join("\n");
        assert_eq!(create_xmas_tree(3), result);
    }

    #[test]
    fn create_xmas_tree_test_4() {
        let result = [
            "___*___", "__***__", "_*****_", "*******", "___#___", "___#___",
        ]
        .join("\n");
        assert_eq!(create_xmas_tree(4), result);
    }

    #[test]
    fn create_xmas_tree_test_5() {
        let result = [
            "____*____",
            "___***___",
            "__*****__",
            "_*******_",
            "*********",
            "____#____",
            "____#____",
        ]
        .join("\n");
        assert_eq!(create_xmas_tree(5), result);
    }
}
