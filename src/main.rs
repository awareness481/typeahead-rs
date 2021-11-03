use sequence_trie::SequenceTrie;
mod json;

fn main() {
    let cities = json::read_json_file(String::from("cities.json"));

    let mut trie: SequenceTrie<char, String> = SequenceTrie::new();
    for city in cities.iter() {
        trie.insert(&city.chars().collect::<Vec<char>>(), String::from(city));
    }
    
    let results = trie.get_node(&['a', 't']).unwrap().values();

    results.into_iter().for_each(|x| println!("{}", x));
}
