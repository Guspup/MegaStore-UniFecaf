use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use memmap2::Mmap;
use rand::RngExt;

// --- Estruturas de Dados ---

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Product {
    id: u32,
    name: String,
    brand: String,
    category: String,
}

#[derive(Debug, Clone)]
enum NodeType {
    Product(u32), // ID do produto
    Term(String), // Nome, Marca ou Categoria
}

struct MegaStoreSearch {
    graph: Graph<NodeType, (), Undirected>,
    term_to_node: HashMap<String, NodeIndex>,
    products_mmap: Mmap,
    offsets: HashMap<u32, usize>, // Mapeia ID do produto para sua posição no arquivo binário
}

impl MegaStoreSearch {
    fn new(binary_file_path: &str) -> anyhow::Result<Self> {
        let file = File::open(binary_file_path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        Ok(MegaStoreSearch {
            graph: Graph::new_undirected(),
            term_to_node: HashMap::new(),
            products_mmap: mmap,
            offsets: HashMap::new(),
        })
    }

    // Adiciona um termo ao índice do grafo e conecta ao produto
    fn add_relation(&mut self, term: &str, product_node: NodeIndex) {
        let term_lc = term.to_lowercase();
        let term_node = *self.term_to_node.entry(term_lc).or_insert_with(|| {
            self.graph.add_node(NodeType::Term(term.to_string()))
        });
        self.graph.add_edge(product_node, term_node, ());
    }

    // Busca produtos por um termo e retorna os detalhes completos
    fn search(&self, query: &str) -> Vec<Product> {
        let query_lc = query.to_lowercase();
        let mut results = Vec::new();

        if let Some(&node_idx) = self.term_to_node.get(&query_lc) {
            for neighbor in self.graph.neighbors(node_idx) {
                if let NodeType::Product(id) = self.graph[neighbor] {
                    if let Ok(product) = self.get_product_by_id(id) {
                        results.push(product);
                    }
                }
            }
        }
        results
    }

    // Recupera os dados do produto do arquivo binário usando o offset (O(1) de acesso ao disco)
    fn get_product_by_id(&self, id: u32) -> anyhow::Result<Product> {
        let &pos = self.offsets.get(&id).ok_or_else(|| anyhow::anyhow!("ID não encontrado"))?;
        let len = u32::from_le_bytes(self.products_mmap[pos..pos+4].try_into()?) as usize;
        let product: Product = bincode::deserialize(&self.products_mmap[pos+4..pos+4+len])?;
        Ok(product)
    }
}

// --- Gerador de Dados para Teste ---

fn generate_sample_data(path: &str, count: usize) -> anyhow::Result<()> {
    println!("--- Gerador: Criando {} produtos no formato binário ---", count);
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    let brands = vec!["Sony", "Samsung", "Apple", "LG", "Dell", "HP"];
    let categories = vec!["Eletrônicos", "Informática", "Eletrodomésticos", "Games", "Áudio"];
    let mut rng = rand::rng();

    for i in 0..count {
        let product = Product {
            id: i as u32,
            name: format!("Produto Especial {}", i),
            brand: brands[rng.random_range(0..brands.len())].to_string(),
            category: categories[rng.random_range(0..categories.len())].to_string(),
        };
        let bytes = bincode::serialize(&product)?;
        let len = bytes.len() as u32;
        writer.write_all(&len.to_le_bytes())?; // Salva o tamanho para leitura posterior
        writer.write_all(&bytes)?;
    }
    writer.flush()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let data_path = "products.bin";
    let product_count = 200; // Alterado para 200 para o seu teste inicial

    // 1. Forçar a regeneração dos dados para o novo formato/quantidade
    generate_sample_data(data_path, product_count)?;

    // 2. Inicializar o motor
    println!("--- Motor: Inicializando Grafo e Mapeamento de Memória ---");
    let mut engine = MegaStoreSearch::new(data_path)?;

    // 3. Indexar o catálogo (Lê o binário uma vez para montar o Grafo)
    let mut current_pos = 0;
    while current_pos + 4 <= engine.products_mmap.len() {
        let start_of_record = current_pos;
        let len = u32::from_le_bytes(engine.products_mmap[current_pos..current_pos+4].try_into()?) as usize;
        current_pos += 4;
        
        let product: Product = bincode::deserialize(&engine.products_mmap[current_pos..current_pos+len])?;
        
        // Registra o offset para busca rápida posterior
        engine.offsets.insert(product.id, start_of_record);

        // Cria o nó do produto no grafo
        let p_node = engine.graph.add_node(NodeType::Product(product.id));

        // Cria as referências (Arestas)
        engine.add_relation(&product.brand, p_node);
        engine.add_relation(&product.category, p_node);
        for word in product.name.split_whitespace() {
            engine.add_relation(word, p_node);
        }

        current_pos += len;
    }
    println!("Catálogo indexado com sucesso!");

    // 4. Interface de Busca Simples
    let queries = vec!["Sony", "Eletrônicos", "Especial"];
    
    println!("\n--- Teste de Busca MegaStore ---");
    for q in queries {
        let start = Instant::now();
        let results = engine.search(q);
        println!("\nBusca por: '{}' (Levou: {:?})", q, start.elapsed());
        println!("Resultados encontrados: {}", results.len());
        
        // Mostra os primeiros 3 resultados detalhados
        for p in results.iter().take(3) {
            println!("  > [ID: {:03}] {} | Marca: {} | Categoria: {}", p.id, p.name, p.brand, p.category);
        }
    }

    Ok(())
}
