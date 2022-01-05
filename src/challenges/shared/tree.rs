#[derive(Clone)]
pub struct Tree {
    pub value: u32,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
}

impl Tree {
    pub fn new(value: u32, left: Option<Tree>, right: Option<Tree>) -> Self {
        let mut boxed_l = None;
        let mut boxed_r = None;

        if let Some(t) = left {
            boxed_l = Some(Box::new(t));
        }

        if let Some(t) = right {
            boxed_r = Some(Box::new(t));
        }

        Tree {
            value,
            left: boxed_l,
            right: boxed_r,
        }
    }
}
