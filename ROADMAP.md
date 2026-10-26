# ROADMAP & OBJETIVOS DO PROJETO: RPG ENGINE D&D 5E (RUST)

Este documento contém todos os objetivos e requisitos técnicos necessários para construir o motor do jogo RPG de mesa multiplayer (D&D 5e) operado por IA em Rust.

---

## 🎯 Objetivo Principal
Criar um servidor em Rust resiliente, de alta performance e tempo real que atue como a **Engine Determinística de Regras de D&D 5e** e **Motor Tático de Campo**, orquestrando partidas multiplayer via WebSockets e integrando um **Gerador Narrativo Procedural via LLM**.

---

## 🏛️ Arquitetura Geral do Sistema (Rust Stack)
- **Servidor HTTP & WebSockets**: `tokio` + `axum` (ou `actix-web`)
- **Serialização & Protocolo**: `serde`, `serde_json`
- **Geração de Números Aleatórios**: `rand` (com geradores CSPRNG determinísticos por semente de partida)
- **Integração com LLM**: `reqwest` (com async/await para streaming/JSON estruturado)
- **Gerenciamento de Estado de Jogo**: `tokio::sync::rwlock` ou arquitetura baseada em Atores (`actix` / `tokio::sync::mpsc`)

---

## 📋 Lista de Objetivos do Sistema

### Fase 1: Motor Tático & Posicionamento no Campo (Grid D&D 5e) - *Foco Inicial*
- [x] **Estrutura de Dados do Grid**: Matriz de coordenadas 2D `(x, y)` representando células de 5 pés ($1.5\text{m}$).
- [x] **Movimentação & Deslocamento**: Sistema em Rust para validar se um token possui deslocamento suficiente no seu turno (considerando terreno difícil = 10 pés por célula).
- [x] **Cálculo de Distância**: Implementação do cálculo de distância de Chebyshev e regra D&D 5e ($1^{\text{a}}$ diagonal = 5ft, $2^{\text{a}}$ diagonal = 10ft).
- [x] **Linha de Visão (Line of Sight - LoS)**: Algoritmo de raycasting/Bresenham em Rust para detectar obstrução entre células e determinar cobertura parcial (+2 CA), três-quartos (+5 CA) ou total.
- [ ] **Áreas de Efeito (AoE)**: Cálculo de máscaras de área para Cones, Esferas, Cilindros e Linhas no grid.
- [ ] **Disparo de Ataques de Oportunidade**: Evento disparado no servidor Rust quando um token sai do raio de alcance de um inimigo sem a ação de Desengajar.

### Fase 2: Motor Determinístico de Regras D&D 5e
- [x] **Sistema de Rolagem de Dados Pseudo-Aleatórias**: Módulo de dados (`d4`, `d6`, `d8`, `d10`, `d12`, `d20`, `d100`) com suporte a Vantagem / Desvantagem e modificadores.
- [x] **Estrutura da Ficha de Personagem (Pydantic-like via Serde/Structs Rust)**: Atributos (`STR`, `DEX`, `CON`, `INT`, `WIS`, `CHA`), Vida (`HP`), Classe de Armadura (`CA`), Classe de Dificuldade (`CD`), Perícias e Inventário.
- [x] **Gerenciador de Turnos e Iniciativa**: Fila de prioridade ordenando jogadores e NPCs por rolagem de iniciativa (`d20 + MOD_DEX`).
- [x] **Módulo de Resolução de Combate**: Validação estrita de `d20 + MOD_ATAQUE >= CA` e aplicação exata de danos e tipos de dano (Cortante, Perfurante, Fogo, etc.).

### Fase 3: Gerador Narrativo Procedural & Mestre IA
- [ ] **Inicializador de Campanha**: Endpoint em Rust que consome a premissa inicial enviada pelos jogadores e solicita ao LLM a geração em JSON da introdução, ambiente e NPCs iniciais.
- [x] **Tradutor de Eventos Numéricos em Narrativa**: Converter os dados brutos calculados pelo motor em Rust (ex: `Crítico! 24 de dano de fogo`) em um prompt estruturado para o LLM gerar a descrição dramática da cena.
- [x] **Memória de Estado & Bíblia do Mundo**: Sistema de armazenamento estruturado em banco de dados para evitar alucinações e manter a consistência da lore ao longo de sessões.

### Fase 4: Comunicação Multiplayer em Tempo Real (WebSockets Tokio/Axum)
- [ ] **Handshake & Autenticação de Salas**: Permitir que múltiplos jogadores humanos entrem na mesma sessão de jogo via código de sala.
- [ ] **Protocolo de Mensagens em JSON**: Tipagem estrita de mensagens WebSocket para:
  - Intenção de Movimento (`MOVE_TOKEN`)
  - Intenção de Ataque (`ATTACK_TARGET`)
  - Uso de Habilidade/Magia (`CAST_SPELL`)
  - Atualização de Estado (`STATE_SYNC`)
- [ ] **Sincronização de Estado Broadcast**: Envio imediato das atualizações do mapa e status de vida para todos os clientes conectados.

---

## 🚀 Próximo Passo Sugerido
Começar a implementação da **Fase 1 (Motor Tático & Posicionamento no Campo)** criando o módulo de coordenadas e grid em Rust.
