use std::fs::File;
use std::io::{self, BufWriter, Write};
use rand::RngExt;
use crate::models::{Product, StringPool};

pub fn generate_demo_data(path: &str) -> io::Result<()> {
    println!("--- Gerador: Criando base de varejo realista (5.000 itens) ---");
    let mut pool = StringPool::default();
    
    // Grupos de Categorias e Marcas Realistas
    let categories = [
        ("Eletrônicos", vec!["Smartphone", "Notebook", "Tablet", "Monitor", "Câmera"]),
        ("Periféricos", vec!["Teclado", "Mouse", "Impressora", "Fone de Ouvido"]),
        ("Beleza", vec!["Shampoo", "Condicionador", "Hidratante", "Perfume"]),
        ("Moda", vec!["Camiseta", "Calça Jeans", "Tênis", "Jaqueta"]),
    ];

    let brands_by_group = [
        ("Eletrônicos", vec!["Apple", "Samsung", "Dell", "LG", "Sony", "Canon"]),
        ("Periféricos", vec!["Logitech", "Razer", "HP", "Epson"]),
        ("Beleza", vec!["Pantene", "Dove", "L'Oreal", "Nivea"]),
        ("Moda", vec!["Nike", "Adidas", "Levi's", "Puma"]),
    ];

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let mut rng = rand::rng();

    for i in 0..5000 {
        // Escolhe um grupo aleatório (Eletrônicos, Beleza, etc)
        let group_idx = rng.random_range(0..categories.len());
        let group_name = categories[group_idx].0;
        
        // Escolhe uma categoria e uma marca dentro desse grupo
        let cat_list = &categories[group_idx].1;
        let brand_list = &brands_by_group[group_idx].1;
        
        let c_str = cat_list[rng.random_range(0..cat_list.len())];
        let b_str = brand_list[rng.random_range(0..brand_list.len())];
        
        // Gera um nome de produto realista
        let name = match c_str {
            "Smartphone" => format!("{} Galaxy S24 Ultra" , b_str),
            "Notebook" => format!("{} Latitude 5000" , b_str),
            "Câmera" => format!("{} Alpha DSLR" , b_str),
            "Impressora" => format!("{} EcoTank Jato de Tinta" , b_str),
            "Shampoo" => format!("{} Hidratação Intensa 400ml" , b_str),
            "Condicionador" => format!("{} Brilho e Maciez" , b_str),
            "Tênis" => format!("{} Esportivo Air Max" , b_str),
            _ => format!("{} {} - Modelo Premium", b_str, c_str),
        };

        let product = Product {
            id: i as u32,
            name,
            brand_id: pool.get_or_intern(b_str),
            category_id: pool.get_or_intern(c_str),
            price: match group_name {
                "Eletrônicos" => rng.random_range(1500.0..15000.0),
                "Periféricos" => rng.random_range(100.0..2500.0),
                "Beleza" => rng.random_range(20.0..350.0),
                "Moda" => rng.random_range(80.0..800.0),
                _ => 100.0,
            },
        };

        let bytes = bincode::serialize(&product).unwrap();
        writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
        writer.write_all(&bytes)?;
    }
    Ok(())
}
