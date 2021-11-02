use sequence_trie::SequenceTrie;

fn main() {
    let mut trie: SequenceTrie<char, i32> = SequenceTrie::new();
    trie.insert(&['h', 'b', 'e'], 3);
    trie.insert(&['h', 'e', 'l', 'l', 'o'], 5);
    print!(
        "{:?}",
        trie.get_node(&['h', 'e'])
            .unwrap()
            .keys()
            .flatten()
            .collect::<Vec<&char>>()
            .into_iter()
            .collect::<String>()
    );
}
