use std::collections::HashMap;

pub fn group_by<T>(items: Vec<T>, key: fn(&T) -> String) -> HashMap<String, Vec<T>> {
    let mut map = HashMap::new();
    for item in items {
        let key = key(&item);
        map.entry(key).or_insert(vec![]).push(item);
    }
    return map;
}

#[cfg(test)]
mod test {
    use super::*;

    struct Book {
        title: String,
        rating: u8,
    }

    impl Book {
        fn new(title: String, rating: u8) -> Self {
            Self { title, rating }
        }
    }

    #[test]
    fn group_floats_by_numbers_test() {
        let floats = vec![6.1, 4.2, 6.3, 4.5, 6.2];
        let floor = |x: &f64| x.floor().to_string();
        let grouped = group_by(floats, floor);

        assert_eq!(grouped.get(&"6".to_string()).unwrap().len(), 3);
        assert_eq!(grouped.get(&"4".to_string()).unwrap().len(), 2);
    }

    #[test]
    fn group_str_by_len_test() {
        let strings = vec!["zxcv", "sa", "pepe", "gil"];
        let len = |x: &&str| x.len().to_string();
        let grouped = group_by(strings, len);

        assert_eq!(grouped.get(&"2".to_string()).unwrap().len(), 1);
        assert_eq!(grouped.get(&"3".to_string()).unwrap().len(), 1);
        assert_eq!(grouped.get(&"4".to_string()).unwrap().len(), 2);
    }

    #[test]
    fn group_books_by_rating_test() {
        let books = vec![
            Book::new("Functional Programming with Javascript".to_string(), 9),
            Book::new("Clean Architecture".to_string(), 8),
            Book::new("Refactorig to Rust".to_string(), 9),
            Book::new("Clean Coder".to_string(), 7),
        ];

        let rating = |x: &Book| x.rating.to_string();
        let grouped = group_by(books, rating);
        assert_eq!(grouped.get(&"9".to_string()).unwrap().len(), 2);
        assert_eq!(grouped.get(&"8".to_string()).unwrap().len(), 1);
        assert_eq!(grouped.get(&"7".to_string()).unwrap().len(), 1);
    }
}
