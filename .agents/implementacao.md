# Plano de Implementação: Passo 4 (O Oculto: Magia)

## Objetivo
Implementar o Passo 4 do Roadmap do D&D 5e: construir o sistema de magia e conjuração (Spellcasting), cobrindo a modelagem de magias do SRD, gerenciamento de Espaços de Magia (Spell Slots), sistema de Concentração e o motor de resolução de conjurações.

## Checklist de Subproblemas
- [x] **Subproblema 1: Estrutura Base de Magias (`src/rules/spells/spell.rs`)**
  - Definir o modelo `Spell` contendo nível (0-9), escola de magia, alcance, tempo de conjuração, componentes (V, S, M), duração e flag de concentração.
  - Definir tipos de efeito de magia (Dano, Cura, Salvaguarda vs CD, Ataque Mágico).
- [x] **Subproblema 2: Espaços de Magia e Concentração (`src/rules/spells/spellcaster.rs`)**
  - Adicionar o gerenciamento de Espaços de Magia (`SpellSlots`) por nível (slots máximos e disponíveis) na `Entity`.
  - Implementar rastreamento de Concentração ativa e teste de Salvaguarda de Constituição (DC = max(10, dano / 2)) ao sofrer dano.
- [x] **Subproblema 3: Motor de Resolução de Conjuração (`src/rules/spells/casting.rs`)**
  - Implementar o método `cast_spell`: validação de slots/componentes, consumo de slot, cálculo da DC de Salvaguarda de Magia (`8 + Prof + Mod`) ou Bônus de Ataque Mágico, e aplicação dos efeitos/dano no alvo.

## Ponto de Partida
**Sugiro começarmos pelo Subproblema 1 (Estrutura Base de Magias)**, pois criar os tipos de dados e representações de magias do SRD fornecerá a base necessária para que o gerenciador de slots e o motor de conjuração operem.
