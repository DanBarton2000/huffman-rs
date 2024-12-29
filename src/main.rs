use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("No filename provided");
    }

    let file = File::open(&args[1]).unwrap();
    let mut reader = BufReader::new(file);
    let frequencies: HashMap<char, u32> = get_frequencies_from_reader(&mut reader).unwrap();

    for (key, value) in frequencies {
        println!("{} {}", key, value);
    }
}

#[derive(Debug, Clone)]
enum HuffmanNode {
    Internal { left: Box<HuffmanNode>, right: Box<HuffmanNode> },
    Leaf { character: char, frequency: usize },
}

impl HuffmanNode {
    fn frequency(&self) -> usize {
        match self {
            HuffmanNode::Internal { left, right } => left.frequency() + right.frequency(),
            HuffmanNode::Leaf { frequency, .. } => *frequency,
        }
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = other.frequency().cmp(&self.frequency());
        if ordering == Ordering::Equal {
            let HuffmanNode::Leaf { character: self_char, frequency: _ } = self else { return ordering; };
            let HuffmanNode::Leaf { character, frequency: _ } = other else { return ordering; };
            character.cmp(self_char)
        } else {
            ordering
        }
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.frequency() == other.frequency()
    }
}

impl Eq for HuffmanNode {}

fn build_huffman_tree(freq_map: &HashMap<char, usize>) -> HuffmanNode {
    // Taken from https://opendsa-server.cs.vt.edu/ODSA/Books/CS3/html/Huffman.html
    let mut heap = BinaryHeap::new();

    for (&character, &frequency) in freq_map.iter() {
        heap.push(HuffmanNode::Leaf { character, frequency });
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();

        let internal = HuffmanNode::Internal {
            left: Box::new(left),
            right: Box::new(right),
        };

        heap.push(internal);
    }

    heap.pop().unwrap()
}

fn generate_huffman_codes(node: &HuffmanNode, prefix: String, codes: &mut HashMap<char, String>) {
    match node {
        HuffmanNode::Leaf { character, .. } => {
            codes.insert(*character, prefix);
        }
        HuffmanNode::Internal { left, right } => {
            generate_huffman_codes(left, prefix.clone() + "0", codes);
            generate_huffman_codes(right, prefix + "1", codes);
        }
    }
}

fn get_frequencies_from_reader<R: BufRead>(reader: &mut R) -> std::io::Result<HashMap<char, u32>> {
    let mut frequencies: HashMap<char, u32> = HashMap::new();
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        let frequencies_temp = get_frequencies(&line);

        for (key, value) in frequencies_temp {
            *frequencies.entry(key).or_insert(0) += value;
        }

        line.clear();
    }

    Ok(frequencies)
}

fn get_frequencies(line: &str) -> HashMap<char, u32> {
    let mut frequencies: HashMap<char, u32> = HashMap::new();
    for c in line.chars() {
        *frequencies.entry(c).or_insert(0) += 1;
    }

    frequencies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_frequencies() {
        let frequencies = get_frequencies("abbcccdddd");

        assert_eq!(frequencies[&'a'], 1);
        assert_eq!(frequencies[&'b'], 2);
        assert_eq!(frequencies[&'c'], 3);
        assert_eq!(frequencies[&'d'], 4);
    }

    #[test]
    fn test_frequencies_from_reader() {
        let mut cursor = std::io::Cursor::new(b"test\nmyreallycooltest");
        let frequencies = get_frequencies_from_reader(&mut cursor).unwrap();

        assert_eq!(frequencies[&'t'], 4);
        assert_eq!(frequencies[&'e'], 3);
        assert_eq!(frequencies[&'s'], 2);
        assert_eq!(frequencies[&'m'], 1);
        assert_eq!(frequencies[&'y'], 2);
        assert_eq!(frequencies[&'r'], 1);
        assert_eq!(frequencies[&'a'], 1);
        assert_eq!(frequencies[&'l'], 3);
        assert_eq!(frequencies[&'c'], 1);
        assert_eq!(frequencies[&'o'], 2);
        assert_eq!(frequencies[&'\n'], 1);
        assert_eq!(frequencies.get(&'d'), None);
    }

    #[test]
    fn test_binary_heap_from_frequencies() {
        let mut frequencies: HashMap<char, usize> = HashMap::new();
        frequencies.insert('C', 32);
        frequencies.insert('D', 42);
        frequencies.insert('E', 120);
        frequencies.insert('K', 7);
        frequencies.insert('L', 42);
        frequencies.insert('M', 24);
        frequencies.insert('U', 37);
        frequencies.insert('Z', 2);

        let root = build_huffman_tree(&frequencies);
        let mut huffman_codes: HashMap<char, String> = HashMap::new();
        generate_huffman_codes(&root, String::new(), &mut huffman_codes);

        let expected = HashMap::from([
            ('M', "11111"),
            ('D', "101"),
            ('U', "100"),
            ('C', "1110"),
            ('E', "0"),
            ('K', "111101"),
            ('L', "110"),
            ('Z', "111100")
        ]);

        for (character, code) in &huffman_codes {
            assert!(expected.get(character).is_some());
            assert_eq!(code, expected[character]);
        }
    }
}