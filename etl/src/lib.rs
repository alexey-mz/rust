use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut res = BTreeMap::new();
    for (&k, v) in h {
        v.iter().for_each(|&x| {
            res.insert(x.to_ascii_lowercase(), k);
        })
    }
    res
}
