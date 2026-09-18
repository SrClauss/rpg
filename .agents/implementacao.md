# Plano de Implementação: Raças do SRD (Páginas 3 a 7) e Traços Raciais

## Objetivo
Implementar nativamente em Rust todas as 9 raças e subraças do SRD 5.1 (páginas 3 a 7), expandindo o `trait Race` e a `Entity` para expor **todos os benefícios, resistências a dano, vantagens em salvaguardas (Darkvision, Fey Ancestry, Gnome Cunning, Relentless Endurance, Lucky, etc.)**.

## Métodos Expandidos do `trait Race`
- `get_speed(&self) -> u8`
- `get_size(&self) -> Size`
- `get_ability_score_increases(&self) -> Vec<(Ability, i8)>`
- `has_darkvision(&self) -> bool`
- `has_fey_ancestry(&self) -> bool` (Vantagem vs Encantamento, imunidade a Sono mágico)
- `has_lucky(&self) -> bool` (Pequenino: rola novamente 1s naturais)
- `has_gnome_cunning(&self) -> bool` (Gnomo: Vantagem em Salvaguardas INT/SAB/CAR vs Magia)
- `has_relentless_endurance(&self) -> bool` (Meio-Orc: Sobrevive a 0 HP com 1 HP uma vez/descanso)
- `get_damage_resistances(&self) -> Vec<DamageType>` (Resistências raciais: Anão -> Veneno, Tiefling -> Fogo, Draconato -> Elemento ancestral)
- `get_traits_summary(&self) -> Vec<String>` (Lista textual dos traços para o contexto da IA e Ficha)

## Checklist de Subproblemas
- [x] **Subproblema 1: Expansão do `trait Race` e Raças Anãs e Élficas (`Dwarf`, `HillDwarf`, `Elf`, `HighElf`)**
  - Adicionar os novos métodos de traços ao `trait Race`.
  - Codificar as structs e implementações para Anão, Anão da Colina, Elfo e Alto Elfo.
- [x] **Subproblema 2: Raças Pequenas e Humanas (`Halfling`, `LightfootHalfling`, `Human`, `Gnome`, `RockGnome`)**
  - Codificar structs para Pequenino Pés-Leves, Gnomo das Rochas e Humano.
- [x] **Subproblema 3: Raças Dracônicas e Híbridas (`Dragonborn`, `HalfElf`, `HalfOrc`, `Tiefling`) e Integração com `Entity`**
  - Codificar structs para Draconato, Meio-Elfo, Meio-Orc e Tiefling.
  - Adicionar métodos auxiliares na `Entity` para consultar diretamente os traços da raça (`entity.has_darkvision()`, `entity.get_damage_resistances()`, etc.).
  - Escrever testes unitários abrangentes para todas as raças.

## Ponto de Partida
**Sugiro começarmos pelo Subproblema 1 (Expansão do Trait + Anões e Elfos)**.
