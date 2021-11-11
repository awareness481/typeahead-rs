use sequence_trie::SequenceTrie;
use std::io::stdin;

mod json;

pub fn get_results(trie: &SequenceTrie<char, String>, text: String) -> Vec<&String> {
    let input = text.to_lowercase();

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
    remove_new_line(input)
}

fn remove_new_line(input: String) -> String {
    let mut new_input = input;
    let input_length = new_input.chars().count();
    if new_input.chars().nth(input_length - 1).unwrap() == '\n' {
        new_input.pop();
    }
    new_input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trie_create_and_find() {
        let mut trie = create_trie();

        trie.insert(&['a', 't', 'h'], String::from("ath"));
        assert!(trie.values().count() == 1);

        let find_item = trie.get(&['a', 't', 'h']).unwrap();
        assert!(find_item == "ath");
    }

    #[test]
    fn input() {
        let mut text = String::from("hello");
        assert!(remove_new_line(text) == *"hello");
        text = String::from("hello\n");
        assert!(remove_new_line(text) == *"hello");
    }
}
