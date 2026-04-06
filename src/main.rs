use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, BufWriter, Write, Read};
use std::time::Instant;
use rand::RngExt;

// --- Dicionário de Termos (Otimização Principal) ---
#[derive(Default, Serialize, Deserialize)]
struct StringPool {
    forward: HashMap<String, u32>,
    reverse: HashMap<u32, String>,
    next_id: u32,
}

impl StringPool {
    fn get_or_intern(&mut self, s: &str) -> u32 {
        let s = s.to_lowercase();
        if let Some(&id) = self.forward.get(&s) { return id; }
        let id = self.next_id;
        self.forward.insert(s.clone(), id);
        self.reverse.insert(id, s);
        self.next_id += 1;
        id
    }
    fn resolve(&self, id: u32) -> &str {
        self.reverse.get(&id).map(|s| s.as_str()).unwrap_or("Desconhecido")
    }
}

// --- Estrutura Otimizada (Usa IDs em vez de Strings repetidas) ---
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Product {
    id: u32,
    name: String,       // Nome permanece string por ser único
    brand_id: u32,      // Otimizado: ID numérico
    category_id: u32,   // Otimizado: ID numérico
    price: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum NodeType {
    Product(u32),
    Term(u32), // Grafo agora usa apenas IDs numéricos
}

struct MegaStoreSearch {
    products: Vec<Product>,
    pool: StringPool,
    graph: Graph<NodeType, (), Undirected>,
    term_to_node: HashMap<u32, NodeIndex>,
    product_to_node: HashMap<u32, NodeIndex>,
}

impl MegaStoreSearch {
    fn init(data_path: &str) -> io::Result<Self> {
        let mut products = Vec::new();
        let pool = StringPool::default(); // Removido o 'mut'
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
        println!("--- Otimização: Mapeando termos em inteiros... ---");
        // Adiciona marcas e categorias ao pool durante a construção
        let products_copy = self.products.clone();
        for p in &products_copy {
            let p_node = self.graph.add_node(NodeType::Product(p.id));
            self.product_to_node.insert(p.id, p_node);

            // Linkar IDs numéricos no grafo
            self.link_id(p_node, p.brand_id);
            self.link_id(p_node, p.category_id);
            
            // Indexar palavras do nome também como IDs
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

    fn search(&self, query: &str) -> Vec<&Product> {
        let query_lc = query.to_lowercase();
        // Converte a busca em ID primeiro (Muito rápido!)
        if let Some(&term_id) = self.pool.forward.get(&query_lc) {
            if let Some(&t_node) = self.term_to_node.get(&term_id) {
                return self.graph.neighbors(t_node)
                    .filter_map(|n| {
                        if let NodeType::Product(id) = self.graph[n] {
                            self.products.iter().find(|p| p.id == id)
                        } else { None }
                    })
                    .collect();
            }
        }
        vec![]
    }

    fn get_recs(&self, p: &Product) -> Vec<(&Product, &str)> {
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

fn generate_demo_data(path: &str) -> io::Result<()> {
    println!("--- Gerador: Criando base com Dicionário de Termos ---");
    let mut pool = StringPool::default();
    let cats = ["Notebook", "Monitor", "Mouse", "Teclado", "Cadeira"];
    let brands = ["Dell", "Samsung", "Logitech", "Razer", "Apple"];
    
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let mut rng = rand::rng();

    for i in 0..3000 {
        let b_str = brands[rng.random_range(0..5)];
        let c_str = cats[rng.random_range(0..5)];
        
        let product = Product {
            id: i as u32,
            name: format!("{} {} Otimizado", b_str, c_str),
            brand_id: pool.get_or_intern(b_str),
            category_id: pool.get_or_intern(c_str),
            price: rng.random_range(100.0..5000.0),
        };
        let bytes = bincode::serialize(&product).unwrap();
        writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
        writer.write_all(&bytes)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let data_path = "products.bin";
    let _ = fs::remove_file(data_path);
    generate_demo_data(data_path)?;

    let engine = MegaStoreSearch::init(data_path)?;

    println!("\n==========================================");
    println!("    MEGASTORE: MOTOR GRAFO OTIMIZADO      ");
    println!("     (Dicionário de Termos Ativado)       ");
    println!("==========================================");

    loop {
        print!("\nBusca [Marca ou Categoria]> ");
        io::stdout().flush()?;
        let mut q = String::new();
        io::stdin().read_line(&mut q)?;
        let q = q.trim();
        if q == "sair" { break; }

        let start = Instant::now();
        let results = engine.search(q);
        
        if results.is_empty() {
            println!("Nenhum resultado.");
        } else {
            for p in results.iter().take(3) {
                let marca = engine.pool.resolve(p.brand_id);
                println!("\n[ID: {:04}] {:<25} | R$ {:>8.2} | Marca: {}", 
                         p.id, p.name, p.price, marca);
                
                for (r, tipo) in engine.get_recs(p) {
                    println!("   -> {:<12}: [ID: {:04}] {:<20} | R$ {:>8.2}", 
                             format!("[{}]", tipo), r.id, r.name, r.price);
                }
            }
            println!("\nBusca concluída em: {:?}", start.elapsed());
        }
    }
    Ok(())
}
