use sequence_trie::SequenceTrie;
use std::io::stdin;
mod json;

fn main() {
    let cities = json::read_json_file(String::from("cities.json"));

    let mut trie: SequenceTrie<char, String> = SequenceTrie::new();
    for city in cities.iter() {
        trie.insert(&city.chars().collect::<Vec<char>>(), String::from(city));
    }

    println!("Input: ");
    let mut input = get_input().to_lowercase();
    input.pop();

    let results = trie.get_node(&input.chars().collect::<Vec<char>>()).unwrap().values();

    results.into_iter().for_each(|x| println!("{}", x));
}

fn get_input() -> String {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }

    input
}