# Plano de Implementação: Classes do SRD 5.1 (Páginas 9 a 55)

## Objetivo
Implementar em Rust todas as **12 classes oficiais do SRD 5.1** (Bárbaro, Bardo, Clérigo, Druida, Guerreiro, Monge, Paladino, Patrulheiro, Ladino, Feiticeiro, Bruxo e Mago), expandindo o `trait CharacterClass` e o enum `ClassVariant` em `src/rules/character/class.rs`.

## As 12 Classes do SRD (Páginas 9 a 55)
1. **Barbarian (Bárbaro)** - Dado de Vida `D12`, Proficiências em Salvaguarda: `STR`, `CON`.
2. **Bard (Bardo)** - Dado de Vida `D8`, Proficiências em Salvaguarda: `DEX`, `CHA`.
3. **Cleric (Clérigo)** - Dado de Vida `D8`, Proficiências em Salvaguarda: `WIS`, `CHA`.
4. **Druid (Druida)** - Dado de Vida `D8`, Proficiências em Salvaguarda: `INT`, `WIS`.
5. **Fighter (Guerreiro)** - Dado de Vida `D10`, Proficiências em Salvaguarda: `STR`, `CON`.
6. **Monk (Monge)** - Dado de Vida `D8`, Proficiências em Salvaguarda: `STR`, `DEX`.
7. **Paladin (Paladino)** - Dado de Vida `D10`, Proficiências em Salvaguarda: `WIS`, `CHA`.
8. **Ranger (Patrulheiro)** - Dado de Vida `D10`, Proficiências em Salvaguarda: `STR`, `DEX`.
9. **Rogue (Ladino)** - Dado de Vida `D8`, Proficiências em Salvaguarda: `DEX`, `INT`.
10. **Sorcerer (Feiticeiro)** - Dado de Vida `D6`, Proficiências em Salvaguarda: `CON`, `CHA`.
11. **Warlock (Bruxo)** - Dado de Vida `D8`, Proficiências em Salvaguarda: `WIS`, `CHA`.
12. **Wizard (Mago)** - Dado de Vida `D6`, Proficiências em Salvaguarda: `INT`, `WIS`.

## Checklist de Subproblemas
- [x] **Subproblema 1: Expansão do Trait e Classes Marciais (`Barbarian`, `Fighter`, `Monk`, `Rogue`)**
  - Expandir o `trait CharacterClass` com `get_name` e `get_class_features_summary`.
  - Codificar as structs e implementações para Bárbaro, Guerreiro, Monge e Ladino.
- [x] **Subproblema 2: Classes Conjuradoras (`Cleric`, `Druid`, `Sorcerer`, `Wizard`)**
  - Codificar as structs e implementações para Clérigo, Druida, Feiticeiro e Mago.
- [x] **Subproblema 3: Classes Híbridas/Pactos (`Bard`, `Paladin`, `Ranger`, `Warlock`) e Atualização do `ClassVariant`**
  - Codificar as structs para Bardo, Paladino, Patrulheiro e Bruxo.
  - Atualizar o enum `ClassVariant` com todas as 12 classes e adicionar métodos de delegação na `Entity`.
  - Criar testes unitários para todas as 12 classes.

## Ponto de Partida
**Sugiro começarmos pelo Subproblema 1 (Expansão do Trait + Bárbaro, Guerreiro, Monge e Ladino)**.
