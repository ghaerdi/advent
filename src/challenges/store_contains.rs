use std::collections::HashMap;

#[derive(Debug)]
pub enum Store {
    Map(HashMap<String, Box<Store>>),
    Item(String),
}

pub fn store_contains(store: Store, item: &str) -> bool {
    match store {
        Store::Map(map) => {
            for (_, store) in map {
                if store_contains(*store, item) {
                    return true;
                }
            }
            false
        }
        Store::Item(i) => i == item,
    }
}

#[cfg(test)]
mod test {
    use super::Store::{Item, Map};
    use super::*;

    fn store_example() -> Store {
        let mut store = HashMap::new();
        let mut shelve = HashMap::new();
        let mut drawer = HashMap::new();

        drawer.insert(
            "product1".to_owned(),
            Box::new(Item("coca-cola".to_owned())),
        );
        drawer.insert("product2".to_owned(), Box::new(Item("fanta".to_owned())));
        drawer.insert("product3".to_owned(), Box::new(Item("sprite".to_owned())));

        shelve.insert("drawer1".to_owned(), Box::new(Map(drawer)));

        store.insert("shelve1".to_owned(), Box::new(Map(shelve)));

        let mut shelve = HashMap::new();
        let mut drawer = HashMap::new();

        drawer.insert("product1".to_owned(), Box::new(Item("pants".to_owned())));
        drawer.insert("product2".to_owned(), Box::new(Item("tshirt".to_owned())));

        shelve.insert("drawer1".to_owned(), Box::new(Item("nothing".to_owned())));
        shelve.insert("drawer2".to_owned(), Box::new(Map(drawer)));

        store.insert("shelve2".to_owned(), Box::new(Map(shelve)));

        Map(store)
    }

    #[test]
    fn store_contains_test() {
        let store = store_example();
        assert!(store_contains(store, "tshirt"));
    }

    fn store_not_contains_test() {
        let store = store_example();
        assert!(!store_contains(store, "gameboy"));
    }
}
