use std::{
    char,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    error::Error,
    fs, usize,
};

pub struct Config {
    pub file: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Incorrect arguments supplied");
        }

        let file = args[1].clone();

        Ok(Config { file })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;

    let char_table = generate_char_table(contents);

    println!("{char_table:#?}");

    let mut queue = build_priority_queue(char_table);

    while let Some(node) = queue.pop() {
        match node.0.root {
            HuffNode::Leaf { element, weight } => {
                println!("Value: {element}, weight: {weight}")
            }
            HuffNode::Internal { .. } => {}
        }
    }

    Ok(())
}

fn generate_char_table(contents: String) -> HashMap<char, usize> {
    contents.chars().fold(HashMap::new(), |mut acc, char| {
        *acc.entry(char).or_insert(0) += 1;
        acc
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_char_table() {
        let content: String = "hello".to_string();
        let table: HashMap<char, usize> = HashMap::from([('h', 1), ('e', 1), ('l', 2), ('o', 1)]);

        assert_eq!(table, generate_char_table(content))
    }
}

#[derive(Debug)]
enum HuffNode {
    Leaf {
        element: char,
        weight: usize,
    },
    Internal {
        weight: usize,
        left: Box<HuffNode>,
        right: Box<HuffNode>,
    },
}

#[derive(Debug)]
struct HuffTree {
    root: HuffNode,
}

impl HuffTree {
    fn new_leaf(element: char, weight: usize) -> Self {
        Self {
            root: HuffNode::Leaf { element, weight },
        }
    }

    fn new_internal(left: HuffNode, right: HuffNode, weight: usize) -> Self {
        Self {
            root: HuffNode::Internal {
                weight,
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    fn is_leaf(&self) -> bool {
        matches!(self.root, HuffNode::Leaf { .. })
    }

    fn weight(&self) -> usize {
        match self.root {
            HuffNode::Internal { weight, .. } => weight,
            HuffNode::Leaf { weight, .. } => weight,
        }
    }

    fn value(&self) -> Option<char> {
        match self.root {
            HuffNode::Internal { .. } => None,
            HuffNode::Leaf { element, .. } => Some(element),
        }
    }
}

impl PartialEq for HuffTree {
    fn eq(&self, other: &Self) -> bool {
        self.weight() == other.weight()
    }
}

impl Eq for HuffTree {}

impl PartialOrd for HuffTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.weight().cmp(&other.weight()))
    }
}

impl Ord for HuffTree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight().cmp(&other.weight())
    }
}


fn build_priority_queue(char_table: HashMap<char, usize>) -> BinaryHeap<Reverse<HuffTree>> {
    char_table
        .iter()
        .map(|(key, value)| {
            // max-heap is lagest first so we want to use min-heap to buld our huffman tree
            // https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#min-heap
            Reverse(HuffTree::new_leaf(*key, *value))
        })
        .collect()
}
