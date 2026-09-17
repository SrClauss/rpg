# Regras e Diretrizes para Implementação do D&D 5e SRD

Este documento define as regras e diretrizes que o agente (eu) deve seguir ao implementar as mecânicas do D&D 5e baseadas no arquivo `SRD-OGL_V5.1.pdf`. Ele serve como um guia arquitetural para garantir que a implementação de Raças, Classes, Magias e Bestiário seja feita de maneira consistente, escalável e segura.

## 1. Abordagem Arquitetural Principal
O motor deve ser implementado seguindo uma **Abordagem Híbrida (Event-Driven + Data-Driven)**:
- **Event-Driven (Código Rust):** O núcleo do combate, resolução de turnos e mecânicas de modificadores complexos (como reações e Vantagens) são nativos no código.
- **Estruturas Fixas para Classes:** Devido à imensa disparidade mecânica (ex: Conjuração do Mago vs. Manobras do Guerreiro), **Classes serão implementadas via código nativo no Rust** (Structs/Traits próprios), não como meros dados genéricos.
- **Data-Driven (SurrealDB / JSON):** Dados puros (atributos base do Bestiário, listas de itens e magias base) devem ser modelados via `serde`. Evite fazer *hardcode* de centenas de monstros/magias que seguem o mesmo padrão estrutural.

## 2. Fluxo de Implementação (Capítulo por Capítulo)
A inserção do conteúdo deve seguir estritamente esta ordem, sem pular etapas, para garantir que as dependências mecânicas sejam resolvidas:

### Passo 1: Capítulos 1, 2 e 3 (Criação de Personagens)
- **Atributos Básicos:** Implementar (FOR, DES, CON, INT, SAB, CAR), o cálculo de modificadores (lembrando do arredondamento correto para negativos no Rust) e testes unitários associados.
- **Raças:** Modelar Tamanho, Deslocamento, Traços Raciais e Aumentos de Valor de Habilidade.
- **Classes:** Modelar Proficiências, Dados de Vida (Hit Dice) e um sistema genérico para Habilidades de Classe ganhas por nível.

### Passo 2: Capítulo 5 (Equipamentos)
- Modelar o sistema de moedas, peso e carga.
- Implementar as propriedades das Armas (Acuidade, Pesada, Alcance, etc.) e os tipos de dano.
- Implementar o cálculo dinâmico da Classe de Armadura (CA) considerando Escudos e Modificadores de Destreza.

### Passo 3: Capítulos 7 e 9 (Uso de Atributos e Combate)
- Implementar rolagens de testes de resistência (Saving Throws) e testes de habilidade.
- Sistema de Ação, Ação Bônus, Reação e Movimento.
- Criar o *Event Bus* de combate (ex: evento `AtaqueResolvido`, `DanoSofrido`) para que habilidades e magias possam "escutar" e alterar o resultado.

### Passo 4: Capítulo 11 (Magias)
- Implementar as estruturas de Conjuração (Slots de magia, Componentes V, S, M, Concentração).
- Preparar o interpretador de efeitos para magias (ex: causar X dano em área, impor condição "Amedrontado").

### Passo 5: Apêndice (Bestiário)
- Após o motor de combate e magias estar robusto, iniciar a inserção em massa do Bestiário, idealmente formatando os dados do PDF para inserção no SurrealDB.

## 3. Padrões de Código Exigidos
- **Preservação:** Nunca altere ou apague funções, docstrings ou comentários preexistentes não relacionados à tarefa atual.
- **Testabilidade:** Toda fórmula matemática crucial (como modificador de atributo, cálculo de CA, e rolagem com vantagem) deve ter um `#[test]` associado no mesmo arquivo.
- **Organização Modular:** Agrupe as mecânicas em pastas lógicas dentro de `src/rules/` (ex: `src/rules/character`, `src/rules/combat`, `src/rules/magic`).
- **Comunicação:** Antes de realizar mudanças maciças na arquitetura do código ou iniciar a inclusão de um novo capítulo, consulte o usuário (apresentando um plano de ação).

## 4. Uso do PDF (SRD)
- O PDF fornecido (`SRD-OGL_V5.1.pdf`) é a fonte de verdade absoluta das regras.
- Se houver ambiguidades nas regras durante a programação, levante a dúvida com o usuário antes de assumir um comportamento.
