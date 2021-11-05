use typeahead_rs::get_results;

extern crate typeahead_rs;

fn main() {
    let trie = typeahead_rs::build_trie();
    println!("Input: ");

    let r = get_results(&trie);
    dbg!(r);
}
