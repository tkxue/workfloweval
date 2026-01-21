use super::*;

use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct Vfs_Node {
    name: String,
    full_name: String,
    children: BTreeMap<String, Vfs_Node>,
}

impl Vfs_Node {
    fn new(name: &str, full_name: &str) -> Self {
        Vfs_Node {
            full_name: full_name.to_string(),
            name: name.to_string(),
            children: BTreeMap::new(),
        }
    }

    /// Recursively inserts a path represented as a slice of strings
    fn insert(&mut self, path: &[&str], full_name: &str) {
        if let Some((&first, rest)) = path.split_first() {
            // Find or create the child node
            let child = self
                .children
                .entry(first.to_string())
                .or_insert_with(|| Vfs_Node::new(first, full_name));

            // Recurse into the child with the remaining path
            child.insert(rest, full_name);
        }
    }

    pub fn build_all() -> Vfs_Node {
        let fnames =
            include_str!("/r/data/xls/lst").lines().collect::<Vec<_>>();

        let mut root = Vfs_Node::new("root", "");

        for path in fnames {
            let parts: Vec<&str> = path
                .split('/')
                .filter(|s| !s.is_empty() && *s != ".")
                .collect();
            root.insert(&parts, path);
        }

        root

        // Print the tree
        // wlog!("{:#?}", root);
    }

    pub fn to_ltree_node(&self) -> LTree_Node {
        LTree_Node::Directory {
            name: self.name.clone(),
            children: self
                .children
                .iter()
                .map(|(k, v)| {
                    if v.children.is_empty() {
                        LTree_Node::Leaf {
                            name: k.clone(),
                            full_name: v.full_name.clone(),
                        }
                    } else {
                        v.to_ltree_node()
                    }
                })
                .collect::<Vec<_>>(),
        }
    }
}
