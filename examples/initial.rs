use typeahead_rs::{get_input, get_results};

extern crate typeahead_rs;

fn main() {
    let data = typeahead_rs::get_data(None);
    let trie = typeahead_rs::build_trie(data);
    println!("Input: ");
    let input = get_input(None);

    let r = get_results(&trie, input);
    dbg!(r);
}
