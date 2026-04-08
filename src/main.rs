mod models;
mod engine;
mod utils;

use std::fs;
use std::io::{self, Write};
use std::time::Instant;
use engine::MegaStoreSearch;
use utils::generate_demo_data;

fn main() -> io::Result<()> {
    let data_path = "data/products.bin";
    
    if !fs::metadata(data_path).is_ok() {
        fs::create_dir_all("data")?;
        generate_demo_data(data_path)?;
    }

    let engine = MegaStoreSearch::init(data_path)?;

    // Cores ANSI
    let blue = "\x1b[34m";
    let green = "\x1b[32m";
    let yellow = "\x1b[33m";
    let cyan = "\x1b[36m";
    let reset = "\x1b[0m";
    let bold = "\x1b[1m";

    println!("\n{}{}{}", bold, blue, "==========================================");
    println!("          MEGASTORE: MOTOR DE GRAFO       ");
    println!("=========================================={}", reset);
    println!("{}Dica: Busque por 'celular', 'nike', 'apple' ou 'computador'{}", yellow, reset);

    loop {
        print!("\n{}Busca [Marca ou Categoria]>{} ", bold, reset);
        io::stdout().flush()?;
        
        let mut q = String::new();
        io::stdin().read_line(&mut q)?;
        let q = q.trim();

        if q == "sair" { break; }
        if q.is_empty() { continue; } // Não sai em linha vazia

        let start = Instant::now();
        let results = engine.search(q);
        
        if results.is_empty() {
            println!("{}Nenhum resultado encontrado para '{}'.{}", yellow, q, reset);
        } else {
            for p in results.iter().take(3) {
                println!("\n{}[ID: {:04}] {:<35} | {}{}{}R$ {:>8.2}{}", 
                         cyan, p.id, p.name, reset, green, bold, p.price, reset);
                println!("   {}Categoria:{} {:<18} | {}Marca:{} {}", 
                         blue, reset, p.category, blue, reset, p.brand);
                
                for (r, tipo) in engine.get_recs(p) {
                    let cor_tipo = if tipo == "Concorrente" { "\x1b[31m" } else { "\x1b[35m" };
                    println!("   {}-> {:<12}{} [ID: {:04}] {:<25} | R$ {:>8.2}", 
                             cor_tipo, format!("[{}]", tipo), reset, r.id, r.name, r.price);
                }
            }
            println!("\n{}Busca concluída em: {:?}{}", yellow, start.elapsed(), reset);
        }
    }
    Ok(())
}
