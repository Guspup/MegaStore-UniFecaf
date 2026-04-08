use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize)]
pub struct StringPool {
    pub forward: HashMap<String, u32>,
    pub reverse: HashMap<u32, String>,
    pub next_id: u32,
}

impl StringPool {
    pub fn get_or_intern(&mut self, s: &str) -> u32 {
        let s = s.to_lowercase();
        if let Some(&id) = self.forward.get(&s) { return id; }
        let id = self.next_id;
        self.forward.insert(s.clone(), id);
        self.reverse.insert(id, s);
        self.next_id += 1;
        id
    }
    pub fn resolve(&self, id: u32) -> &str {
        self.reverse.get(&id).map(|s| s.as_str()).unwrap_or("Desconhecido")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub brand_id: u32,
    pub category_id: u32,
    pub price: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    Product(u32),
    Term(u32),
}
