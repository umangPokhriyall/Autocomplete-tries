use crate::node::Trie;

mod node;

fn main() {
    let mut trie = Trie::new();
    trie.insert("and");
    trie.insert("ant");
    trie.insert("dad");
    trie.insert("do");

    assert!(trie.exists("and"));
    assert!(trie.exists("ant"));
    assert!(trie.exists("dad"));

    assert_eq!(trie.search("an"), vec!["and", "ant"]);
    assert_eq!(trie.search("d"), vec!["dad", "do"]);

    trie.insert("dog");
    trie.insert("done");
    trie.insert("ant");

    assert_eq!(trie.search("do"), vec!["do", "dog", "done"]);
    assert_eq!(trie.search("an"), vec!["ant", "and"]);

    println!("all test passed");
}
