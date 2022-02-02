use super::shared::Tree;

pub fn count_decorations(tree: Tree) -> u32 {
    fn counter(t: Option<Box<Tree>>) -> u32 {
        if let Some(t) = t {
            return t.value + counter(t.left) + counter(t.right);
        }
        0
    }

    counter(Some(Box::new(tree)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_decorations_test_1() {
        let tree = Tree::new(
            1,
            Some(Tree::new(2, None, None)),
            Some(Tree::new(3, None, None)),
        );

        assert_eq!(count_decorations(tree), 6);
    }

    #[test]
    fn count_decorations_test_2() {
        let tree = Tree::new(
            1,
            Some(Tree::new(
                5,
                Some(Tree::new(7, Some(Tree::new(3, None, None)), None)),
                None,
            )),
            Some(Tree::new(
                6,
                Some(Tree::new(5, None, None)),
                Some(Tree::new(1, None, None)),
            )),
        );

        assert_eq!(count_decorations(tree), 28);
    }
}
