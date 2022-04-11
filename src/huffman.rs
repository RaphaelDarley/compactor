use crate::utils::*;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

pub fn list_to_tree(mut nodes: Vec<HuffNode>) -> HuffNode {
    if nodes.len() < 1 {
        nodes.push(HuffNode::from_tok(0 as char, 0)) // adds null node to stop empty coding with one character
    }
    println!("initial nodes: {:?}", nodes);
    loop {
        if nodes.len() < 2 {
            break;
        }
        nodes = sort_desc_tokfreq(nodes);
        let right_node = nodes.pop().unwrap();
        let left_node = nodes.pop().unwrap();
        nodes.push(HuffNode::from_nodes(left_node, right_node));
    }
    println!("final node: {:?}", nodes);
    nodes.pop().unwrap()
}

pub fn tree_to_code(node: &HuffNode, path: Vec<TreePath>) -> HashMap<char, Vec<TreePath>> {
    let mut acc = HashMap::new();

    match &node.value {
        HuffValue::Tok(tok) => {
            acc.insert(*tok, path);
        }
        HuffValue::Conns(conns) => {
            acc.extend(tree_to_code(&*conns.left, path.appended(TreePath::Left)));
            acc.extend(tree_to_code(&*conns.right, path.appended(TreePath::Right)));
        }
    }
    acc
}

pub fn encode(text: &str) -> (Vec<TreePath>, HuffNode) {
    let tree = list_to_tree(str_to_tokfreq(text).unwrap());
    let code = tree_to_code(&tree, vec![]);
    (
        text.chars().map(|c| code[&c].clone()).flatten().collect(),
        tree,
    )
}

pub fn decode(encoding: Vec<TreePath>, tree: &HuffNode) -> String {
    let mut acc = String::new();
    let mut remaining = VecDeque::from(encoding);

    loop {
        if remaining.len() < 1 {
            break;
        }
        acc.push(traverse_tree(tree, &mut remaining));
        println!("{}", acc);
    }

    acc
}

fn traverse_tree<'a>(tree: &HuffNode, path: &mut VecDeque<TreePath>) -> char {
    println!("{:?}", path);
    match &tree.value {
        HuffValue::Tok(c) => {
            return *c;
        }
        HuffValue::Conns(c) => {
            let conns = c;
            match path.pop_front().unwrap() {
                TreePath::Left => traverse_tree(&*conns.left, path),
                TreePath::Right => traverse_tree(&*conns.right, path),
            }
        }
    }
}

pub fn str_to_tokfreq(txt: &str) -> Option<Vec<HuffNode>> {
    let mut count_acc: HashMap<char, u32> = HashMap::new();

    for c in txt.chars() {
        count_acc.entry(c).and_modify(|i| *i += 1).or_insert(1);
    }

    let mut out_acc = vec![];

    for (k, v) in count_acc {
        out_acc.push(HuffNode {
            value: HuffValue::Tok(k),
            freq: v,
        })
    }

    Some(sort_desc_tokfreq(out_acc))
}

fn sort_desc_tokfreq(to_sort: Vec<HuffNode>) -> Vec<HuffNode> {
    to_sort
        .into_iter()
        .sorted_by(|a, b| b.freq.cmp(&a.freq))
        .collect::<Vec<HuffNode>>()
}

#[derive(Clone, Debug)]
pub enum TreePath {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Debug)]
pub struct HuffNode {
    pub value: HuffValue,
    pub freq: u32,
}

#[derive(PartialEq, Eq, Debug)]
pub enum HuffValue {
    Tok(char),
    Conns(HuffConns),
}

#[derive(PartialEq, Eq, Debug)]
pub struct HuffConns {
    left: Box<HuffNode>,
    right: Box<HuffNode>,
}

impl HuffNode {
    pub fn from_tok(tok: char, freq: u32) -> HuffNode {
        HuffNode {
            value: HuffValue::Tok(tok),
            freq,
        }
    }
    pub fn from_nodes(left: HuffNode, right: HuffNode) -> HuffNode {
        let combinded_freq = left.freq + right.freq;
        HuffNode {
            value: HuffValue::Conns(HuffConns {
                left: Box::new(left),
                right: Box::new(right),
            }),
            freq: combinded_freq,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::assert_equal;
    use rand::{self, Rng};

    #[test]
    fn tokfreq_check() {
        let res = str_to_tokfreq(&"aaaaffddddddddddddd").unwrap();
        let expected = vec![
            HuffNode::from_tok('d', 13),
            HuffNode::from_tok('a', 4),
            HuffNode::from_tok('f', 2),
        ];
        println!("result: {:?}", res);
        assert_equal(res, expected);
    }

    #[test]
    fn tokfreq_check_empty() {
        let res = str_to_tokfreq(&"").unwrap();
        let expected = vec![];
        println!("result: {:?}", res);
        assert_equal(res, expected);
    }

    #[test]
    fn test_simple_tree_build() {
        let nodes = str_to_tokfreq(&"abbb").unwrap();
        let l_node = HuffNode::from_tok('b', 3);
        let r_node = HuffNode::from_tok('a', 1);
        let expected = HuffNode::from_nodes(l_node, r_node);
        assert_eq!(list_to_tree(nodes), expected);
    }

    fn test_encode_decode(test_value: &str) {
        let (test_encode, test_tree) = encode(&test_value);

        println!("encoding: {:?}", test_encode);
        println!("encoding: {:?}", test_tree);

        let test_decode = decode(test_encode, &test_tree);

        println!("decoded: {:?}", test_decode);
        assert_eq!(test_value, test_decode)
    }

    #[test]
    fn empty_encode_decode() {
        test_encode_decode("");
    }

    #[test]
    fn random_short_encode_decode() {
        let mut rng = rand::thread_rng();
        for _ in 0..32 {
            let length: u8 = rng.gen();
            let mut test_value = String::new();
            for _ in 0..length {
                test_value.push(rng.gen::<u8>() as char);
            }
            test_encode_decode(&test_value);
        }
    }

    #[test]
    fn random_long_encode_decode() {
        let mut rng = rand::thread_rng();
        for _ in 0..2 {
            let length: u16 = 1000;
            let mut test_value = String::new();
            for _ in 0..length {
                test_value.push(rng.gen::<u8>() as char);
            }
            test_encode_decode(&test_value);
        }
    }
}
