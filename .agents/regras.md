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

---

## 6. Fluxo Macro da Partida, Esqueleto Oculto, Memória Indexada e "Cena Viva"

### 6.1. Criação da Sessão pelo Guest, Lobby Multiplayer e Esqueleto Narrativo Oculto
- **Papel da IA (100% GM) e Composição da Mesa:** A IA atua exclusivamente como Mestre de Jogo (GM). O **Guest** (criador da sessão) é um jogador com seu próprio personagem e pode convidar de **1 a 3 amigos** via WebSockets, compondo uma *party* de **1 a 4 jogadores** (`Guest + até 3 convidados`).
- **Premissa e Estilo Literário:** Na criação da sessão, o Guest fornece apenas a **Premissa da História** e o **Estilo/Tom em que ela será contada** (não há inserção manual de esqueleto pelo usuário).
- **Esqueleto Narrativo Oculto no Servidor:**
  - A partir da Premissa e do Estilo, a IA gera proceduralmente no servidor o **Esqueleto Narrativo**, que permanece **100% oculto** para todos os jogadores (incluindo o Guest, evitando *spoilers*).
  - O Esqueleto contém:
    1. Os **eventos autônomos do mundo** (o que acontecerá nos bastidores independentemente das ações do grupo);
    2. O **rumo esperado da história** (capítulos/arcos planejados).
  - **Corrupção e Recálculo do Esqueleto:** A IA conduz a campanha seguindo o Esqueleto existente. Caso as ações dos jogadores fujam drasticamente do esperado e "corrompam" a linha planejada, a IA possui total liberdade para recalcular e reescrever o Esqueleto Narrativo daquele ponto em diante.

### 6.2. O "Livro da Crônica" (Log Geral) vs. Memória Viva de Entidades (JSON + Ponteiros)
- **O Livro (Log Geral Rastreável):** Registra de forma contínua e cronológica toda a prosa literária consolidada e todas as interações reais da campanha.
- **Fichas JSON Dinâmicas por Entidade (Personagens/NPCs, Lugares e Objetos):**
  - Sempre que uma entidade relevante surge ou é mencionada na campanha, o sistema cria/atualiza sua estrutura JSON própria **fora do log contínuo**, evitando que a IA precise reprocessar todo o histórico toda vez que a entidade reaparecer.
  - **Mutação Psicológica/Estrutural + Ponteiro para o Log Geral:**
    - Quando um evento marcante afeta uma entidade (ex: *Capítulo 1: NPC João vê a esposa ser morta por lobisomens*; mais tarde: *João é traído pelo Ladino do grupo*), a IA calcula o impacto desse evento (trauma, rancor, mudança de lealdade, estado físico) e grava esse estado diretamente no JSON de `João`.
    - **Consulta Profunda sob Demanda:** Cada evento registrado no JSON da entidade guarda também a **referência/ID do evento no Log Geral (Livro)**. Assim, no uso comum, a IA lê apenas o JSON enxuto da entidade; mas, se precisar de detalhes granulares do que ocorreu naquele episódio passado (ex: falas exatas ou circunstâncias da traição), a IA consulta pontualmente aquele trecho no Log Geral.

### 6.3. Alternância de Agentes e o Conceito de "Cena Viva" (`Living Scene`) Regida pelo D&D 5e
A campanha alterna entre dois modos de operação de IA:
1. **Modo Crônica Literária (Escritor + Revisor + Reescrita):**
   - Conduz o avanço macro da história pelo Esqueleto Narrativo e escreve a prosa polida no **Livro** para todos os jogadores acompanharem.
2. **Modo "Cena Viva" (`Living Scene` — Agente de Ambiente + Motor D&D 5e em Rust):**
   - Quando a narrativa atinge um ponto de ação, exploração ou interação direta do grupo (ex: chegada a uma cidade, diálogo tenso ou combate), a IA Literária pausa e entra em ação o **Agente de Ambiente** (IA rápida, direta e **sem etapa de revisão**).
   - **Soberania das Regras do D&D 5e na Cena Viva:**
     - **Toda e qualquer interação ou combate dentro da Cena Viva é decidida pelas regras determinísticas do D&D 5e no motor Rust.**
     - **Interações Sociais e de Exploração:** Se um jogador tenta convencer, intimidar ou enganar um NPC (como o `João`), investigar uma pista ou forçar uma passagem, o Agente de Ambiente consulta o JSON do NPC/Cenário (ex: se `João` já foi traído pelo Ladino, o JSON dele impõe **Desvantagem** ou aumenta a **CD** contra o Ladino) e aciona o motor Rust para rolar o teste de perícia/atributo real (`d20 + modificador + proficiência vs. CD` ou teste resistido). O NPC só reage de acordo com o resultado matemático dos dados.
     - **Combate e Recursos:** Segue estritamente a fila de Iniciativa (`d20 + DEX`), economia de turno (`Ação`, `Ação Bônus`, `Movimento`, `Reação`), Classe de Armadura (`CA`), Salvaguardas e gasto real de *Espaços de Magia* e recursos das 12 classes na ficha de cada jogador e dos inimigos.
     - **Ordem de Turnos Multiplayer (1 a 4 Jogadores):** Mesmo fora de combate, a Cena Viva opera em rodadas organizadas para que todos os 4 jogadores conectados via WebSocket tenham sua vez de agir/falar usando suas 3 Vias, seguidos pela reação empática e tática do Agente de Ambiente.
   - **Objetivos Claros da Cena Viva (Prevenção de Loop Infinito):**
     - Ao iniciar uma **Cena Viva**, a IA define e entrega ao grupo os **Objetivos / Marcos de Resolução** daquele estágio (o que precisa acontecer ou ser decidido para aquela etapa se cumprir).
     - O próprio **Agente de Ambiente** monitora esses objetivos e detecta automaticamente o fim da Cena Viva quando:
       - O grupo atinge os objetivos/ações esperados da cena; **OU**
       - O grupo decide implicitamente abandonar/sair do local.
