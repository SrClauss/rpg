# Plano de Implementação: Registro Global Estático de Equipamentos (`LazyLock`) e Ataques Especiais de Armas

## Objetivo
Implementar em Rust um registro global de equipamentos baseado em `std::sync::LazyLock` (`EquipmentRegistry`), pré-carregando estaticamente todas as 37 armas e 13 armaduras do SRD 5.1 e permitindo busca direta por ID e injeção de itens Homebrew JSON. Em seguida, implementar a resolução de **Ataques Especiais de Armas** (Lance, Net, etc.) para que a IA DM e o motor de jogo apliquem os efeitos mecânicos durante o combate.

## Checklist de Subtarefas

- [x] **Subtarefa 1: Implementação do `EquipmentRegistry` com `LazyLock` global e catálogo SRD/Homebrew**
  - Criar `src/rules/equipment/registry.rs` com a constante global `GLOBAL_EQUIPMENT: LazyLock<EquipmentRegistry>`.
  - Implementar métodos estáticos `EquipmentRegistry::get_weapon(id)`, `EquipmentRegistry::get_armor(id)` e `EquipmentRegistry::load_json(json_str)`.
  - Exportar o módulo em `src/rules/equipment/mod.rs`.
  - Escrever testes unitários validando busca global por ID e carregamento de equipamentos homebrew.

- [ ] **Subtarefa 2: Resolução de Ataques Especiais de Armas (Lance, Net) no Motor de Combate**
  - Atualizar `src/rules/combat/attack.rs` com `resolve_attack_with_context` considerando distância em pés e montaria.
  - Implementar o efeito da Lance (Desvantagem a 5 pés) e da Rede (Aplica condição `Restrained`).
  - Escrever testes unitários em `attack.rs` validando ataques de Lance e Rede.

## Subtarefa Atual
**Subtarefa 1: Implementação do `EquipmentRegistry` com `LazyLock` global e catálogo SRD/Homebrew**
