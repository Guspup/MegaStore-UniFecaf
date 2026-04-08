use std::fs::File;
use std::io::{self, BufWriter, Write};
use rand::RngExt;
use crate::models::Product;

pub fn generate_demo_data(path: &str) -> io::Result<()> {
    println!("--- Gerador: Criando catálogo em Silos de Mercado (5.000 itens) ---");
    
    let catalog = [
        // Tecnologia
        ("Apple", "Smartphone Celular", vec!["iPhone 15 Pro Max", "iPhone 15 Pro", "iPhone 15"], "Tecnologia"),
        ("Apple", "Computador Notebook", vec!["MacBook Pro M3 Max", "MacBook Pro M3", "MacBook Air M3"], "Tecnologia"),
        ("Samsung", "Smartphone Celular", vec!["Galaxy S24 Ultra", "Galaxy S24+", "Galaxy S23 FE"], "Tecnologia"),
        ("Samsung", "Monitor", vec!["Odyssey G9", "Odyssey G5", "Smart Monitor M8"], "Tecnologia"),
        ("Motorola", "Smartphone Celular", vec!["Edge 50 Pro", "Edge 40 Neo", "Moto G84 5G"], "Tecnologia"),
        ("Dell", "Computador Notebook", vec!["XPS 13 Plus", "XPS 15", "Inspiron 15 3000"], "Tecnologia"),
        ("Dell", "Computador Desktop", vec!["Alienware Aurora R15", "OptiPlex Micro"], "Tecnologia"),
        ("Sony", "Camera", vec!["Alpha a7 IV", "Alpha a6400", "ZV-E10"], "Tecnologia"),
        ("Sony", "Fone de Ouvido", vec!["WH-1000XM5", "WF-1000XM5", "LinkBuds S"], "Tecnologia"),
        ("Logitech", "Periférico", vec!["Mouse MX Master 3S", "Teclado MX Keys S"], "Tecnologia"),
        // Moda
        ("Nike", "Calçados e Roupas", vec!["Air Jordan 1", "Air Max Pulse", "Dunk Low", "Camiseta Dri-FIT", "Jaqueta Windrunner"], "Moda"),
        ("Adidas", "Calçados e Roupas", vec!["Ultraboost Light", "Stan Smith", "Samba OG", "Jaqueta Tiro"], "Moda"),
        ("Levis", "Calçados e Roupas", vec!["Calça 501 Original Fit", "Jaqueta Jeans Trucker"], "Moda"),
        // Beleza
        ("Pantene", "Produtos de Beleza", vec!["Shampoo Restauração", "Condicionador Brilho", "Máscara Hidratação"], "Beleza"),
        ("L'Oreal", "Produtos de Beleza", vec!["Sérum Revitalift", "Protetor Solar UV Defender", "Creme Hialurônico"], "Beleza"),
    ];

    let variations = ["Preto", "Branco", "Prata", "Azul", "Grafite", "Titanium", "Edição Especial", "Pro", "Plus"];

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let mut rng = rand::rng();

    for i in 0..5000 {
        let (brand, category, models, group) = &catalog[rng.random_range(0..catalog.len())];
        let model = models[rng.random_range(0..models.len())];
        let variant = variations[rng.random_range(0..variations.len())];
        
        let name = format!("{} {} {}", brand, model, variant);
        
        let product = Product {
            id: i as u32,
            name,
            brand: brand.to_string(),
            category: category.to_string(),
            market_group: group.to_string(),
            price: match group.as_ref() {
                "Tecnologia" => rng.random_range(800.0..25000.0),
                "Moda" => rng.random_range(150.0..2500.0),
                "Beleza" => rng.random_range(25.0..350.0),
                _ => 100.0,
            },
        };

        let bytes = bincode::serialize(&product).unwrap();
        writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
        writer.write_all(&bytes)?;
    }
    Ok(())
}
