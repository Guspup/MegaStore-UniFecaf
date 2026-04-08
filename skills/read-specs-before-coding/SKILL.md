---
name: read-specs-before-coding
description: Obriga a leitura de especificações na pasta /specs antes de qualquer codificação ou refatoração. Use sempre que o usuário solicitar uma nova funcionalidade ou correção de bug para garantir que os padrões de arquitetura (como o uso de Grafos e leitura de /data) sejam respeitados.
---

# Instruções de Uso

Sempre que houver uma tarefa de implementação ou modificação de código, siga este fluxo:

1.  **Leitura Obrigatória:** Execute `list_directory` na pasta `/specs`.
2.  **Análise de Requisitos:** Leia o conteúdo de todos os arquivos `.md` em `/specs` para entender as restrições de arquitetura.
3.  **Validação de Contexto:** Verifique se a mudança proposta respeita:
    -   A estrutura de Grafos definida (Nós de Produto e Termos).
    -   A conectividade Produto-Produto (quando aplicável).
    -   A origem de dados na pasta `/data`.
4.  **Codificação:** Somente após entender as especificações, proceda com o desenvolvimento.

## Referências
-   `specs/search-and-product.md`: Define a estrutura do Grafo e os tipos de Arestas.
