# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do Projeto
O MegaStore Search é um motor de busca de alta performance desenvolvido em Rust, projetado para simular a experiência de grandes varejistas como Amazon ou eBay. O sistema utiliza estruturas de grafos avançadas para gerenciar um catálogo de 5.000 itens reais, oferecendo buscas inteligentes e recomendações contextuais que respeitam silos de mercado (Tecnologia, Moda e Beleza).

### Funcionalidades Principais
*   **Silos de Mercado:** Recomendações inteligentes que não misturam categorias. Tecnologia recomenda eletrônicos, Moda recomenda vestuário e Beleza recomenda cosméticos.
*   **Busca Inteligente:** Entende singulares e plurais (ex: busca por "roupa" encontra a categoria "Roupas") e indexa palavras individuais de marcas e categorias.
*   **Catálogo Ultra-Realista:** Produtos e marcas do mundo real (Apple, Samsung, Nike, L'Oreal) com preços condizentes com o mercado.
*   **Interface Colorida e Responsiva:** CLI intuitiva com cores ANSI para facilitar a leitura de Preços, Marcas e Categorias.
*   **Grafo Bidirecional:** Travessia rápida entre produtos e termos, garantindo que cada conexão seja uma via de mão dupla para descoberta de dados.

## Tecnologias Utilizadas
*   **Linguagem:** [Rust](https://www.rust-lang.org/) (Edição 2024).
*   **Crates (Bibliotecas):**
    *   `petgraph`: Motor de grafos para relacionamentos e vizinhança.
    *   `serde` & `bincode`: Persistência binária ultra-veloz.
    *   `rand`: Geração de dados sintéticos baseados em matrizes reais.

## Instruções de Execução
Para compilar e rodar o sistema, certifique-se de ter o Rust instalado.

1.  **Gerar base de dados e iniciar busca:**
    ```bash
    cargo run
    ```
    *O sistema detectará a ausência do arquivo `data/products.bin` e gerará automaticamente 5.000 itens reais.*

2.  **Sair do sistema:** Digite `sair` ou pressione `Ctrl+C`.

## Instruções de Teste
O projeto possui testes unitários que validam a bidirecionalidade do grafo e a precisão da busca.
```bash
cargo test
```

## Exemplos de Uso
Após iniciar o sistema, você pode realizar consultas como:
*   **Tecnologia:** `iphone`, `macbook`, `samsung`, `computador`.
*   **Moda:** `nike`, `roupa`, `tenis`, `adidas`.
*   **Beleza:** `shampoo`, `loreal`, `perfume`.

O sistema exibirá os resultados e sugerirá itens **[Relacionados]** (mesma categoria) ou **[Complemento]** (mesma marca), sempre respeitando o grupo de mercado do produto consultado.

## Arquitetura do Sistema
*   `src/models.rs`: Estruturas de dados e String Pooling.
*   `src/engine.rs`: Lógica do Grafo, Busca e Filtros de Silo.
*   `src/utils.rs`: Gerador de catálogo realista com matrizes de mercado.
*   `src/main.rs`: Interface CLI responsiva e colorida.

## Algoritmos e Desempenho
*   **Filtro de Stop-words:** O motor ignora termos genéricos (como "Pro", "Ultra", "Modelo") para evitar conexões erradas entre produtos distantes.
*   **Indexação por Termos:** Busca O(1) através de um dicionário de termos mapeado para o Grafo.
*   **Latência:** Buscas concluídas em média entre **5ms e 30ms** para um catálogo de 5.000 itens.
