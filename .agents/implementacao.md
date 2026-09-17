# Plano de Implementação: Passo 3 (O Motor de Regras: Combate e Testes)

## Objetivo
Implementar o Passo 3 do Roadmap do D&D 5e: construir o motor de regras de combate e testes, cobrindo Testes de Habilidade e Resistência, Sistema de Condições, Resolução de Ataques com Resistências/Vulnerabilidades e Gerenciador de Iniciativa e Turnos.

## Checklist de Subproblemas
- [x] **Subproblema 1: Testes de Habilidade e Resistência (`src/rules/checks.rs`)**
  - Implementar estrutura e métodos para Testes de Habilidade (Ability Checks) e Salvaguardas (Saving Throws).
  - Integrar com o sistema de dados (`Roller`) aceitando `AdvantageState`.
  - Aplicar modificador de habilidade e bônus de proficiência quando aplicável.
- [x] **Subproblema 2: Sistema de Condições (`src/rules/conditions.rs`)**
  - Modelar o enum `Condition` com as condições do SRD (Cego, Caído, Envenenado, Incapacitado, etc.).
  - Adicionar suporte a condições ativas na `Entity`.
- [x] **Subproblema 3: Resolução de Ataques e Dano (`src/rules/combat/attack.rs`)**
  - Implementar a mecânica de ataque (`1d20 + Mod + Prof` vs `CA`).
  - Tratar Acerto Crítico (20 natural) e Falha Crítica (1 natural).
  - Calcular dano (dobrar dados no crítico) e aplicar Resistências, Imunidades e Vulnerabilidades a tipos de dano.
- [x] **Subproblema 4: Iniciativa e Ordem de Turnos (`src/rules/combat/initiative.rs`)**
  - Implementar a rolagem de iniciativa (`1d20 + Mod. DES`) para combatentes.
  - Ordenar a fila de iniciativa e gerenciar avanços de turno e rodadas.

## Ponto de Partida
**Sugiro começarmos pelo Subproblema 1 (Testes de Habilidade e Resistência)**, pois ele estabelece a mecânica fundamental de rolagens com modificadores e proficiências que alimentará tanto os testes genéricos quanto os testes de salvaguarda de combate.
