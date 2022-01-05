type Carrier<'a> = (&'a str, u32, Vec<&'a str>);

pub fn count_packages(carriers: &[Carrier], carrier_id: &str) -> u32 {
    let count_subpackages = |id: &String| count_packages(carriers, id);
    let mut total = 0;

    carriers
        .iter()
        .for_each(|(id, total_packages, carriers_id)| {
            let carriers_id: Vec<String> = carriers_id.iter().map(|&s| s.to_string()).collect();
            if *id == carrier_id {
                total += total_packages;

                if carriers_id.len() > 0 {
                    total += carriers_id.iter().map(count_subpackages).sum::<u32>();
                }
            }
        });

    return total;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_packages_test_1() {
        let carriers = [
            ("dapelu", 5, vec!["midu", "jelowing"]),
            ("midu", 2, vec![]),
            ("jelowing", 2, vec![]),
        ];

        assert_eq!(count_packages(&carriers, "dapelu"), 9);
        assert_eq!(count_packages(&carriers, "midu"), 2);
        assert_eq!(count_packages(&carriers, "jelowing"), 2);
    }

    #[test]
    fn count_packages_test_2() {
        let carriers = [
            ("lolivier", 8, vec!["camila", "jesuspoleo"]),
            ("camila", 5, vec!["sergiomartinez", "conchaasensio"]),
            ("jesuspoleo", 4, vec![]),
            ("sergiomartinez", 4, vec![]),
            ("conchaasensio", 3, vec!["facundocapua", "faviola"]),
            ("facundocapua", 2, vec![]),
            ("faviola", 1, vec![]),
        ];

        assert_eq!(count_packages(&carriers, "lolivier"), 27);
        assert_eq!(count_packages(&carriers, "camila"), 15);
        assert_eq!(count_packages(&carriers, "jesuspoleo"), 4);
        assert_eq!(count_packages(&carriers, "sergiomartinez"), 4);
        assert_eq!(count_packages(&carriers, "conchaasensio"), 6);
        assert_eq!(count_packages(&carriers, "facundocapua"), 2);
        assert_eq!(count_packages(&carriers, "faviola"), 1);
    }
}