3. **Fechamento da Cena Viva e Retorno ao Livro:**
   - Encerrada a **Cena Viva**, o sistema:
     1. Gera um **resumo estruturado** de tudo o que ocorreu durante a Cena Viva;
     2. Calcula o **impacto** nos JSONs das entidades envolvidas e avalia se o **Esqueleto Narrativo** foi mantido ou corrompido;
     3. Aciona a função da **Dobradinha Literária (Escrita $\rightarrow$ Revisão $\rightarrow$ Reescrita)** para transformar o resumo da Cena Viva em narrativa literária definitiva no **Livro da História**, sincronizando a leitura para todos os jogadores da sala.

### 6.4. Arquitetura de Gravação e Persistência de Estado por Campanha
Cada campanha é isolada em seu próprio diretório estruturado (`data/campaigns/<campaign_id>/`) utilizando uma **arquitetura híbrida de arquivos JSON modulares + Log Append-Only (`JSONL`) indexado em memória pelo Rust**:
1. **`manifest.json` (Sessão, Premissa e Esqueleto Oculto):**
   - Guarda os metadados da sala, os IDs dos 1 a 4 jogadores (`Guest + até 3`), a `Premise`, o Estilo Literário e o **Esqueleto Narrativo Oculto** (incluindo seu histórico de mutações caso tenha sido corrompido).
2. **`chronicle_log.jsonl` (O Livro Geral Rastreável — *Append-Only*):**
   - Arquivo em formato *JSON Lines* onde cada linha é um evento imutável com `event_id` sequencial (ex: `evt_0001`, `evt_0042`), contendo o timestamp, capítulo/cena, rolagens determinísticas de D&D 5e e o texto narrativo/diálogo exato.
   - **Por que `JSONL` com Índice de Offset em Memória (`HashMap<String, u64>`):** Como o Livro só cresce, gravar novas entradas no fim do arquivo é instantâneo ($O(1)$) e à prova de corrupção. Quando a IA precisa consultar os detalhes exatos de um trauma passado de um NPC, o servidor Rust usa o `event_id` gravado na ficha do NPC para buscar diretamente a linha exata no `chronicle_log.jsonl` em menos de 1 milissegundo, sem carregar o resto do livro na memória da IA.
3. **`entities/players/<player_id>.json` e `entities/world/<entity_id>.json` (Fichas Vivas Isoladas):**
   - Cada jogador (1 a 4) e cada entidade do mundo (NPCs como `joao.json`, Locais, Objetos importantes) possui seu próprio arquivo JSON individual (gravado de forma atômica via arquivo `.tmp` + `rename`).
   - O JSON de cada entidade contém:
     - **Bloco Mecânico D&D 5e:** Atributos, HP, CA, Perícias, Condições, Espaços de Magia usados/totais e Recursos de Classe.
     - **Bloco de Memória e Impactos (`memory_impacts`):** Lista enxuta dos eventos que moldaram aquela entidade, contendo o impacto calculado pela IA (ex: *"Odeia lobisomens; Desconfia profundamente do Ladino do grupo (+5 na CD de Persuasão para o Ladino)"*) e o ponteiro `source_event_id` apontando para o `chronicle_log.jsonl`.
4. **`active_scene.json` (Checkpoint da Cena Viva em Andamento):**
   - Mantém o estado em tempo real da **Cena Viva** aberta (objetivos da cena, turno atual, orçamento de ações de cada jogador, posição no Grid e turnos já jogados na cena). Se o servidor reiniciar no meio de um combate ou diálogo, a Cena Viva retoma exatamente do turno em que parou.

