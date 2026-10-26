# Plano de Implementação — Motor de Campanha Procedural, Modo Offline (Hot-Seat), Chat da Party, Modal de Auditoria e Persistência de Premissas/Campanhas

## Objetivo
Expandir a arquitetura do RPG D&D 5e unindo o modo multiplayer via WebSockets (com Chat da Party em tempo real) a um **Modo Offline Local (Hot-Seat)** para jogar na mesma máquina e facilitar testes rápidos. Incluir um **Modal de Auditoria** (botão fixo no canto inferior esquerdo) para inspecionar personagens, entidades do mundo e o esqueleto/crônica da história, além de **Exportação/Importação de Premissas (JSON / Banco)** e **Save/Resume Externo de Campanhas**.

---

## Diretrizes de Execução
- **Liberdade Total:** O agente tem total autonomia para decidir como organizar o código, escolher arquiteturas internas, executar quantas tarefas quiser por turno e utilizar subagentes conforme achar necessário.
- **Regras de Processo Descartadas:** Não há restrição de 1 tarefa por turno, tags ou aprovações intermediárias. O agente pode iterar livremente até a conclusão.
- **Instruções Diretas para Modelos Menores:** Cada tarefa abaixo possui escopo claro, arquivos exatos e entradas/saídas bem definidas para execução simples e sem ambiguidade.

---

## Fase 1 — Concluída (Base do Motor e Cena Viva)
- [x] **T1 a T10 — Motor Base, Esqueleto Oculto, Cena Viva, Entidades e Lobby Inicial.**

---

## Fase 2 — Novas Tarefas (Chat, Offline Hot-Seat, Auditoria e Saves Externos)

- [x] **T11 — Biblioteca e Exportação/Importação de Premissas (`src/bin/server.rs`, `SessionLobby.tsx` e `api.ts`):**
  - **Backend:** Criar rotas `GET /api/premises` e `POST /api/premises` que salvam e listam premissas reutilizáveis em `data/premises/<id>.json` (ou coleção MongoDB se `MONGODB_URI` estiver presente), contendo `title`, `premise`, `literary_style`, `ai_creative_freedom` e `custom_monsters`.
  - **Frontend:** No `SessionLobby.tsx`, adicionar botões *"💾 Salvar Premissa (JSON/DB)"* e *"📂 Carregar Premissa (.json ou Salvas)"*, permitindo baixar o arquivo `.json` da premissa para compartilhar com outras pessoas ou carregar um arquivo `.json` do disco.

- [x] **T12 — Gravação Externa e Retomada (Resume) de Campanhas (`src/bin/server.rs`, `SessionLobby.tsx` e `api.ts`):**
  - **Backend:** Criar rotas:
    - `GET /api/campaigns/saved`: Lista todas as campanhas salvas em `data/campaigns/` com resumo (ID, código, ato atual, premissa, data e jogadores).
    - `GET /api/campaign/{id}/export`: Retorna um pacote JSON completo (`CampaignBackupBundle`) contendo `manifest.json`, `active_scene.json`, `chronicle_log.jsonl` e todas as fichas de `entities/players` e `entities/world`.
    - `POST /api/campaign/import`: Recebe um `CampaignBackupBundle` (arquivo `.json` externo), restaura a pasta em `data/campaigns/<id>/`, registra na memória do servidor e permite retomar exatamente de onde parou.
  - **Frontend:** Adicionar aba/seção *"3. Retomar / Importar Campanha"* no `SessionLobby.tsx` para continuar campanhas salvas ou subir um arquivo `.json` de backup externo.

- [x] **T13 — Modo Offline Local / Hot-Seat no Mesma Máquina (`SessionLobby.tsx`, `App.tsx` e `BaseGameHUD.tsx`):**
  - **Fluxo:** No `SessionLobby.tsx`, permitir ativar a chave *"🖥️ Modo Offline / Local (Mesmo PC)"* e definir de 1 a 4 jogadores locais.
  - **Criação:** Permitir criar de 1 a 4 heróis em sequência (ou usar um botão *"⚡ Gerar Party de Teste Rápida"* para testes imediatos).
  - **Troca de Turno Local (Hot-Seat):** No `BaseGameHUD.tsx`, exibir no topo uma barra de alternância rápida entre os heróis locais (`[Herói 1] [Herói 2] ...`) e sincronizar automaticamente o cockpit com o jogador ativo da fila de iniciativa da Cena Viva.

- [x] **T14 — Chat da Party em Tempo Real no Canto da Tela (`src/bin/server.rs` e `frontend/src/components/game/PartyChatWidget.tsx`):**
  - **Backend:** No handler WebSocket (`ws_handler` em `src/bin/server.rs`) e rota `POST /api/campaign/{id}/chat`, receber mensagens de chat (`{ sender_id, sender_name, text, timestamp }`) e fazer broadcast imediato de `CHAT_MESSAGE` para todos na sala.
  - **Frontend:** Criar componente retrátil `PartyChatWidget.tsx` fixado no **canto inferior direito** do HUD, exibindo mensagens da party em tempo real, contador de mensagens não lidas e suporte tanto a salas WebSocket quanto ao modo local.

- [x] **T15 — Botão e Modal de Auditoria Completa (`src/bin/server.rs` e `frontend/src/components/game/AuditModal.tsx`):**
  - **Backend:** Criar rota `GET /api/campaign/{id}/audit` que retorna o pacote completo de inspeção: `manifest` (incluindo o `hidden_skeleton` completo, atos, marcos e `homebrew_bosses`), lista completa de `players`, lista completa de `world_entities` (com atributos, flags e ponteiros de memória), `active_scene` e todas as entradas de `chronicle`.
  - **Frontend:** Criar um botão discreto e fixo no **canto inferior esquerdo** (`🔍 Auditoria`) visível em qualquer tela do jogo. Ao clicar, abre o `AuditModal.tsx` dividido em 4 abas claras:
    1. **Heróis Criados:** Fichas completas, atributos, recursos de classe e JSON bruto.
    2. **Entidades & Bosses:** Monstros homebrew, NPCs, HP/CA atual e impactos de memória.
    3. **Esqueleto Oculto & Premissa:** Visualização dos atos planejados pela IA, eventos autônomos e diretrizes criativas.
    4. **Livro da Crônica & Cena Viva:** Log completo de eventos, rolagens e estado da cena ativa.
