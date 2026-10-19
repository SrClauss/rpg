# Plano de Implementacao — Frontend: Criacao de Personagens, 12 Telas de Classe e Interacao Turno a Turno

## Objetivo
Construir o frontend do RPG D&D 5e contendo o fluxo inicial de Criação de Personagens, **12 telas de jogo exclusivas** (uma para cada classe original do D&D 5e) exibindo todos os atributos, vida, recursos de classe e espaços de magia usados/disponíveis, integradas ao painel de comunicação com o mundo e consulta de possibilidades turno a turno através das 3 vias (Possibilidades da Ficha, Possibilidades Contextuais do Cenário e Interação Textual Livre avaliada pela IA).

---

## Checklist de Subproblemas

### Fase 1: Fundacao e Criacao de Personagens
- [x] **T1** — **Fundação do Projeto Frontend e Contratos de Dados (`frontend/`):** Configurar o projeto web (React + TypeScript + Vite) com os contratos de tipos espelhando o motor Rust (Atributos, Modificadores, Perícias, HP, CA, Espaços de Magia usados/totais, Recursos de Classe, Economia de Ações do Turno e Estado do Mundo/Beat).
- [x] **T2** — **Fluxo de Criação de Personagens:** Implementar a tela passo a passo de criação de personagem (Nome, Raça, Antecedente, alocação de Atributos `STR/DEX/CON/INT/WIS/CHA` com cálculo em tempo real de modificadores/CA/HP, Perícias, Equipamento e seleção das 12 Classes Originais), redirecionando ao final para a tela específica da classe escolhida.

### Fase 2: Nucleo de Jogo e Triade de Interacao Turno a Turno
- [x] **T3** — **Motor de Tela de Jogo Base & Console de Turno (3 Vias):** Implementar os módulos fundamentais presentes nas 12 telas de classe:
  - Painel de Atributos, Salvaguardas, Perícias e Status Vitais (`HP` atual/máx/temp, `CA`, Iniciativa, Deslocamento, Condições, Salvaguardas de Morte).
  - Rastreador interativo de Espaços de Magia (*Spell Slots* usados vs. disponíveis do 1º ao 9º círculo).
  - Visão de Mundo (Narrativa de *Beats* + Grid Tático de Combate).
  - Consultor de Possibilidades Turno a Turno com orçamento de `Ação`, `Ação Bônus`, `Movimento` e `Reação` operando nas **3 Vias**:
    1. *Via 1:* Possibilidades Mecânicas da Ficha filtradas pelos recursos/slots ativos.
    2. *Via 2:* Possibilidades Contextuais do Cenário (testes e interações com objetos/NPCs da cena).
    3. *Via 3:* Interação Textual Livre (enviada para avaliação da IA sobre uso ou não das mecânicas do jogo).

### Fase 3: As 12 Telas Exclusivas por Classe Original
- [x] **T4** — **Telas das Classes Marciais (`Bárbaro`, `Guerreiro`, `Ladino`, `Monge`):**
  - *Bárbaro:* HUD de *Fúria* (ativa/inativa, usos gastos, bônus de dano), *Defesa Sem Armadura* (`10 + DEX + CON`), *Ataque Descuidado* e *Crítico Brutal*.
  - *Guerreiro:* HUD de *Estilo de Combate*, *Retomar o Fôlego*, *Surto de Ação*, *Indomável* e Manobras/Dados de Superioridade.
  - *Ladino:* HUD de *Ataque Furtivo* (dados escaláveis e validação de vantagem/aliado adjacente), atalhos de *Ação Ardilosa* e *Especializações (Expertise)*.
  - *Monge:* HUD de *Pontos de Ki* (usados/restantes), atalhos de Ki (*Rajada de Golpes*, *Defesa Paciente*, *Passo do Vento*), dado de *Artes Marciais* e *Defesa Sem Armadura* (`10 + DEX + WIS`).
- [x] **T5** — **Telas dos Conjuradores Divinos e Naturais (`Clérigo`, `Druida`, `Paladino`, `Patrulheiro`):**
  - *Clérigo:* HUD de *Canalizar Divindade* (usos e efeitos), *Magias de Domínio* sempre preparadas, gaveta de *Magias Preparadas* (`WIS + nível`) e *Espaços de Magia*.
  - *Druida:* HUD de *Forma Selvagem (Wild Shape)* com sobreposição de HP/atributos da besta assumida, *Magias Preparadas* (`WIS + nível`) e *Espaços de Magia*.
  - *Paladino:* HUD com pool numérico de *Cura pelas Mãos* (`nível x 5` pontos gastos/restantes), *Sentido Divino*, gatilho de *Destruição Divina (Divine Smite)* por slot e *Espaços de Magia* (1º ao 5º).
  - *Patrulheiro:* HUD de *Inimigo Favorito*, *Explorador Natural*, *Estilo de Combate*, controle de *Marca do Caçador* e *Espaços de Magia* (1º ao 5º).
- [x] **T6** — **Telas dos Conjuradores Arcanos e de Pacto (`Mago`, `Feiticeiro`, `Bruxo`, `Bardo`):**
  - *Mago:* HUD dividido entre *Grimório Completo* (magias no livro e rituais) vs. *Magias Preparadas* (`INT + nível`), *Recuperação Arcana* e *Espaços de Magia* (1º ao 9º).
  - *Feiticeiro:* HUD de *Pontos de Feitiçaria* (usados/restantes), conversor *Pontos ↔ Espaços de Magia*, seletores de *Metamagia* acoplados à conjuração e *Espaços de Magia*.
  - *Bruxo:* HUD exclusivo de *Magia de Pacto* (slots de nível único recarregáveis em descanso curto), *Invocações Místicas* ativas, *Dádiva do Pacto* e *Patrono*.
  - *Bardo:* HUD de dados de *Inspiração de Bardo* (usados/restantes), *Canção de Descanso*, *Aptidão (Expertise)*, Magias Conhecidas e *Espaços de Magia*.

### Fase 4: Integracao com o Servidor Rust e Arbitragem da IA
- [x] **T7** — **Integração API Backend (`Axum`) + Frontend:** Expor endpoints HTTP no backend em Rust para criação de personagem, sincronização do estado de turno (`CampaignState`) e processamento das 3 vias de ação no turno (conectando a entrada textual livre ao orquestrador IA + resolvedor mecânico determinístico).
