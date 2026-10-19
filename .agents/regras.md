# Regras de Integração IA & Motor Rust

Estas regras definem o protocolo de funcionamento entre as IAs e o Motor Determinístico em Rust.

## 1. Pipeline de Refinamento Editorial em Duas Vias (Dual-Pass Narrative Pipeline)
A geração de cenas dramáticas, arcos narrativos e fechamento de *Beats* opera através de um pipeline em 3 passos coordenados, 100% *stateless*, econômico e de altíssima densidade literária:
1. **Passo 1 (Criação / Rascunho) — Gemini Flash-Lite:**
   - O Gemini Flash-Lite recebe o estado atual (`Premise`, histórico do `Beat` e a intenção do jogador) e gera um rascunho rápido, cinematográfico e sensorial da cena.
2. **Passo 2 (Linter Crítico & Regras) — DeepSeek V3:**
   - O DeepSeek V3 atua exclusivamente como **Editor Literário e Árbitro de Regras**.
   - Ele analisa o rascunho do Gemini e emite um parecer crítico conciso: corta clichês de folhetim, elimina exposições didáticas desnecessárias (*infodump*), ajusta a verossimilhança psicológica e valida a consistência com as regras do mundo.
3. **Passo 3 (Narração Final Polida) — Gemini Flash-Lite:**
   - O Gemini Flash-Lite recebe seu rascunho original acrescido das diretrizes do DeepSeek e reescreve a cena final definitiva, entregando uma prosa madura, profunda e homogênea ao jogador.

## 2. O Motor Rust (A Fonte da Verdade e Resolução Mecânica)
- Todos os testes de dados, regras de D&D 5e, fichas de personagens, condições e controle de combate ocorrem exclusivamente no backend em Rust de forma determinística e são gravados no `CampaignState`.
- Quando uma ação exigir rolagem mecânica, o Gemini emite a tag `#regras` (ou chamada de função estruturada). O Rust calcula o resultado matemático exato (D20 + modificadores, dano, CA, resistências) e devolve o JSON determinístico para a narração.

## 3. Estrutura Narrativa Baseada em "Beats" (Pulsos)
- A narrativa não é contínua e amorfa; ela é dividida e processada em **Beats** (Pulsos Narrativos).
- O `CampaignState` armazena a narrativa organizada por esses Beats.
- **Mutação de Entidades:** No final de cada Beat, o motor resolve e aplica as consequências no Estado do Mundo. Qualquer entidade afetada (Personagem, Objeto importante, ou Local) sofrerá mutação em sua ficha/estado com rastreabilidade histórica (`EntityMutation`).

## 4. Premissa e Diretrizes Narrativas
- O `CampaignState` mantém no topo a `Premise` da campanha (Gênero, Tropos, Tom e Gancho Inicial).
- A Dobradinha Sinistra consulta essa Premissa a cada novo Beat para garantir que o tom permaneça consistente do início ao fim da campanha.

---

## 5. Regras de Negócio e Fluxos do Frontend

### 5.1. Fluxo Inicial: Criação de Personagens
- O fluxo do jogador inicia na **Criação de Personagem**, onde ele define Raça, Antecedente, Atributos (`STR`, `DEX`, `CON`, `INT`, `WIS`, `CHA`), Perícias, Equipamento Inicial e escolhe uma das **12 Classes Originais do D&D 5e** (sem escopo de interface para *homebrew* neste momento).
- Ao concluir a criação, o jogador é direcionado para a **Tela de Jogo Específica da sua Classe**.

### 5.2. As 12 Telas de Jogo por Classe (HUD Assimétrico do Jogador)
A tela onde o jogador enxerga o jogo e se comunica com o mundo é **diferenciada para cada uma das 12 classes originais**. Ela funciona como o "cockpit" completo daquele personagem, exibindo em tempo real:
- **Núcleo Comum Obrigatório em Todas as 12 Telas:**
  - **Atributos e Modificadores:** `STR`, `DEX`, `CON`, `INT`, `WIS`, `CHA`, Bônus de Proficiência, Salvaguardas (*Saving Throws*) e Perícias.
  - **Status Vitais e Combate:** Pontos de Vida (`HP` atual / máximo / temporário), Classe de Armadura (`CA`), Iniciativa, Deslocamento, Dados de Vida e Salvaguardas contra a Morte.
  - **Controle de Recursos e Espaços de Magia:** Marcadores visuais de *Espaços de Magia (Spell Slots)* usados vs. disponíveis por círculo (1º ao 9º), além de magias concentradas e condições ativas.
