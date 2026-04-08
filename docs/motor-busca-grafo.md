# Documentação Técnica: Motor de Busca em Grafo MegaStore

## Visão Geral
Esta documentação detalha os aspectos técnicos da implementação do motor de busca para a MegaStore, focado em uma base de **5.000 itens**.

## Estrutura do Grafo
O sistema utiliza um **Grafo Bipartido** implícito:
- **Nós de Produto:** Representam a entidade final.
- **Nós de Termo:** Representam atributos (Marca, Categoria, Palavras do Nome).

As arestas conectam Produtos a Termos. Não existem conexões diretas entre dois produtos ou entre dois termos, o que simplifica a travessia.

## Fluxo de Recomendação
A lógica de recomendação segue o algoritmo de vizinhança:
1. Identificar todos os nós `T` (termos) conectados ao produto `P`.
2. Para cada `T`, identificar todos os produtos `P'` conectados.
3. Classificar `P'` como:
   - **Concorrente:** Se `P'.category_id == P.category_id`.
   - **Complemento:** Se `P'.brand_id == P.brand_id` mas categoria diferente.

## Otimizações de Memória
A principal otimização é o **String Pooling**. Em vez de cada objeto `Product` carregar strings pesadas para "Marca" e "Categoria", ele carrega apenas um `u32`. A tradução para texto ocorre apenas na camada de exibição (UI).

## Resultados de Testes
- **Tempo médio de busca:** 0.05ms a 0.15ms.
- **Tempo de construção do grafo:** ~1.2s para 5.000 itens.
- **Consumo de RAM estimado:** < 50MB.
