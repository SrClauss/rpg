# Regras de Negócio e Decisões de Arquitetura

## Arquitetura de Geração Procedural do Grid via LLM (Function Calling / Structured Output)

### 1. Como funciona a Criação do Grid pelo LLM
Quando a premissa da aventura é definida pelos jogadores (ex: *"Entramos na cripta infestada de cultistas"*), o servidor em Rust invoca o LLM exigindo uma chamada de ferramenta (**Function Calling**) estritamente tipada via JSON Schema.

### 2. Definição da Tool / Schema (`generate_battle_grid`)

O schema JSON exposto para o LLM instrui o modelo a construir o mapa e posicionar os inimigos e obstáculos:

```json
{
  "name": "generate_battle_grid",
  "description": "Gera um mapa de combate tático baseado na descrição narrativa da cena.",
  "parameters": {
    "type": "object",
    "properties": {
      "width": { "type": "integer", "description": "Largura do grid (ex: 10 a 20 células)" },
      "height": { "type": "integer", "description": "Altura do grid (ex: 10 a 20 células)" },
      "theme": { "type": "string", "description": "Tema visual do mapa (ex: 'crypt', 'forest', 'tavern')" },
      "obstacles": {
        "type": "array",
        "items": {
          "type": "object",
          "properties": {
            "x": { "type": "integer" },
            "y": { "type": "integer" },
            "terrain_type": { 
              "type": "string", 
              "enum": ["Obstacle", "Difficult", "CoverHalf", "CoverThreeQuarters"] 
            },
            "description": { "type": "string", "description": "Ex: 'Pilar de pedra antigo', 'Mesa de madeira quebrada'" }
          },
          "required": ["x", "y", "terrain_type"]
        }
      },
      "spawn_points_players": {
        "type": "array",
        "description": "Coordenadas seguras onde os jogadores iniciam a batalha",
        "items": {
          "type": "object",
          "properties": { "x": { "type": "integer" }, "y": { "type": "integer" } }
        }
      },
      "enemies": {
        "type": "array",
        "items": {
          "type": "object",
          "properties": {
            "name": { "type": "string", "description": "Ex: 'Goblin Xamã'" },
            "monster_statblock": { "type": "string", "description": "ID da ficha de D&D 5e (ex: 'goblin_shaman')" },
            "position": {
              "type": "object",
              "properties": { "x": { "type": "integer" }, "y": { "type": "integer" } }
            }
          },
          "required": ["name", "monster_statblock", "position"]
        }
      }
    },
    "required": ["width", "height", "theme", "obstacles", "spawn_points_players", "enemies"]
  }
}
```

### 3. Fluxo de Validação e Construção no Rust Backend
1. **O LLM retorna a Function Call**: O LLM envia o payload JSON acima propondo a disposição do mapa.
2. **O Rust Valida e Monta o `BattleGrid`**:
   - O servidor Rust faz o parse via `serde_json` para structs Rust.
   - **Validação de Regras (Sanity Check)**: O Rust garante que nenhum inimigo ou obstáculo foi colocado na mesma coordenada `(x, y)` dos jogadores e que o mapa possui um caminho válido.
3. **Instanciação na Memória**:
   - O Rust cria o `BattleGrid` na memória e posiciona os tokens dos jogadores e monstros nas posições indicadas.
4. **Broadcast para os Jogadores (WebSocket)**:
   - Envia o mapa completo gerado pela IA para a tela de todos os clientes.
