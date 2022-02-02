use std::collections::BTreeMap;

#[derive(Debug)]
struct Trip {
    get: Option<u32>,
    delivery: Option<u32>,
}

impl Trip {
    fn new() -> Trip {
        Trip {
            get: None,
            delivery: None,
        }
    }
}

pub fn can_carry(capacity: u32, trip: &[(u32, u32, u32)]) -> bool {
    let mut trip_map = BTreeMap::new();

    trip.iter().for_each(|item| {
        trip_map.entry(item.1).or_insert_with(Trip::new);
        trip_map.entry(item.2).or_insert_with(Trip::new);

        trip_map.get_mut(&item.1).unwrap().get = Some(item.0);
        trip_map.get_mut(&item.2).unwrap().delivery = Some(item.0);
    });

    let mut current: i32 = 0;
    for trip in trip_map.into_values() {
        if current > capacity as i32 {
            return false;
        }

        if let Some(value) = trip.get {
            current += value as i32;
        }
        if let Some(value) = trip.delivery {
            current -= value as i32;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_carry_test() {
        assert!(can_carry(3, &[(1, 1, 5), (2, 2, 10)]));
        assert!(can_carry(3, &[(2, 1, 5), (3, 5, 7)]));
        assert!(can_carry(4, &[(2, 3, 8), (2, 5, 7)]));
    }

    #[test]
    fn cannot_carry_test() {
        assert!(!can_carry(4, &[(2, 5, 8), (3, 6, 10)]));
        assert!(!can_carry(1, &[(2, 3, 8)]));
        assert!(!can_carry(2, &[(1, 2, 4), (2, 3, 8)]));
    }
}
