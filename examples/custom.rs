use typeahead_rs::get_results;

extern crate typeahead_rs;

fn main() {
    let data = vec![["athens", "greece"], ["madrid", "spain"]];
    let mut trie = typeahead_rs::create_trie();


    // We add more text in the value, to add context for the results.
    for city in data.iter() {
        trie.insert(&city[0].chars().collect::<Vec<char>>(), format!("{}, {}", city[0], city[1]));
    }

    println!("Input: ");

    let r = get_results(&trie);
    dbg!(r);
}
