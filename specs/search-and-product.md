# Especificação 1: Motor de Busca e Grafo

## Objetivo
O sistema deve ler dados da pasta `data/`, carregar os produtos em memória e permitir buscas ultra-rápidas utilizando uma estrutura de grafo.

## Fluxo de Dados
1.  **Origem:** Os dados devem ser lidos de arquivos binários localizados na pasta `/data`.
2.  **Memória:** Todos os produtos e o grafo de relações devem ser mantidos em RAM para garantir latência mínima.
3.  **Grafo:** O grafo deve permitir buscas por termos (tokens) e travessia entre nós.

## Estrutura de Conectividade
O grafo não deve ser apenas bipartido (Produto-Termo). Ele deve suportar:
-   **Arestas Produto -> Termo:** Para indexação e categorização.
-   **Arestas Produto -> Produto:** Para conexões diretas de similaridade, pacotes de produtos (bundles) ou histórico de compras relacionadas.

---

# Especificação 2: Definição de Produto e Nós

## Estrutura do Produto
Cada produto deve conter:
-   `id`: Identificador único numérico.
-   `name`: Nome descritivo (String).
-   `brand_id`: ID da marca (referência ao pool de strings).
-   `category_id`: ID da categoria (referência ao pool de strings).
-   `price`: Valor monetário.

## Estrutura de Nós (Graph Nodes)
Os nós no grafo podem representar:
1.  **Entidade Produto:** Contém o ID do produto.
2.  **Atributo/Termo:** Contém IDs de palavras-chave, marcas ou categorias.

## Regras de Arestas
-   Um nó de **Produto** deve se conectar a nós de **Termo** que o descrevem.
-   Um nó de **Produto** pode ter arestas diretas com outros nós de **Produto** quando houver uma relação de "Frequentemente Comprados Juntos" ou "Substitutos Diretos".
