pub fn parse_db(lines: Vec<&str>) -> Database {
    let mut ranges: Vec<(u8, u8)> = vec![];
    let mut available: Vec<u8> = vec![];
    let mut adding_ranges = true;
    for line in lines {
        println!("{}", line);
        if line == "" {
            adding_ranges = false;
        } else if adding_ranges {
            ranges.push(
                line.split_once("-")
                    .map(|r| (r.0.parse().unwrap(), r.1.parse().unwrap()))
                    .unwrap(),
            );
        } else {
            available.push(line.parse().unwrap());
        }
    }
    Database {
        fresh_ingredient_id_ranges: ranges,
        available_ingredient_ids: available,
    }
}

pub fn count_fresh_ingredients(db: Database) -> usize {
    let mut count = 0;
    for availabe in db.available_ingredient_ids {
        if db
            .fresh_ingredient_id_ranges
            .iter()
            .any(|r| availabe >= r.0 && availabe <= r.1)
        {
            count += 1;
        }
    }
    count
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Database {
    pub fresh_ingredient_id_ranges: Vec<(u8, u8)>,
    pub available_ingredient_ids: Vec<u8>,
}
