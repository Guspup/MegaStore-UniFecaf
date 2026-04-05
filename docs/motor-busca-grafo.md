# Plano de Implementação: Motor de Busca em Grafo (Escala 1,5 Mi Itens - Nível Industrial)

## Objetivo
Desenvolver um motor de busca de extrema performance em Rust para a "MegaStore", capaz de indexar e pesquisar entre 1.500.000+ produtos usando grafos, índices invertidos e persistência binária.

## Arquitetura de Big Data
Para suportar 1,5 milhão de itens com eficiência:

1. **Persistência Binária (mmap):** Arquivos binários estruturados para acesso aleatório rápido sem carregar tudo na RAM.
2. **Índice Invertido:** Mapeamento de termos de busca (Tokens) para IDs de produtos para busca O(1).
3. **Grafo de Conexões Compacto:**
   - Representação do grafo usando IDs numéricos (`u32`) em vez de Strings para economizar memória.
   - Uso de `FixedBitSet` ou estruturas similares para representar relacionamentos em massa.
4. **Gerenciamento de Memória:** Carregamento seletivo de metadados críticos.

## Passos de Implementação

1. **Dependências Industriais:** `petgraph`, `serde`, `bincode`, `memmap2`, `tantivy` (opcional, para inspiração em indexação).
2. **Sistema de Tokenização:** Analisador léxico para extrair termos de busca dos 1,5 mi produtos.
3. **Pipeline de Indexação:**
   - Função para processar a massa de dados e gerar o arquivo binário.
   - Geração do Índice Invertido em disco.
4. **Motor de Busca de Baixa Latência:**
   - Algoritmo que recebe a query, consulta o índice e utiliza o grafo para enriquecer os resultados com recomendações.
5. **Teste de Estresse:** Geração e busca em uma base de 1,5 milhão de itens fictícios.

## Verificação
- Latência de busca (objetivo: < 50ms).
- Uso de memória (objetivo: < 1GB de RAM para 1,5 mi itens).