- **Diferenciação de Tela e Painéis de Habilidades por Classe:**
  1. **Bárbaro (`Barbarian`):** HUD focado no medidor de *Fúria* (ativa/inativa, usos gastos/restantes, bônus de dano), *Defesa Sem Armadura* (`10 + DEX + CON`), *Ataque Descuidado (Reckless Attack)* e *Crítico Brutal*.
  2. **Bardo (`Bard`):** HUD com contador de dados de *Inspiração de Bardo* (usados/restantes), *Canção de Descanso*, *Aptidão (Expertise)*, painel de *Espaços de Magia* usados/disponíveis e lista de Magias Conhecidas.
  3. **Bruxo (`Warlock`):** HUD exclusivo para *Magia de Pacto* (slots de círculo único que recarregam em Descanso Curto), lista de *Invocações Místicas* ativas, *Dádiva do Pacto* e habilidades do *Patrono*.
  4. **Clérigo (`Cleric`):** HUD com contador de usos de *Canalizar Divindade* (incluindo *Expulsar Mortos-Vivos*), painel de *Espaços de Magia* usados/disponíveis, *Magias de Domínio* (sempre preparadas) e gaveta de *Magias Preparadas* (`WIS + nível`).
  5. **Druida (`Druid`):** HUD com rastreador de usos de *Forma Selvagem (Wild Shape)* (exibindo os atributos e HP da besta assumida sobrepostos ao HP original), *Espaços de Magia* usados/disponíveis e *Magias Preparadas* (`WIS + nível`).
  6. **Feiticeiro (`Sorcerer`):** HUD centrado na reserva de *Pontos de Feitiçaria (Font of Magic)* gastos/restantes, conversor rápido entre *Pontos de Feitiçaria ↔ Espaços de Magia*, seletores de *Metamagia* (ex: Magia Acelerada, Magia Duplicada) e *Espaços de Magia*.
  7. **Guerreiro (`Fighter`):** HUD tático com marcadores de *Retomar o Fôlego (Second Wind)*, *Surto de Ação (Action Surge)*, *Indomável*, *Estilo de Combate* ativo e recursos de subclasse (ex: Dados de Superioridade / Manobras).
  8. **Ladino (`Rogue`):** HUD focado em gatilhos e dados de *Ataque Furtivo (Sneak Attack)*, atalhos permanentes de *Ação Ardilosa (Cunning Action: Disparada, Desengajar, Esconder)*, *Esquiva Sobrenatural* e *Especializações*.
  9. **Mago (`Wizard`):** HUD com separação entre *Grimório Completo* (para rituais e troca em descanso longo) e *Magias Preparadas Ativas* (`INT + nível`), marcador de *Recuperação Arcana* e *Espaços de Magia* usados/disponíveis por círculo.
  10. **Monge (`Monk`):** HUD centrado na reserva de *Pontos de Ki* gastos/restantes, dado atual de *Artes Marciais*, e atalhos diretos de Ki (*Rajada de Golpes*, *Defesa Paciente*, *Passo do Vento*, *Ataque Atordoante*).
  11. **Paladino (`Paladin`):** HUD com pool numérico de *Cura pelas Mãos (Lay on Hands)* (pontos gastos/restantes), usos de *Sentido Divino* e *Canalizar Divindade*, seletor de *Destruição Divina (Divine Smite)* acoplado aos ataques, Auras e *Espaços de Magia* (1º ao 5º círculo).
  12. **Patrulheiro (`Ranger`):** HUD com referências ativas de *Inimigo Favorito* e *Explorador Natural*, *Estilo de Combate*, marcação de presa (*Hunter's Mark*) e *Espaços de Magia* usados/disponíveis (1º ao 5º círculo).

### 5.3. Interação com o Mundo e Consulta Turno a Turno (Tríade de Ação)
Dentro da tela da sua classe, o jogador acompanha o mundo (narrativa do *Beat* atual e mapa/grid tático) e, a cada turno, consulta e executa suas possibilidades através de **3 vias integradas**:
1. **Via 1 — Possibilidades Mecânicas da Ficha (Economia de Ações):**
   - O painel filtra e exibe tudo o que o personagem pode executar no turno com base no seu orçamento atual (`Ação`, `Ação Bônus`, `Movimento`, `Reação`) e nos recursos restantes (ex: desabilita magias cujo círculo não tenha *Espaços de Magia* disponíveis, destaca armas equipadas, alcance, Linha de Visão e cobertura).
2. **Via 2 — Possibilidades Contextuais do Cenário (Sugestões do Mundo):**
   - O painel lista ações interativas extraídas da cena/ambiente atual (ex: *"Examinar o altar profanado [Religião]"*, *"Arrombar o baú [Ferramentas de Ladrão]"*, *"Empurrar a estátua sobre o inimigo [Atletismo]"*), mostrando o custo na economia do turno e a perícia/atributo associado.
3. **Via 3 — Interação Textual Livre (Avaliada pela IA + Motor de Regras):**
   - O jogador possui um campo de entrada livre para descrever qualquer ação criativa ou diálogo com o mundo.
   - **Arbitragem Inteligente:** A IA analisa a intenção textual do jogador no contexto da cena e decide automaticamente:
     - **Ação Puramente Narrativa/Social:** Segue para o pipeline narrativo sem exigir rolagem se não houver risco ou oposição mecânica.
     - **Ação com Impacto Mecânico (`#regras`):** A IA aciona o Motor Determinístico em Rust (ex: exigindo um teste de habilidade, salvaguarda, ataque improvisado ou gasto de recurso/slot), o Rust resolve a matemática e o estado (`CampaignState`), e o resultado alimenta a narração final do turno.



