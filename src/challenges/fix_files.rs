use std::collections::HashMap;

pub fn fix_files(files: Vec<&str>) -> Vec<String> {
    let mut counter = HashMap::new();

    return files
        .iter()
        .map(|&f| {
            if let Some(n) = counter.get_mut(f) {
                *n += 1;
                return format!("{}({})", f, n);
            }

            counter.insert(f, 0);
            return f.to_string();
        })
        .collect();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fix_files_test() {
        assert_eq!(
            fix_files(vec!["photo", "postcard", "photo", "photo", "video"]),
            vec!["photo", "postcard", "photo(1)", "photo(2)", "video"]
        );
        assert_eq!(
            fix_files(vec!["file", "file", "file", "game", "game"]),
            vec!["file", "file(1)", "file(2)", "game", "game(1)"]
        );
        assert_eq!(
            fix_files(vec!["file", "file(1)", "icon", "icon(1)", "icon(1)"]),
            vec!["file", "file(1)", "icon", "icon(1)", "icon(1)(1)"]
        );
    }
}
