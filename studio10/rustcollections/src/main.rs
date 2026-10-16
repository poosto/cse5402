use std::collections::{BTreeMap, BTreeSet, HashSet};

fn main() {
    let text = "She sells sea shells by the sea shore.".to_string();
    let vec: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    let btree: BTreeSet<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    let hash: HashSet<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    let bmap: BTreeMap<usize, String> = vec.iter().cloned().enumerate().collect();
    let bmap_reverse: BTreeMap<String, usize> = vec
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect();

    println!("Vec: {:?}", vec);
    println!("BTree: {:?}", btree);
    println!("Hash: {:?}", hash);
    println!("BMap: {:?}", bmap);
    println!("BMapRev: {:?}", bmap_reverse);
}
