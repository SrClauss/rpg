# Plano de Implementação: Catálogo de Equipamentos SRD 5.1 & Registro Homebrew (Páginas 62 a 75)

## Objetivo
Implementar em Rust a **totalidade dos equipamentos oficiais do SRD 5.1** (~189 itens: Armaduras, Armas, Equipamentos de Aventura, Ferramentas, Kits e Montarias) de forma nativa/hardcoded, combinados com um registro central (`EquipmentRegistry`) que permite o carregamento dinâmico de equipamentos **Homebrew via arquivos JSON**.

## Abordagem Dupla (SRD Hardcoded + Homebrew JSON)
- **SRD 5.1 (Oficial)**: Construtores nativos em Rust para todas as 13 Armaduras, 37 Armas, ~83 Equipamentos de Aventura, 37 Ferramentas/Kits e 19 Montarias/Veículos.
- **Homebrew**: Suporte a desserialização de novos itens customizados em JSON via `EquipmentRegistry::load_json()`.

## Checklist de Subtarefas

- [x] **Subtarefa 1: Todas as 13 Armaduras e 37 Armas SRD 5.1 com Propriedades Completas**
  - Adicionar todas as 13 armaduras em `src/rules/equipment/armor.rs` (Custo, Peso, CA, Força Mínima, Desvantagem em Furtividade).
  - Expandir `WeaponProperty` (Ammunition, Loading, Special) e adicionar todas as 37 armas em `src/rules/equipment/weapons.rs` (Custo, Peso, Alcance, Dano, Propriedades).
  - Escrever testes unitários para todas as armaduras e armas.

- [ ] **Subtarefa 2: Equipamentos de Aventura (`gear.rs`), Ferramentas/Kits (`tools.rs`) e Montarias (`mounts.rs`)**
  - Criar `src/rules/equipment/gear.rs` com os ~83 itens de aventura (custo em PO/PE/PP/PC, peso, descrição e uso).
  - Criar `src/rules/equipment/tools.rs` com as 37 ferramentas, kits de artesão e instrumentos musicais.
  - Criar `src/rules/equipment/mounts.rs` com as 19 montarias, selas e veículos.
  - Escrever testes unitários.

- [ ] **Subtarefa 3: Registro Central `EquipmentRegistry` com Suporte a Homebrew JSON**
  - Criar `src/rules/equipment/registry.rs` agregando todo o catálogo.
  - Implementar métodos `get_item(id)`, `spawn_item(id)` e `load_json(json_str)`.
  - Escrever testes unitários validando a busca por ID e carregamento de equipamentos homebrew via JSON.

## Subtarefa Atual
**Subtarefa 1: Todas as 13 Armaduras e 37 Armas SRD 5.1 com Propriedades Completas**
