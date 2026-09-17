# Plano de Implementação: Mapeamento Completo de Terrenos nos Metadados da Imagem

## Objetivo
Mapear todos os tipos de terrenos da imagem gerada através da análise do DeepSeek e gravar nos metadados (`_metadata.json` e SurrealDB) a lista de todos os hexágonos `(q, r)` correspondentes.

## Checklist de Subtarefas

- [x] **Subtarefa 1: Atualizar o Schema e Prompts do DeepSeek Visão (`src/llm.rs`)**
  - Arquivo: `src/llm.rs`
  - Ação: Ajustado `analyze_map_vision` para instruir o DeepSeek a classificar exaustivamente todos os elementos e tipos de terrenos por regiões de hexágonos `(q, r)`.


- [x] **Subtarefa 2: Persistir Mapeamento Completo nos Metadados (`src/gemini.rs`)**
  - Arquivo: `src/gemini.rs`
  - Ação: Atualizado `save_map_regions` para gravar o objeto de regiões de terrenos completo tanto no JSON `_metadata.json` quanto no SurrealDB.


- [x] **Subtarefa 3: Conectar e Validar Pipeline no `main.rs` (`src/main.rs`)**
  - Arquivo: `src/main.rs`
  - Ação: Orquestrado o pipeline completo. O DeepSeek Visão mapeou exaustivamente todos os terrenos (`viga_de_madeira_caida`, `mesa_central`, `cadeiras_quebradas`, `chao_limpo`, `escombros`, `paredes`) e suas listas de hexágonos `(q, r)` correspondentes, gravando 100% dos dados nos metadados e no SurrealDB!

