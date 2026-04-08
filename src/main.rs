mod models;
mod engine;
mod utils;

use std::fs;
use std::io::{self, Write};
use std::time::Instant;
use engine::MegaStoreSearch;
use utils::generate_demo_data;

fn main() -> io::Result<()> {
    let data_path = "products.bin";
    
    // Gera dados se não existirem (ou recria para o teste)
    if !fs::metadata(data_path).is_ok() {
        generate_demo_data(data_path)?;
    }

    let engine = MegaStoreSearch::init(data_path)?;

    println!("\n==========================================");
    println!("          MEGASTORE: MOTOR DE GRAFO       ");
    println!("==========================================");

    loop {
        print!("\nBusca [Marca ou Categoria]> ");
        io::stdout().flush()?;
        let mut q = String::new();
        io::stdin().read_line(&mut q)?;
        let q = q.trim();
        if q == "sair" || q.is_empty() { break; }

        let start = Instant::now();
        let results = engine.search(q);
        
        if results.is_empty() {
            println!("Nenhum resultado.");
        } else {
            for p in results.iter().take(3) {
                let marca = engine.pool.resolve(p.brand_id);
                let categoria = engine.pool.resolve(p.category_id);
                println!("\n[ID: {:04}] {:<35} | R$ {:>8.2}", 
                         p.id, p.name, p.price);
                println!("   Categoria: {:<12} | Marca: {}", categoria, marca);
                
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
