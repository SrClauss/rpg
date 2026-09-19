# Plano de Implementação: Regras SRD 5.1 Páginas 56 a 61 (Evolução, Multiclasse, Alinhamento, Idiomas e Inspiração)

## Objetivo
Implementar em Rust as regras do capítulo "Beyond 1st Level" do SRD 5.1 (Páginas 56 a 61), abrangendo o sistema de Alinhamentos, Idiomas, Inspiração, Tabela de Experiência (XP), Evolução de Nível (`level_up`) e Regras de Multiclasse (Pré-requisitos e Conjuração Multiclasse).

## Checklist de Subtarefas

- [x] **Subtarefa 1: Implementação de Alinhamento (`Alignment`), Idiomas (`Language`), Inspiração (`Inspiration`) e vinculação com `Entity`**
  - Criar `src/rules/character/alignment.rs` com os enums `Ethics`, `Morality` e a struct `Alignment`.
  - Criar `src/rules/character/language.rs` com o enum `Language` (Comum, Élfico, Anão, etc.).
  - Adicionar campos `alignment: Alignment`, `languages: Vec<Language>` e `has_inspiration: bool` em `Entity`.
  - Criar métodos `grant_inspiration()` e `use_inspiration()` em `Entity`.
  - Escrever testes unitários para alinhamento, idiomas e inspiração.

- [ ] **Subtarefa 2: Tabela de XP, Progresso de Nível (`level_up`) e Ganho de Vida**
  - Criar `src/rules/character/advancement.rs` com a tabela oficial de XP (níveis 1 a 20).
  - Adicionar campo `xp: u32` na `Entity`.
  - Implementar método `level_up(&mut self, chosen_class: ClassVariant, hp_gain: i32)` em `Entity`.
  - Escrever testes unitários verificando avanço de nível, aumento de HP e atualização de bônus de proficiência.

- [ ] **Subtarefa 3: Regras de Multiclasse (Pré-requisitos e Conjuração Multiclasse)**
  - Implementar função `can_multiclass_into(entity: &Entity, target_class: &ClassVariant) -> Result<(), String>` validando mínimo 13 no atributo exigido.
  - Implementar tabela e cálculo de `SpellSlots` para conjuradores multiclasse (Página 58).
  - Escrever testes unitários para validação de multiclasse e cálculo de slots.

## Subtarefa Atual
**Subtarefa 1: Implementação de Alinhamento (`Alignment`), Idiomas (`Language`), Inspiração (`Inspiration`) e vinculação com `Entity`**
