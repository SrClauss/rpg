# Plano de Implementação: Arquitetura de Subclasses & Subclasse Assassin (Homebrew/Custom Data-Driven)

## Objetivo
Criar o sistema de **Subclasses** em Rust (`Subclass`, `SubclassFeature`, `FeatureEffect`), integrado com `Entity` e serializável via `serde` (JSON/YAML/SurrealDB), e testar a criação da subclasse **Assassin** (PHB Rogue Archetype).

## Estruturas Principais
- `SubclassFeature`: Representa uma habilidade concedida pela subclasse em determinado nível (ex: Nível 3 - Assassinate / Assassinar).
- `Subclass`: Guarda `id`, `name`, `parent_class`, `unlock_level` e a lista de `features`.
- Integração em `Entity`: Adicionar o campo `subclass: Option<Subclass>` e métodos `set_subclass` e `active_subclass_features`.

## Checklist de Tarefas
- [x] **Subtarefa 1: Implementação das structs `Subclass`, `SubclassFeature`, integração em `Entity` e testes com a subclasse Assassin**
  - Criar `src/rules/character/subclass.rs` com `Subclass`, `SubclassFeature` e construtor helper `create_assassin_subclass()`.
  - Atualizar `src/rules/character/mod.rs` e `src/rules/character/entity.rs` para aceitar `subclass: Option<Subclass>`.
  - Escrever testes unitários testando serialização JSON e associação do Assassin ao Rogue no nível 3.
