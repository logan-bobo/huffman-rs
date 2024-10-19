use std::{
    char,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    error::Error,
    fmt::Display,
    fs,
};

pub struct Config {
    pub input_file: String,
    pub output_file: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Incorrect arguments supplied");
        }

        let input_file = args[1].clone();
        let output_file = args[2].clone();

        Ok(Config {
            input_file,
            output_file,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.input_file)?;

    let char_table = generate_char_table(contents);

    // we are fine to consume the char_table here as
    // its never needed again when we have converted
    // the keys and values to HuffNode
    let queue = build_priority_queue(char_table);

    // TODO: Move the build huff tree to HuffTree::from_queue()
    let huff_tree = build_huff_tree(queue);

    println!("{huff_tree:#?}");

    let huff_table = HuffTable::from_huff_tree(huff_tree);

    println!("{huff_table:#?}");

    Ok(())
}

fn generate_char_table(contents: String) -> HashMap<char, usize> {
    contents.chars().fold(HashMap::new(), |mut acc, char| {
        *acc.entry(char).or_insert(0) += 1;
        acc
    })
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
    root: Box<HuffNode>,
}

impl HuffTree {
    fn new_leaf(element: char, weight: usize) -> Self {
        Self {
            root: Box::new(HuffNode::Leaf { element, weight }),
        }
    }

    fn new_internal(left: HuffNode, right: HuffNode) -> Self {
        let left_weight = match left {
            HuffNode::Internal { weight, .. } => weight,
            HuffNode::Leaf { weight, .. } => weight,
        };

        let right_weight = match right {
            HuffNode::Internal { weight, .. } => weight,
            HuffNode::Leaf { weight, .. } => weight,
        };

        let total_weight = left_weight + right_weight;

        let (left, right) = if left_weight <= right_weight {
            (left, right)
        } else {
            (right, left)
        };

        Self {
            root: Box::new(HuffNode::Internal {
                weight: total_weight,
                left: Box::new(left),
                right: Box::new(right),
            }),
        }
    }

    fn weight(&self) -> usize {
        match *self.root {
            HuffNode::Internal { weight, .. } => weight,
            HuffNode::Leaf { weight, .. } => weight,
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
        self.weight()
            .cmp(&other.weight())
            .then_with(|| match (&*self.root, &*other.root) {
                (HuffNode::Leaf { .. }, HuffNode::Internal { .. }) => Ordering::Less,
                (HuffNode::Internal { .. }, HuffNode::Leaf { .. }) => Ordering::Greater,
                _ => Ordering::Equal,
            })
    }
}

#[derive(Debug)]
struct HuffTable {
    rows: Vec<HuffTableRow>,
}

impl HuffTable {
    fn new() -> Self {
        Self { rows: Vec::new() }
    }

    fn add_row(&mut self, char: char, frequency: usize, code: usize, bits: usize) {
        self.rows.push(HuffTableRow {
            char,
            frequency,
            code,
            bits,
        });
    }

    fn from_huff_tree(huff_tree: Reverse<HuffTree>) -> Self {
        let mut table = Self::new();
        Self::traverse_tree(&huff_tree.0.root, 0, 0, &mut table);
        table
    }

    fn traverse_tree(node: &HuffNode, code: usize, bits: usize, table: &mut Self) {
        match node {
            HuffNode::Leaf { element, weight } => {
                table.add_row(*element, *weight, code, bits);
            }
            HuffNode::Internal { left, right, .. } => {
                Self::traverse_tree(left, code << 1, bits + 1, table);
                Self::traverse_tree(right, (code << 1) | 1, bits + 1, table);
            }
        }
    }
}

impl Display for HuffTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = self.rows.iter().map(|value| {
            write!(
                f,
                "{}, {}, {}, {}",
                value.char, value.frequency, value.code, value.bits
            )
        });
        Ok(())
    }
}

#[derive(Debug)]
struct HuffTableRow {
    char: char,
    frequency: usize,
    code: usize,
    bits: usize,
}

impl Display for HuffTableRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.char, self.frequency, self.code, self.bits
        )
    }
}

fn build_priority_queue(char_table: HashMap<char, usize>) -> BinaryHeap<Reverse<HuffTree>> {
    // the initial queue will be leaf nodes only however at some point this
    // needs to handle internal nodes...
    char_table
        .iter()
        .map(|(key, value)| {
            // max-heap is largest first so we want
            // to use min-heap to build the Huffman tree
            Reverse(HuffTree::new_leaf(*key, *value))
        })
        .collect()
}

fn build_huff_tree(mut queue: BinaryHeap<Reverse<HuffTree>>) -> Reverse<HuffTree> {
    while queue.len() > 1 {
        let tmp1 = queue.pop().expect("expect better error handling");
        let tmp2 = queue.pop().expect("expect better error handling");
        let combined = HuffTree::new_internal(*tmp1.0.root, *tmp2.0.root);
        queue.push(Reverse(combined));
    }
    queue.pop().expect("better error handling")
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

    #[test]
    fn create_priority_queue() {
        let content = generate_char_table("abbcccdddd".to_string());

        // order is not guaranteed when iterating over a hash map so
        // we need to ensure in this test that the smallest occurrence
        // does not happen twice
        let mut queue = build_priority_queue(content);

        if let Some(value) = queue.pop() {
            match *value.0.root {
                HuffNode::Leaf { element, .. } => {
                    assert_eq!('a', element)
                }
                // We never build a queue from internal nodes
                // They are only ever pushed to the queue
                HuffNode::Internal { .. } => {
                    panic!("oh no we should not build a queue from internal nodes")
                }
            }
        }
    }

    #[test]
    fn create_huff_tree() {
        let content = generate_char_table("aaaaabbbccd\n".to_string());
        let queue = build_priority_queue(content);
        let huff_tree = build_huff_tree(queue);

        assert_eq!(huff_tree.0.weight(), 12);
    }

    #[test]
    fn create_huff_table() {
        let content = generate_char_table("aaaaabbbccd\n".to_string());
        let queue = build_priority_queue(content);
        let huff_tree = build_huff_tree(queue);
        let huff_table = HuffTable::from_huff_tree(huff_tree);

        let expected_table = HuffTable {
            rows: Vec::from([
                HuffTableRow { char: 'a', frequency: 5, code: 0, bits: 1 },
                HuffTableRow { char: 'b', frequency: 3, code: 2, bits: 2 },
                HuffTableRow { char: 'c', frequency: 2, code: 6, bits: 3 },
                // Generation for nodes of equal length is not deterministic
            ]),
        };
        
        for i in 0..expected_table.rows.len() {
            let expected_row: &HuffTableRow = &expected_table.rows[i];
            let huff_row: &HuffTableRow = &huff_table.rows[i];
            assert_eq!(expected_row.char, huff_row.char);
            assert_eq!(expected_row.frequency, huff_row.frequency);
            assert_eq!(expected_row.code, huff_row.code);
            assert_eq!(expected_row.bits, huff_row.bits);
        }
    }
}
