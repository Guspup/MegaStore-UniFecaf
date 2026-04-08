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
            pool: StringPool::default(),
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

            // 1. Indexar Categoria (com suporte a singular/plural para Roupas)
            let cat_lc = p.category.to_lowercase();
            let c_id = self.pool.get_or_intern(&cat_lc);
            self.link_id(p_node, c_id);
            
            if cat_lc.contains("roupas") {
                let r_id = self.pool.get_or_intern("roupa");
                self.link_id(p_node, r_id);
            }

            for word in cat_lc.split_whitespace() {
                let w_id = self.pool.get_or_intern(word);
                self.link_id(p_node, w_id);
            }

            // 2. Indexar Marca
            let b_id = self.pool.get_or_intern(&p.brand);
            self.link_id(p_node, b_id);
            
            // 3. Indexar Palavras do Nome (Filtradas)
            let stop_words = ["-", "modelo", "premium", "original", "fit", "light", "ii", "iv", "pro", "max", "plus", "edicao", "especial"];
            for word in p.name.split_whitespace() {
                let w_lc = word.to_lowercase();
                if !stop_words.contains(&w_lc.as_str()) && w_lc.len() >= 2 {
                    let w_id = self.pool.get_or_intern(&w_lc);
                    self.link_id(p_node, w_id);
                }
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
        let mut seen_names = HashSet::new();
        seen_names.insert(p.name.clone());

        if let Some(&p_node) = self.product_to_node.get(&p.id) {
            for t_node in self.graph.neighbors(p_node) {
                for sib in self.graph.neighbors(t_node) {
                    if let NodeType::Product(id) = self.graph[sib] {
                        let rel = self.products.iter().find(|pr| pr.id == id).unwrap();
                        
                        // FILTRO DE SILO: Só recomenda se for do mesmo grupo de mercado
                        if rel.market_group == p.market_group && !seen_names.contains(&rel.name) && recs.len() < 3 {
                            seen_names.insert(rel.name.clone());
                            let tipo = if rel.category == p.category { "Relacionados" } else { "Complemento" };
                            recs.push((rel, tipo));
                        }
                    }
                }
            }
        }
        recs
    }
}
