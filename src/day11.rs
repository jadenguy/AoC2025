use std::collections::HashMap;
type Node = String;
#[derive(Debug, PartialEq)]
pub struct Reactor {
    pub network: HashMap<Node, Vec<Node>>,
}

impl Reactor {
    pub fn from_str(lines: Vec<&str>) -> Reactor {
        let mut network = HashMap::new();
        for line in lines {
            let (key, value) = line.split_once(": ").unwrap();
            network.insert(
                Node::from(key),
                value.split(" ").map(|x| Node::from(x)).collect(),
            );
        }
        Reactor { network }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // stolen from somewhere?
    macro_rules! map {
        ($($k:expr => $v:expr),* $(,)?) => {
            HashMap::from([
                $(($k.to_string(), $v.iter().map(|s| s.to_string()).collect()),)*
            ])
        };
    }
    #[test]
    fn test_parse_reactor() {
        let sample_data = sample_data();
        let reactor: Reactor = Reactor::from_str(sample_data);
        println!("{:?}", reactor.network);

        let n = map!(
            "aaa"=> ["you", "hhh"],
            "you"=> ["bbb", "ccc"],
            "bbb"=> ["ddd", "eee"],
            "ccc"=> ["ddd", "eee", "fff"],
            "ddd"=> ["ggg"],
            "eee"=> ["out"],
            "fff"=> ["out"],
            "ggg"=> ["out"],
            "hhh"=> ["ccc", "fff", "iii"],
            "iii"=> ["out"],
        );
        assert_eq!(reactor.network, n);
    }
    fn sample_data() -> Vec<&'static str> {
        r#" aaa: you hhh
            you: bbb ccc
            bbb: ddd eee
            ccc: ddd eee fff
            ddd: ggg
            eee: out
            fff: out
            ggg: out
            hhh: ccc fff iii
            iii: out"#
            .split("\n")
            .map(|x| x.trim())
            .collect()
    }
}
