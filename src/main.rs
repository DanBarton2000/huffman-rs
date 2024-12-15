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

#[derive(Eq, PartialEq)]
struct Node {
    is_leaf: bool,
    character: Option<char>,
    count: u32,
    left: Option<Box<Node>>,
    right:  Option<Box<Node>>
}

impl Node {
    fn new_leaf(character: char, count: u32) -> Node {
        Node {
            is_leaf: true,
            character: Some(character),
            count,
            left: None,
            right: None
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.count.cmp(&self.count)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn binary_heap_from_frequencies(frequencies: &HashMap<char, u32>) -> BinaryHeap<Node> {
    let mut heap = BinaryHeap::new();

    for (key, value) in frequencies {
        heap.push(Node::new_leaf(*key, *value));
    }

    heap
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
        let mut frequencies: HashMap<char, u32> = HashMap::new();
        frequencies.insert('C', 32);
        frequencies.insert('D', 42);
        frequencies.insert('E', 120);
        frequencies.insert('K', 7);
        frequencies.insert('L', 42);
        frequencies.insert('M', 24);
        frequencies.insert('U', 37);
        frequencies.insert('Z', 2);

        let mut heap = binary_heap_from_frequencies(&frequencies);

        let expected = [2, 7, 24, 32, 37, 42, 42, 120];

        let mut index = 0;
        while let Some(node) = heap.pop() {
            assert_eq!(node.count, expected[index]);
            index += 1;
        }
    }
}