#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use aoc2025::day8::Box;
    use aoc2025::day8::connect_junction_boxes;
    use aoc2025::day8::parse_junction_boxes;

    #[test]
    fn test_parse_manifold_strings() {
        // arrange
        let sample_data = sample_data();
        let actual_manifold = parse_junction_boxes(sample_data);
        // act
        let mut actual_junctions = connect_junction_boxes(actual_manifold, 10);
        let mut sizes: Vec<usize> = actual_junctions.iter().map(|x| x.len()).collect();
        sizes.sort();
        let product_of_three_longest: usize = sizes.iter().rev().take(3).product();
        // assert
        assert_eq!(product_of_three_longest, 40)
    }

    #[test]
    fn test_parse_junction_boxes() {
        // arrange
        let sample_data = sample_data();
        let expected: HashSet<Box> = HashSet::from_iter([
            Box {
                x: 162,
                y: 817,
                z: 812,
            },
            Box {
                x: 57,
                y: 618,
                z: 57,
            },
            Box {
                x: 906,
                y: 360,
                z: 560,
            },
            Box {
                x: 592,
                y: 479,
                z: 940,
            },
            Box {
                x: 352,
                y: 342,
                z: 300,
            },
            Box {
                x: 466,
                y: 668,
                z: 158,
            },
            Box {
                x: 542,
                y: 29,
                z: 236,
            },
            Box {
                x: 431,
                y: 825,
                z: 988,
            },
            Box {
                x: 739,
                y: 650,
                z: 466,
            },
            Box {
                x: 52,
                y: 470,
                z: 668,
            },
            Box {
                x: 216,
                y: 146,
                z: 977,
            },
            Box {
                x: 819,
                y: 987,
                z: 18,
            },
            Box {
                x: 117,
                y: 168,
                z: 530,
            },
            Box {
                x: 805,
                y: 96,
                z: 715,
            },
            Box {
                x: 346,
                y: 949,
                z: 466,
            },
            Box {
                x: 970,
                y: 615,
                z: 88,
            },
            Box {
                x: 941,
                y: 993,
                z: 340,
            },
            Box {
                x: 862,
                y: 61,
                z: 35,
            },
            Box {
                x: 984,
                y: 92,
                z: 344,
            },
            Box {
                x: 425,
                y: 690,
                z: 689,
            },
        ]);
        // act
        let actual_manifold: HashSet<Box> = HashSet::from_iter(parse_junction_boxes(sample_data));
        // assert
        assert_eq!(actual_manifold, expected);
    }
    fn sample_data() -> Vec<&'static str> {
        r#"162,817,812
            57,618,57
            906,360,560
            592,479,940
            352,342,300
            466,668,158
            542,29,236
            431,825,988
            739,650,466
            52,470,668
            216,146,977
            819,987,18
            117,168,530
            805,96,715
            346,949,466
            970,615,88
            941,993,340
            862,61,35
            984,92,344
            425,690,689"#
            .split("\n")
            .map(|x| x.trim())
            .collect()
    }
}
