# Plano de Implementação: Gerador de BattleGrid em Rust com Function Calling (DeepSeek/LLM)

## Objetivo
Criar o módulo em Rust responsável por definir a estrutura do **BattleGrid** (grid, tokens do grupo, monstros, obstáculos e terrenos) e conectar ao **LLM (DeepSeek)** via **Function Calling / Tool Use** para gerar proceduralmente tabuleiros de combate a partir de premissas narrativas.

---

## Checklist de Subproblemas

- [x] **Subproblema 1: Estruturação dos Tipos e Montador do BattleGrid em Rust (`src/grid.rs`)**
  - Configurar projeto Cargo com `serde`, `serde_json`, `tokio`, `reqwest` e `dotenvy`.
  - Criar structs para `Position`, `TerrainType`, `Obstacle`, `Token` (Heroi/Monstro) e `BattleGrid`.
  - Criar métodos de inicialização, validação de limites e renderização ASCII para debug no terminal (`fmt::Display`).

- [x] **Subproblema 2: Cliente de LLM DeepSeek com Function Calling em Rust (`src/llm.rs`)**
  - Configurar chamada HTTP assíncrona (`reqwest` + `tokio`) apontando para `https://api.deepseek.com`.
  - Definir a Tool `generate_battle_grid` usando `serde_json`.
  - Tratar a resposta da IA e desserializar a chamada de ferramenta para a struct `BattleGrid`.

- [x] **Subproblema 3: CLI de Teste de Integração (`src/main.rs`)**
  - Carregar a chave de API do arquivo `.env`.
  - Solicitar ao LLM a geração de um mapa com base em uma premissa (ex: *"O grupo de 2 heróis entra em uma emboscada em uma ponte de madeira em ruínas sobre um desfiladeiro. Dois goblins atacam do outro lado da ponte."*).
  - Imprimir o `BattleGrid` gerado no terminal (renderização ASCII e dump do JSON).

---

## Conclusão da Fase 1
O gerador de mapas de combate táticos via Function Calling com DeepSeek LLM em Rust está 100% funcional e integrado!
