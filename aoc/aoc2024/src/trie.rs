use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Trie<T> 
where T: Eq + Hash + Copy {
    pub is_leaf: bool,
    pub children: HashMap<T, Trie<T>>
}

impl <T> Trie<T>
where T: Eq + Hash + Copy {
    pub fn new() -> Trie<T> {
        Trie {
            is_leaf: false,
            children: HashMap::new()
        }
    }

    pub fn push(&mut self, item: &[T]) {
        let mut node = self;
        for &i in item {
            node = node.children.entry(i).or_insert_with(Trie::new);
        }
        node.is_leaf = true;
    }
}