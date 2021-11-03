use sequence_trie::SequenceTrie;

fn main() {
    let mut trie: SequenceTrie<char, String> = SequenceTrie::new();
    trie.insert(&['h', 'e', 'e'], String::from("hbe"));
    trie.insert(&['h', 'e', 'l', 'l', 'o'], String::from("hello"));
    // println!(
    //     "{:?}",
    //     trie.get_node(&['h', 'e'])
    //         .unwrap()
    //         .keys()
    //         .flatten()
    //         .collect::<Vec<&char>>()
    //         .into_iter()
    //         .collect::<String>()
    // );
    let results = trie
        .get_node(&['h', 'e'])
        .unwrap()
        .values();

    results.into_iter().for_each(|x| println!("{}", x));
}
