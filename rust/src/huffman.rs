use huffman::HuffmanCodeTree::*;
use std::collections::*;
use std::cmp::Ordering;

pub type ProbabilityDistribution = HashMap<char, i32>;
pub type EncodingTable = HashMap<char, String>;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HuffmanCode {
    tree: HuffmanCodeTree,
    encoding_table: EncodingTable,
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HuffmanCodeTree {
    HuffmanCodeTreeLeaf { value: char, probability: i32 },
    HuffmanCodeTreeBranch {
        probability: i32,
        left: Box<HuffmanCodeTree>,
        right: Box<HuffmanCodeTree>,
    },
}

impl HuffmanCodeTree {
    pub fn probability(&self) -> i32 {
        match self {
            &HuffmanCodeTreeLeaf { probability, .. } => probability,
            &HuffmanCodeTreeBranch { probability, .. } => probability,
        }
    }
}

impl Ord for HuffmanCodeTree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.probability().cmp(&other.probability()).reverse()
    }
}

impl PartialOrd for HuffmanCodeTree {
    fn partial_cmp(&self, other: &HuffmanCodeTree) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl HuffmanCodeTree {
    fn generate_encoding_table(&self) -> EncodingTable {
        match self {
            &HuffmanCodeTreeLeaf { value, .. } => {
                let mut map = EncodingTable::new();
                map.insert(value, "".to_string());
                map
            }
            &HuffmanCodeTreeBranch {
                ref left,
                ref right,
                ..
            } => {
                let mut map = EncodingTable::new();

                for (character, string) in left.generate_encoding_table() {
                    map.insert(character, "0".to_string() + &string);
                }

                for (character, string) in right.generate_encoding_table() {
                    map.insert(character, "1".to_string() + &string);
                }

                map
            }
        }
    }
}

impl HuffmanCode {
    pub fn from_sample_text(text: &str) -> HuffmanCode {
        let mut distribution = ProbabilityDistribution::new();

        for character in text.chars() {
            *distribution.entry(character).or_insert(0) += 1;
        }

        let mut trees: BinaryHeap<HuffmanCodeTree> = BinaryHeap::new();
        for (key, value) in distribution.into_iter() {
            trees.push(HuffmanCodeTreeLeaf {
                           value: key,
                           probability: value,
                       });
        }

        while trees.len() > 1 {
            let lowest = trees.pop().unwrap();
            let lower = trees.pop().unwrap();
            let joint_probability = lowest.probability() + lower.probability();
            trees.push(HuffmanCodeTreeBranch {
                           left: Box::new(lowest),
                           right: Box::new(lower),
                           probability: joint_probability,
                       })
        }

        let tree = trees.pop().unwrap();
        let encoding_table = tree.generate_encoding_table();
        HuffmanCode {
            tree: tree,
            encoding_table: encoding_table,
        }
    }

    pub fn encode(&self, string: &String) -> String {
        string
            .chars()
            .map(|character: char| self.encoding_table.get(&character).unwrap().clone())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn decode(&self, string: &String) -> String {
        let mut ret = "".to_string();
        let mut pointer = &self.tree;
        let chars: Vec<char> = string.chars().collect();

        let mut i = 0;

        while i < chars.len() {
            let character = chars[i];
            match (character, pointer) {
                ('0', &HuffmanCodeTreeBranch { ref left, .. }) => {
                    pointer = left;
                    i += 1;
                }
                ('1', &HuffmanCodeTreeBranch { ref right, .. }) => {
                    pointer = right;
                    i += 1;
                }
                (_, &HuffmanCodeTreeLeaf { value, .. }) => {
                    ret.push(value);
                    pointer = &self.tree;
                }
                (_, _) => panic!("I should not be here {}", character),
            }
        }

        match *pointer {
            HuffmanCodeTreeLeaf { value, .. } => ret.push(value),
            HuffmanCodeTreeBranch { .. } => panic!("Terminated unexpectedly when parsing"),
        }

        ret
    }
}

