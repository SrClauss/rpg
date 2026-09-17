# Roadmap de Implementação das Regras (D&D 5e SRD)

Este documento mapeia os passos exatos para traduzir o livro de regras (SRD-OGL_V5.1.pdf) para o código Rust, organizados do pilar mais genérico para o mais específico.

---

## Passo 1: A Fundação Genérica (Personagens Básicos)
*Corresponde ao Livro: Capítulos 1, 2, 3, e 4.*

- [x] **Capítulo 1: Atributos Básicos (Ability Scores)**
  - Implementar os 6 atributos principais (FOR, DES, CON, INT, SAB, CAR).
  - Codificar a matemática exata de conversão de Valor de Habilidade para Modificador.
  - Implementar Testes Unitários de cálculo (garantindo que `Valor 1 = Modificador -5`).
- [x] **A Entidade Base (Entity)**
  - Estrutura base de qualquer ator do jogo (Possui PV, Deslocamento, Tamanho, Atributos).
- [x] **Capítulo 2: Raças (Races)**
  - Criar o modelo de Traços Raciais (Racial Traits) e Bônus de Atributos (Ability Score Increase).
- [x] **Capítulo 3: Classes (Classes)**
  - Modelar Dados de Vida (Hit Dice).
  - Sistema de Proficiências e aquisição de Características de Classe (Class Features) por Nível.
- [x] **Capítulo 4: Personalidade e Antecedentes (Backgrounds)**
  - Modelar as proficiências em perícias e ferramentas concedidas.

---

## Passo 2: O Físico (Equipamento e Movimento)
*Corresponde ao Livro: Capítulos 5, 8.*

- [x] **Capítulo 5: Equipamento (Equipment)**
  - Tabela de Armas (Dano base, tipo de dano e Propriedades como *Acuidade*, *Pesada*).
  - Tabela de Armaduras e cálculo dinâmico da Classe de Armadura (CA = Base + Mod. DES limitado).
  - Sistema simples de peso e capacidade de carga (Opcional para a engine de batalha).
- [x] **Capítulo 8: Aventurando-se (Adventuring)**
  - Modelar Tipos de Movimento e Deslocamento, Dano de Queda e as regras de Descanso Longo e Curto (Recuperação de PV e Dados de Vida).

---

## Passo 3: O Motor de Regras (Combate e Testes)
*Corresponde ao Livro: Capítulos 7, 9, e Apêndice A (Condições).*

- [x] **Capítulo 7: Usando Valores de Habilidade**
  - Implementar Testes de Habilidade (Ability Checks) e Testes de Resistência (Saving Throws) usando o módulo de dados.
  - Lógica de Vantagem e Desvantagem aninhada aos eventos.
- [x] **Capítulo 9: Combate (Combat)**
  - Sistema de Iniciativa e organização de turnos.
  - O ciclo de Ação: Ação Comum, Ação Bônus, Reação, Interação Livre.
  - Resolução de Ataque (`1d20 + Modificador + Proficiência` vs `CA`).
  - Acertos Críticos (dobrar dados de dano) e Falhas Críticas.
  - Implementação de Resistências, Imunidades e Vulnerabilidades a tipos de dano.
- [x] **Apêndice A: Condições (Conditions)**
  - Criar o sistema de "Efeitos de Status" (Ex: Cego impõe Desvantagem em ataques e dá Vantagem aos inimigos).

---

## Passo 4: O Oculto (Magia)
*Corresponde ao Livro: Capítulos 10 e 11.*

- [ ] **Capítulo 10: Conjuração (Spellcasting)**
  - Gestão de Espaços de Magia (Spell Slots).
  - Regras de Componentes (V, S, M).
  - Sistema de Concentração (Teste de Resistência de Constituição ao tomar dano).
- [ ] **Capítulo 11: Magias (Spells)**
  - Criação da estrutura de dados (JSON/SurrealDB) das magias do SRD.
  - Motor de Eventos para aplicar os efeitos numéricos das magias no combate (Áreas de efeito, Dificuldade de Magia, Danos e Buffs).

---

## Passo 5: Inserção Massiva de Dados
*Corresponde ao Livro: Apêndice MM (Monsters).*

- [ ] **Apêndice: Bestiário**
  - Mapear atributos, ataques multi-ataque e habilidades especiais das criaturas do SRD para os formatos aceitos pela nossa engine Híbrida.
  - Povoar o SurrealDB com as informações.

---

## 🏛️ Discussões Arquiteturais e Decisões

### Abordagem para "Classes" (Estruturas Fixas em Código vs JSON)
Por exigência de design, **as Classes no D&D (Mago, Guerreiro, etc.) não serão meros contêineres de dados JSON, mas sim estruturas de código fixas (Structs nativas em Rust)**.

Como as mecânicas de um Mago (gerenciamento de *Spell Slots*, preparo de magias, grimório) são fundamentalmente diferentes das de um Guerreiro (Ação Surto, Manobras de Combate, Superiority Dice), tentar forçar isso em um formato puramente *Data-Driven* limitaria a fidelidade das regras.

**A solução em Rust (Traits e Enums):**
1. **O Trait Genérico (`trait CharacterClass`):** Definiremos uma interface comum que toda classe deve obedecer, com métodos como `aplicar_beneficios_de_nivel()`, `acoes_disponiveis()`, e `reacoes_disponiveis()`.
2. **As Structs Específicas (`struct Wizard`, `struct Fighter`):** Cada classe terá seu próprio arquivo `.rs` contendo a lógica pesada de suas habilidades. O Mago terá lógica de código real para conjuração, e o Guerreiro lógica de código real para seus ataques extras.
3. **Multiclasse via Composição (`Vec<ClassVariant>`):** Para suportar a mecânica de Multiclasse do D&D, a Entidade não "se transforma" em uma nova classe. Em vez disso, ela possui um vetor (`Array`/`List`) de componentes de classe. Um personagem Guerreiro Nível 2 e Mago Nível 1 simplesmente armazenará `[ClassVariant::Fighter(lvl2), ClassVariant::Wizard(lvl1)]`. Em tempo de execução, o motor itera por esse vetor para extrair ações e habilidades passivas de todas as classes que a entidade possui.
