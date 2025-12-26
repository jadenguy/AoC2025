type Range = (usize, usize);
pub fn parse_db(lines: Vec<String>) -> Database {
    let mut ranges: Vec<Range> = vec![];
    let mut available: Vec<usize> = vec![];
    let mut adding_ranges = true;
    for line in lines {
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

pub fn total_fresh_ids(db: &Database) -> usize {
    let mut count = 0;
    let normal_ranges = normalize_ranges(&db);

    for range in normal_ranges {
        println!("{} to {}", range.0, range.1);
        // for n in range.0..=range.1 {
        //     count += 1;
        // }
        count += range.1 - range.0 + 1;
    }
    count
}

fn normalize_ranges(db: &Database) -> Vec<Range> {
    let mut x: Vec<Range> = db
        .fresh_ingredient_id_ranges
        .iter()
        .map(|x| x.to_owned())
        .collect();
    x.sort();
    let mut i = 0;
    while i < x.len() - 1 {
        let a = x[i];
        let b = x[i + 1];
        if a.1 >= b.1 {
            println!("first {}-{} overlaps second {}-{}", a.0, a.1, b.0, b.1);
            x.remove(i + 1);
        } else if a.1 >= b.0 {
            println!(
                "first {}-{} combines with second {}-{}, now {}-{}",
                a.0, a.1, b.0, b.1, a.0, b.1
            );

            x[i] = (a.0, b.1);
            x.remove(i + 1);
        } else {
            println!(
                "range {}-{} not in contact with range {}-{}",
                a.0, a.1, b.0, b.1
            );
            i += 1;
        }
    }
    x
}
pub fn count_fresh_ingredients(db: &Database) -> usize {
    let mut count = 0;
    for availabe in &db.available_ingredient_ids {
        if db
            .fresh_ingredient_id_ranges
            .iter()
            .any(|r| availabe >= &r.0 && availabe <= &r.1)
        {
            count += 1;
        }
    }
    count
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Database {
    pub fresh_ingredient_id_ranges: Vec<Range>,
    pub available_ingredient_ids: Vec<usize>,
}
