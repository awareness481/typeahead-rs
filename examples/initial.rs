use typeahead_rs::get_results;

extern crate typeahead_rs;

fn main() {
    let data = typeahead_rs::get_data(None);
    let trie = typeahead_rs::build_trie(data);
    println!("Input: ");

    let r = get_results(&trie);
    dbg!(r);
}
