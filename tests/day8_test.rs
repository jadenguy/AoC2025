#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use aoc2025::day8::JunctionBox;
    use aoc2025::day8::connect_junction_boxes;
    use aoc2025::day8::parse_junction_boxes;

    #[test]
    fn test_parse_manifold_strings() {
        // arrange
        let sample_data = sample_data();
        let actual_manifold = parse_junction_boxes(sample_data);
        // act
        let actual_junctions = connect_junction_boxes(actual_manifold, 10);
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
        let expected: HashSet<JunctionBox> = HashSet::from_iter([
            JunctionBox {
                id: 'a',
                x: 162,
                y: 817,
                z: 812,
            },
            JunctionBox {
                id: 'b',
                x: 57,
                y: 618,
                z: 57,
            },
            JunctionBox {
                id: 'c',
                x: 906,
                y: 360,
                z: 560,
            },
            JunctionBox {
                id: 'd',
                x: 592,
                y: 479,
                z: 940,
            },
            JunctionBox {
                id: 'e',
                x: 352,
                y: 342,
                z: 300,
            },
            JunctionBox {
                id: 'f',
                x: 466,
                y: 668,
                z: 158,
            },
            JunctionBox {
                id: 'g',
                x: 542,
                y: 29,
                z: 236,
            },
            JunctionBox {
                id: 'h',
                x: 431,
                y: 825,
                z: 988,
            },
            JunctionBox {
                id: 'i',
                x: 739,
                y: 650,
                z: 466,
            },
            JunctionBox {
                id: 'j',
                x: 52,
                y: 470,
                z: 668,
            },
            JunctionBox {
                id: 'k',
                x: 216,
                y: 146,
                z: 977,
            },
            JunctionBox {
                id: 'l',
                x: 819,
                y: 987,
                z: 18,
            },
            JunctionBox {
                id: 'm',
                x: 117,
                y: 168,
                z: 530,
            },
            JunctionBox {
                id: 'n',
                x: 805,
                y: 96,
                z: 715,
            },
            JunctionBox {
                id: 'o',
                x: 346,
                y: 949,
                z: 466,
            },
            JunctionBox {
                id: 'p',
                x: 970,
                y: 615,
                z: 88,
            },
            JunctionBox {
                id: 'q',
                x: 941,
                y: 993,
                z: 340,
            },
            JunctionBox {
                id: 'r',
                x: 862,
                y: 61,
                z: 35,
            },
            JunctionBox {
                id: 's',
                x: 984,
                y: 92,
                z: 344,
            },
            JunctionBox {
                id: 't',
                x: 425,
                y: 690,
                z: 689,
            },
        ]);
        // act
        let actual_manifold: HashSet<JunctionBox> =
            HashSet::from_iter(parse_junction_boxes(sample_data));
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
