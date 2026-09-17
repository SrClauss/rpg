# Plano de Implementação: Passo 2 (Físico, Equipamento e Movimento)

## Objetivo
Implementar o Passo 2 do Roadmap do D&D 5e: criar o sistema base de armas, armaduras e o cálculo dinâmico da Classe de Armadura (CA) para a nossa Entidade, fundamentando as mecânicas físicas necessárias para o futuro motor de batalha.

## Checklist de Subproblemas
- [x] **Subproblema 1: Criação da Base de Armas (`src/rules/equipment/weapons.rs`)**
  - Implementar enum para Tipos de Dano (Cortante, Perfurante, Concussão).
  - Implementar propriedades das armas (Acuidade, Pesada, Versátil).
  - Modelar a estrutura `Weapon` com o Dado de Dano.
- [x] **Subproblema 2: Criação da Base de Armaduras (`src/rules/equipment/armor.rs`)**
  - Implementar enum de Categoria de Armadura (Leve, Média, Pesada, Escudo).
  - Modelar a estrutura `Armor` contendo a CA Base e limites do Modificador de Destreza.
- [x] **Subproblema 3: Integração do Equipamento na Ficha (`src/rules/character/entity.rs`)**
  - Adicionar slots de equipamento ou lista de itens equipados na `Entity`.
  - Criar o método `get_armor_class(&self)` que calcula dinamicamente a CA (Base + Modificador de DES limitado pela armadura).
- [x] **Subproblema 4: Sistema de Aventurar-se (`src/rules/adventuring.rs`)**
  - Codificar a estrutura base de Tipos de Movimento (Caminhada, Voo, Natação) e as regras de Descanso Longo (recuperação total de PV).

## Ponto de Partida
**Sugiro começarmos pelo Subproblema 1 (Armas)**, pois a modelagem dos tipos de dano (Cortante, Concussão, etc.) e os tipos de dados criam o pilar central físico. Uma vez que armas e armaduras estiverem construídas isoladamente, a integração matemática na Ficha do Personagem (Subproblema 3) fluirá de forma muito mais segura.
