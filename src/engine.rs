use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, Read};
use crate::models::{Product, NodeType, StringPool};

pub struct MegaStoreSearch {
    pub products: Vec<Product>,
    pub pool: StringPool,
    pub graph: Graph<NodeType, (), Undirected>,
    pub term_to_node: HashMap<u32, NodeIndex>,
    pub product_to_node: HashMap<u32, NodeIndex>,
}

impl MegaStoreSearch {
    pub fn init(data_path: &str) -> io::Result<Self> {
        let mut products = Vec::new();
        let pool = StringPool::default();
        let mut file = File::open(data_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        
        let mut cursor = 0;
        while cursor + 4 <= buffer.len() {
            let len = u32::from_le_bytes(buffer[cursor..cursor+4].try_into().unwrap()) as usize;
            cursor += 4;
            let product: Product = bincode::deserialize(&buffer[cursor..cursor+len]).unwrap();
            cursor += len;
            products.push(product);
        }

        let mut engine = MegaStoreSearch {
            products,
            pool,
            graph: Graph::new_undirected(),
            term_to_node: HashMap::new(),
            product_to_node: HashMap::new(),
        };

        engine.build_graph();
        Ok(engine)
    }

    fn build_graph(&mut self) {
        let products_copy = self.products.clone();
        for p in &products_copy {
            let p_node = self.graph.add_node(NodeType::Product(p.id));
            self.product_to_node.insert(p.id, p_node);

            self.link_id(p_node, p.brand_id);
            self.link_id(p_node, p.category_id);
            
            for word in p.name.split_whitespace() {
                let w_id = self.pool.get_or_intern(word);
                self.link_id(p_node, w_id);
            }
        }
    }

    fn link_id(&mut self, p_node: NodeIndex, term_id: u32) {
        let t_node = *self.term_to_node.entry(term_id).or_insert_with(|| {
            self.graph.add_node(NodeType::Term(term_id))
        });
        self.graph.add_edge(p_node, t_node, ());
    }

    pub fn search(&self, query: &str) -> Vec<&Product> {
        let query_lc = query.to_lowercase();
        if let Some(&term_id) = self.pool.forward.get(&query_lc) {
            if let Some(&t_node) = self.term_to_node.get(&term_id) {
                let mut seen_ids = HashSet::new();
                return self.graph.neighbors(t_node)
                    .filter_map(|n| {
                        if let NodeType::Product(id) = self.graph[n] {
                            if seen_ids.insert(id) {
                                self.products.iter().find(|p| p.id == id)
                            } else { None }
                        } else { None }
                    })
                    .collect();
            }
        }
        vec![]
    }

    pub fn get_recs(&self, p: &Product) -> Vec<(&Product, &str)> {
        let mut recs = Vec::new();
        let mut seen = HashSet::new();
        seen.insert(p.id);

        if let Some(&p_node) = self.product_to_node.get(&p.id) {
            for t_node in self.graph.neighbors(p_node) {
                for sib in self.graph.neighbors(t_node) {
                    if let NodeType::Product(id) = self.graph[sib] {
                        if !seen.contains(&id) && recs.len() < 3 {
                            seen.insert(id);
                            let rel = self.products.iter().find(|pr| pr.id == id).unwrap();
                            let tipo = if rel.category_id == p.category_id { "Concorrente" } else { "Complemento" };
                            recs.push((rel, tipo));
                        }
                    }
                }
            }
        }
        recs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::StringPool;

    fn setup_test_engine() -> MegaStoreSearch {
        let mut pool = StringPool::default();
        let products = vec![
            Product { id: 0, name: "Dell Computador".into(), brand_id: pool.get_or_intern("Dell"), category_id: pool.get_or_intern("Computador"), price: 3000.0 },
            Product { id: 1, name: "HP Computador".into(), brand_id: pool.get_or_intern("HP"), category_id: pool.get_or_intern("Computador"), price: 2500.0 },
            Product { id: 2, name: "Dell Monitor".into(), brand_id: pool.get_or_intern("Dell"), category_id: pool.get_or_intern("Monitor"), price: 1000.0 },
            Product { id: 3, name: "Logitech Mouse".into(), brand_id: pool.get_or_intern("Logitech"), category_id: pool.get_or_intern("Mouse"), price: 150.0 },
            Product { id: 4, name: "Razer Mouse".into(), brand_id: pool.get_or_intern("Razer"), category_id: pool.get_or_intern("Mouse"), price: 400.0 },
            Product { id: 5, name: "Samsung Monitor".into(), brand_id: pool.get_or_intern("Samsung"), category_id: pool.get_or_intern("Monitor"), price: 1200.0 },
            Product { id: 6, name: "Apple Macbook".into(), brand_id: pool.get_or_intern("Apple"), category_id: pool.get_or_intern("Computador"), price: 8000.0 },
            Product { id: 7, name: "Logitech Teclado".into(), brand_id: pool.get_or_intern("Logitech"), category_id: pool.get_or_intern("Teclado"), price: 300.0 },
            Product { id: 8, name: "Dell Mouse".into(), brand_id: pool.get_or_intern("Dell"), category_id: pool.get_or_intern("Mouse"), price: 100.0 },
            Product { id: 9, name: "Samsung Celular".into(), brand_id: pool.get_or_intern("Samsung"), category_id: pool.get_or_intern("Celular"), price: 5000.0 },
        ];

        let mut engine = MegaStoreSearch {
            products,
            pool,
            graph: Graph::new_undirected(),
            term_to_node: HashMap::new(),
            product_to_node: HashMap::new(),
        };
        engine.build_graph();
        engine
    }

    #[test]
    fn test_search_by_brand() {
        let engine = setup_test_engine();
        let results = engine.search("Dell");
        assert!(results.iter().any(|p| p.name.contains("Dell Computador")));
        assert!(results.iter().any(|p| p.name.contains("Dell Monitor")));
        assert_eq!(results.len(), 3); // Dell Computador, Monitor, Mouse
    }

    #[test]
    fn test_search_by_category() {
        let engine = setup_test_engine();
        let results = engine.search("Computador");
        assert_eq!(results.len(), 3); // Dell, HP, Apple
    }

    #[test]
    fn test_recommendations() {
        let engine = setup_test_engine();
        let p = &engine.products[0]; // Dell Computador
        let recs = engine.get_recs(p);
        
        assert!(!recs.is_empty());
        let tem_computador = recs.iter().any(|(r, _)| r.category_id == p.category_id);
        let tem_mesma_marca = recs.iter().any(|(r, _)| r.brand_id == p.brand_id);
        
        assert!(tem_computador || tem_mesma_marca);
    }

    #[test]
    fn test_graph_integrity() {
        let engine = setup_test_engine();
        for p in &engine.products {
            let brand = engine.pool.resolve(p.brand_id);
            let category = engine.pool.resolve(p.category_id);
            
            let found_by_brand = engine.search(brand);
            let found_by_cat = engine.search(category);
            
            assert!(found_by_brand.iter().any(|res| res.id == p.id), "Produto {} não encontrado pela marca", p.id);
            assert!(found_by_cat.iter().any(|res| res.id == p.id), "Produto {} não encontrado pela categoria", p.id);
        }
    }
}
