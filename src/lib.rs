use sequence_trie::SequenceTrie;
use std::io::stdin;

mod json;

pub fn get_results(trie: &SequenceTrie<char, String>) -> Vec<&String> {
    let mut input = get_input(None).to_lowercase();
    input.pop();

    let results = trie
        .get_node(&input.chars().collect::<Vec<char>>())
        .unwrap();
    results.values().collect::<Vec<&String>>()
}

pub fn build_trie(data: Vec<String>) -> SequenceTrie<char, String> {
    let mut t = create_trie();

    for city in data.iter() {
        t.insert(&city.chars().collect::<Vec<char>>(), String::from(city));
    }

    t
}

pub fn create_trie() -> SequenceTrie<char, String> {
    let trie: SequenceTrie<char, String> = SequenceTrie::new();
    trie
}

pub fn get_data(path: Option<String>) -> Vec<String> {
    let input = match path {
        Some(res) => res,
        None => String::from("cities.json"),
    };

    json::read_json_file(input)
}

pub fn get_input(pre: Option<String>) -> String {
    let mut input = pre.unwrap_or_else(|| String::from(""));
    match stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }

    input
}
