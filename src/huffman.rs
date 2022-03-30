use std::collections::HashMap;
use itertools::Itertools;

fn str_to_tokfreq(txt: &str) -> Option<Vec<TokFreq>> {
    let mut count_acc: HashMap<char, u32> = HashMap::new();

    for c in txt.chars() {
        count_acc.entry(c)
            .and_modify(|i| *i += 1)
            .or_insert(1);
    }

    let mut out_acc = vec![];

    for (k, v) in count_acc {
        out_acc.push( TokFreq { tok: k, freq: v } )
    }

    Some(sort_desc_tokfreq(out_acc))
}

fn sort_desc_tokfreq(to_sort: Vec<TokFreq>) -> Vec<TokFreq> {
    to_sort.into_iter().sorted_by(|a, b| b.freq.cmp(&a.freq)).collect::<Vec<TokFreq>>()
}

#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub struct TokFreq {
    pub tok: char,
    pub freq: u32,
}

impl TokFreq {
    pub fn from(tok: char, freq: u32) -> TokFreq {
        TokFreq{tok, freq}
    }
}




#[cfg(test)]
mod test {
    use super::*;
    use itertools::assert_equal;

    #[test]
    fn tokfreq_check() {
        let res = str_to_tokfreq(&"aaaaffddddddddddddd").unwrap();
        let expected = vec![ TokFreq::from('d', 13), TokFreq::from('a', 4), TokFreq::from('f', 2) ];
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

}