use sequence_trie::SequenceTrie;
mod json;

fn main() {
    let cities = json::read_json_file(String::from("cities.json"));

    let mut trie: SequenceTrie<char, String> = SequenceTrie::new();
    trie.insert(&['h', 'e', 'e'], String::from("hbe"));
    trie.insert(&['h', 'e', 'l', 'l', 'o'], String::from("hello"));

    for city in cities.iter() {
        trie.insert(&city.chars().collect::<Vec<char>>(), String::from(city));
    }
    dbg!(cities);

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
    let results = trie.get_node(&['a', 't']).unwrap().values();

    results.into_iter().for_each(|x| println!("{}", x));
}
