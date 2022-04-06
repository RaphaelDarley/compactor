use crate::utils::*;
use itertools::Itertools;
use std::{collections::HashMap, ops::RemAssign};

pub fn list_to_tree(mut nodes: Vec<HuffNode>) -> HuffNode {
    println!("{:?}", nodes);
    loop {
        if nodes.len() < 2 {
            break;
        }
        nodes = sort_desc_tokfreq(nodes);
        let right_node = nodes.pop().unwrap();
        let left_node = nodes.pop().unwrap();
        nodes.push(HuffNode::from_nodes(left_node, right_node));
    }
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
    let mut temp;
    let mut remaining = encoding.as_slice();

    loop {
        (temp, remaining) = traverse_tree(tree, remaining);
        acc.push(temp);
        if remaining.len() <= 1 {
            break;
        }
    }

    acc
}

fn traverse_tree<'a>(tree: &HuffNode, path: &'a [TreePath]) -> (char, &'a [TreePath]) {
    match &tree.value {
        HuffValue::Tok(c) => {
            return (*c, &path[1..]);
        }
        HuffValue::Conns(c) => {
            let conns = c;
            match &path[0] {
                TreePath::Left => traverse_tree(&*conns.left, &path[1..]),
                TreePath::Right => traverse_tree(&*conns.right, &path[1..]),
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

    // #[test]
    // fn test_simple_tree_build() {
    //     let mut nodes = str_to_tokfreq(&"aaaaffddddddddddddd").unwrap();
    //     loop{
    //         if nodes.len() < 2 {break}
    //         let right_node = nodes.pop().unwrap();
    //         let left_node = nodes.pop().unwrap();
    //         nodes.push(HuffNode::from_nodes(left_node, right_node));
    //         nodes = sort_desc_tokfreq(nodes);
    //     }
    //     println!("{:?}", nodes.pop().unwrap());
    //     let expected = HuffNode{value: HuffValue::Conns( HuffConns{left: Box } ), freq: 19}
    //     panic!()
    // }
}
