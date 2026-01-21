#[derive(Default)]
pub struct Node {
    pub children: Vec<Node>,
    pub key: Option<char>,
    pub value: Option<String>,
    pub count: usize,
}

impl Node {
    pub fn new() -> Self {
        Node::default()
    }

    pub fn with_key(c: char) -> Self {
        Node {
            key: Some(c),
            ..Default::default()
        }
    }
}

pub struct Trie {
    pub root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Trie { root: Node::new() }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;

        for ch in word.chars() {
            match current
                .children
                .binary_search_by(|node| node.key.cmp(&Some(ch)))
            {
                Ok(index) => {
                    current = &mut current.children[index];
                }
                //if once error occured we dont need to match
                Err(index) => {
                    current.children.insert(index, Node::with_key(ch));
                    current = &mut current.children[index];
                }
            }
        }

        current.count += 1;
        current.value = Some(word.to_string());
    }

    pub fn exists(&self, word: &str) -> bool {
        let mut current = &self.root;

        for ch in word.chars() {
            match current
                .children
                .binary_search_by(|node| node.key.cmp(&Some(ch)))
            {
                Ok(index) => {
                    current = &current.children[index];
                }
                Err(_) => {
                    return false;
                }
            }
        }
        current.count > 0
    }

    pub fn search(&self, prefix: &str) -> Vec<String> {
        let mut current = &self.root;

        for ch in prefix.chars() {
            match current
                .children
                .binary_search_by(|node| node.key.cmp(&Some(ch)))
            {
                Ok(index) => {
                    current = &current.children[index];
                }
                Err(_) => {
                    return Vec::new();
                }
            }
        }
        //te
        //tea
        //teaspoon

        let mut results: Vec<(usize, String)> = Vec::new();
        let mut stack: Vec<&Node> = vec![current];

        while let Some(node) = stack.pop() {
            if node.count > 0 {
                results.push((node.count, node.value.as_ref().unwrap().clone()));
            }

            for child in &node.children {
                stack.push(child);
            }
        }

        results.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));

        results.into_iter().map(|(_, word)| word).collect()
    }
}
