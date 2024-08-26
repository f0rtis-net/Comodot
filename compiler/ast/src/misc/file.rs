use crate::{AstNode};

pub struct ParsedFile {
    name: String,
    path: String,
    hash: String,
    pub expressions: Vec<AstNode>,
}

impl ParsedFile {
    pub fn new(name: String, hash: String, path: String, expressions: Vec<AstNode>) -> Self {
        Self {
            name,
            path,
            hash,
            expressions
        }
    }

    pub fn get_hash(&self) -> String { self.hash.clone() }

    pub fn get_name(&self) -> String { self.name.clone() }

    pub fn get_path(&self) -> String { self.path.clone() }
}