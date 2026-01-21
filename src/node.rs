pub struct Node {
    pub children: Vec<Node>,
    pub key: Option<char>,
    pub value: Option<String>,
    pub count: usize,
}

pub struct Trie {
    pub root: Node,
}
