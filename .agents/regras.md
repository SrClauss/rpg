# Mapeamento e Especificação de Regras: SRD 5.1 (Páginas 76 - 99)

Este documento especifica a representação determinística em Rust das regras contidas nas páginas 76 a 99 do documento SRD 5.1 (`SRD-OGL_V5.1.pdf`).

---

## 1. Usando Valores de Habilidade (Páginas 76–83)
- **Testes de Atributo (Ability Checks)**: $1\text{d}20 + \text{Modificador de Atributo}$.
- **Salvaguardas (Saving Throws)**: $1\text{d}20 + \text{Modificador} + (\text{Proficiência se aplicável})$.
- **Perícias (Skills)**: $1\text{d}20 + \text{Modificador do Atributo Base} + (\text{Proficiência/Especialização})$.
- **Contestes (Contests)**: Dois testes comparados diretamente; empates mantêm a situação anterior.
- **Valores Passivos**: $10 + \text{Modificador} + \text{Proficiência} \pm 5$ (Vantagem/Desvantagem).
- **Capacidade Física (Força)**:
  - Carga: $\text{FOR} \times 15$ lbs.
  - Empurrar/Puxar/Levantar: $\text{FOR} \times 30$ lbs.

---

## 2. Aventurando-se (Páginas 84–89)
- **Movimento e Salto**:
  - Salto em Distância: Pés equivalentes ao valor de $\text{FOR}$ (com 10ft de corrida prévia).
  - Salto em Altura: $3 + \text{modificador de FOR}$ em pés.
- **Riscos Ambientais**:
  - **Queda**: $1\text{d}6$ de dano de concussão por cada 10 pés ($3\text{m}$) de queda (máximo $20\text{d}6$). Criatura cai `Prone`.
  - **Asfixia/Sufocamento**: $1 + \text{mod CON}$ minutos retendo a respiração. Após zerar o ar, sobrevive por rodadas = $\text{mod CON}$ (mínimo 1), caindo para 0 PV no início do próximo turno.
  - **Visão e Luz**: Escuridão total aplica condição `Blinded`. Penumbra concede Desvantagem em testes de Percepção (visuais).
- **Descansos**:
  - **Short Rest (1 hora)**: Gastar Dados de Vida ($\text{Hit Dice} + \text{mod CON}$) para recuperar PV.
  - **Long Rest (8 horas)**: Recupera todos os PV e metade do total de Dados de Vida (máximo 1 vez a cada 24 horas).

---

## 3. Combate (Páginas 90–99)
- **Ações no Turno**: `Attack`, `CastSpell`, `Dash`, `Disengage`, `Dodge`, `Help`, `Hide`, `Ready`, `Search`, `UseObject`.
- **Manobras de Ataque**:
  - **Agarrar (Grapple)**: Teste de Atletismo vs Atletismo ou Acrobacia do alvo. Aplica condição `Grappled` (deslocamento 0).
  - **Empurrar (Shove)**: Teste de Atletismo vs Atletismo ou Acrobacia do alvo. Derruba `Prone` ou empurra 5 pés.
- **Cobertura (Cover)**:
  - Meia Cobertura (+2 CA / +2 DEX save).
  - Três-Quartos (+5 CA / +5 DEX save).
  - Cobertura Total (não pode ser alvo direto).
- **Vida, Morte e Salvaguardas**:
  - **Morte Instantânea**: Dano restante $\ge$ PV Máximo.
  - **Death Saving Throws**: $1\text{d}20$ sem modificadores. $\ge 10$ sucesso, $< 10$ falha. $1$ natural = 2 falhas. $20$ natural = recupera 1 PV e levanta imediatamente. 3 sucessos = estabilizado. 3 falhas = morte.
