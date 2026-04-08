# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do Projeto
O MegaStore Search é um motor de busca de alta performance desenvolvido em Rust, projetado para gerenciar catálogos de produtos utilizando estruturas de grafos e dicionários de termos (String Pooling). O sistema permite buscas instantâneas por marcas, categorias ou palavras-chave, além de gerar recomendações inteligentes (concorrentes e complementos) baseadas na conectividade entre os produtos.

### Funcionalidades Principais
*   **Busca em Grafo:** Relacionamentos complexos entre produtos e atributos.
*   **Recomendações Dinâmicas:** Identificação de produtos similares via vizinhança no grafo.
*   **Dicionário de Termos (Interning):** Redução drástica do uso de memória ao mapear strings repetitivas para IDs numéricos.
*   **Deduplicação de Resultados:** Garantia de resultados únicos mesmo com múltiplas conexões.
*   **Persistência Binária:** Carregamento eficiente de dados via serialização `bincode`.

## Tecnologias Utilizadas
*   **Linguagem:** [Rust](https://www.rust-lang.org/) (Edição 2024).
*   **Crates (Bibliotecas):**
    *   `petgraph`: Gerenciamento e travessia da estrutura de grafos.
    *   `serde` & `bincode`: Serialização binária de alta performance.
    *   `rand`: Geração de dados sintéticos para testes.
    *   `hashbrown` (via std): Tabelas Hash otimizadas para mapeamento de termos.

## Instruções de Execução
Para compilar e rodar o sistema, certifique-se de ter o Rust instalado.

1.  **Gerar base de dados e iniciar busca:**
    ```bash
    cargo run
    ```
    *O sistema detectará a ausência do arquivo `products.bin` e gerará automaticamente 5.000 itens de demonstração.*

2.  **Sair do sistema:** Digite `sair` ou pressione `Ctrl+C`.

## Instruções de Teste
O projeto possui uma suíte de testes unitários que valida a integridade do grafo e a precisão da busca.

1.  **Executar todos os testes:**
    ```bash
    cargo test
    ```

## Exemplos de Uso
Após iniciar o sistema, você pode realizar consultas como:
*   **Busca por Marca:** Digite `Dell` ou `Samsung`.
*   **Busca por Categoria:** Digite `Notebook` ou `Monitor`.
*   **Busca por Palavra-chave:** Digite `Otimizado`.

O sistema retornará os 3 primeiros produtos encontrados e, para cada um, sugerirá até 3 recomendações (Ex: "[Concorrente]" para mesma categoria ou "[Complemento]" para mesma marca).

## Arquitetura do Sistema
O projeto é dividido em módulos para facilitar a manutenção:
*   `src/models.rs`: Definições de estruturas de dados (`Product`, `StringPool`).
*   `src/engine.rs`: O "coração" do sistema, contendo o Grafo e algoritmos de busca.
*   `src/utils.rs`: Ferramentas auxiliares para geração de dados sintéticos.
*   `src/main.rs`: Interface de linha de comando (CLI).

## Algoritmos e Estruturas de Dados
O sistema prioriza a performance O(1) e O(k) através de:
*   **Tabelas Hash (HashMap/HashSet):** Utilizadas no `StringPool` para tradução instantânea de termos em IDs e na deduplicação de resultados de busca.
*   **Grafo Não-Direcionado:** Implementado via `petgraph`, onde produtos e termos são nós. As arestas representam o pertencimento de um produto a um atributo.
*   **Busca por Vizinhos:** As recomendações são calculadas encontrando "vizinhos de vizinhos" no grafo, permitindo descobrir relações sem percorrer toda a lista de produtos.

## Desempenho e Escalabilidade
*   **Escalabilidade:** O sistema foi validado com 5.000 itens, mantendo latência de busca inferior a **1ms**.
*   **Memória:** Graças ao `StringPool`, strings repetitivas (como a marca "Dell") são armazenadas uma única vez, reduzindo o consumo de RAM.
*   **Complexidade:** A busca por termos é O(1) no acesso ao índice e proporcional ao número de resultados no grafo, garantindo estabilidade mesmo com o crescimento do catálogo.

## Contribuições
Este é um projeto de cunho acadêmico/experimental. Para contribuir:
1.  Faça um Fork do projeto.
2.  Crie uma Branch para sua feature (`git checkout -b feature/nova-feature`).
3.  Abra um Pull Request para revisão.

## Licença
Este projeto está sob a licença MIT. Consulte o arquivo LICENSE para mais detalhes.
