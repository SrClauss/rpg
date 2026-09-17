# Plano de Implementação: Passo 1 (Fundação Genérica Completa)

## Objetivo
Finalizar a implementação do Passo 1 do Roadmap (Personagens Básicos), criando a infraestrutura mecânica para Perícias, Raças e Antecedentes, e integrando-os na Entidade principal com cálculos matemáticos corretos (Nível Total, Bônus de Proficiência e Modificadores Raciais).

## Checklist de Subproblemas
- [x] **Subproblema 1: Implementar Perícias e Atributos (`src/rules/character/skills.rs`)**
  - Mapear as 18 perícias oficiais no `enum Skill`.
  - Criar o vínculo de cada perícia ao seu respectivo `Ability` base.
- [x] **Subproblema 2: Implementar Raças Nativas (`src/rules/character/race.rs`)**
  - Criar o `trait Race` e `enum Size`.
  - Estruturar Humanos (+1 em tudo) e Elfos (+2 Destreza) em código puro Rust.
- [x] **Subproblema 3: Implementar Antecedentes (`src/rules/character/background.rs`)**
  - Estruturar proficiências mecânicas (Skills) concedidas por backgrounds.
- [x] **Subproblema 4: Refatorar Entidade e Testes Finais (`src/rules/character/entity.rs`)**
  - Adicionar as propriedades Race e Background à ficha de personagem.
  - Implementar lógica matemática rigorosa (Bônus Racial aplicado antes da Vida Máxima e `proficiency_bonus`).
  - Passar 100% da suíte de testes.
