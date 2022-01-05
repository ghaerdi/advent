use super::shared::Tree;

pub fn check_is_same_tree(tree_a: Tree, tree_b: Tree) -> bool {
    fn check(tree_a: Option<Box<Tree>>, tree_b: Option<Box<Tree>>) -> bool {
        if let (Some(a), Some(b)) = (tree_a, tree_b) {
            return a.value == b.value && check(a.left, b.left) && check(a.right, b.right);
        }

        return true;
    }
    return check(Some(Box::new(tree_a)), Some(Box::new(tree_b)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_is_same_tree_test_1() {
        let tree_a = Tree::new(
            1,
            Some(Tree::new(2, None, None)),
            Some(Tree::new(3, None, None)),
        );
        let tree_b = Tree::new(
            1,
            Some(Tree::new(3, Some(Tree::new(2, None, None)), None)),
            Some(Tree::new(5, None, Some(Tree::new(4, None, None)))),
        );

        assert!(check_is_same_tree(tree_a.clone(), tree_a.clone()));
        assert!(check_is_same_tree(tree_b.clone(), tree_b.clone()));
        assert!(!check_is_same_tree(tree_a, tree_b));
    }
}
