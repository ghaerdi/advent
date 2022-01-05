pub fn count_sheeps(sheeps: Vec<Sheep>) -> Vec<Sheep> {
    let is_red = |s: &Sheep| s.color == "red";
    let constains_char_in_name = |s: &Sheep, letter: char| s.name.contains(letter);

    sheeps
        .into_iter()
        .filter(is_red)
        .filter(|sheep| constains_char_in_name(sheep, 'n') && constains_char_in_name(sheep, 'a'))
        .collect()
}

pub struct Sheep {
    name: String,
    color: String,
}

impl Sheep {
    pub fn new(name: &str, color: &str) -> Sheep {
        Sheep {
            name: name.to_string(),
            color: color.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_sheeps_test() {
        let data = vec![
            Sheep::new("carlo", "red"),
            Sheep::new("lucia", "black"),
            Sheep::new("anuel", "red"),
            Sheep::new("armando", "red"),
        ];

        let result = count_sheeps(data);

        assert_eq!(result[0].name, "anuel");
        assert_eq!(result[1].name, "armando");
        for item in result.iter() {
            assert_eq!(item.color, "red");
        }
    }
}
