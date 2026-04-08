# Documentação Técnica: Arquitetura em Silos de Mercado MegaStore

## Estrutura do Motor de Grafos
O sistema evoluiu para uma arquitetura de grafos enriquecida, onde os dados são particionados em silos virtuais de mercado (**Tecnologia, Moda, Beleza**). Isso garante que recomendações contextuais sejam altamente pertinentes para o usuário final.

### Modelagem de Dados
Cada produto contém uma metatag `market_group`, que atua como um filtro rigoroso durante o cálculo de vizinhança no grafo:
- **Tecnologia:** Inclui dispositivos, acessórios, notebooks e periféricos.
- **Moda:** Inclui calçados, vestuário e acessórios têxteis.
- **Beleza:** Inclui dermocosméticos, higiene e perfumaria.

## Lógica de Recomendação (Context-Aware)
O algoritmo de recomendação (`get_recs`) percorre os vizinhos do grafo (produtos que compartilham atributos), mas só aceita o item como sugestão se o `market_group` do item relacionado for idêntico ao do item original. Isso elimina a ocorrência de "pontes cruzadas" causadas por cores ou adjetivos genéricos (ex: Tênis Branco e MacBook Branco).

### Indexação de Busca Inteligente
O sistema utiliza uma indexação de multicamadas para garantir que o usuário encontre o que precisa, independentemente da variação linguística:
1.  **Indexação de Categoria Inteira:** Match exato (ex: "Smartphone Celular").
2.  **Indexação por Palavras-Chave:** Cada palavra da categoria é um nó no grafo (ex: "Smartphone" e "Celular" individualmente).
3.  **Normalização Singular/Plural:** Para categorias comuns como "Roupas", o motor indexa automaticamente o termo singular ("Roupa"), permitindo que ambos os termos tragam resultados.
4.  **Filtro de Stop-words:** Termos genéricos que poderiam criar falsas conexões no grafo são ignorados durante a indexação de nomes de produtos.

## Performance em Larga Escala
Mesmo com 5.000 itens reais e uma lógica de filtragem complexa, a latência de busca permanece estável abaixo dos **50ms**. O custo de RAM é otimizado através do uso de `StringPool` para centralizar a memória de strings repetitivas (como nomes de marcas e categorias).
