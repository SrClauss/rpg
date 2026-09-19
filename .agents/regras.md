# Regras e Diretrizes para Implementação do RPG Engine

*(Arquivo de Regras de Negócio limpo a pedido do usuário. Novas regras e diretrizes acordadas durante as discussões `#regras` serão documentadas aqui.)*

### Abordagem Dupla para Equipamentos (SRD vs Homebrew)
- **Equipamentos SRD 5.1 (Oficiais)**: Implementados nativamente (hardcoded) em Rust (`src/rules/equipment/`), abrangendo todas as Armaduras, Armas, Equipamentos de Aventura, Ferramentas, Kits e Montarias do SRD.
- **Equipamentos Homebrew (Customizados)**: Suportados dinamicamente via registro (`EquipmentRegistry` / `load_json`), permitindo que usuários e mestres adicionem itens customizados via arquivos JSON sem recompilar o código.

### Eficiência de Contexto da IA DM (Evitar Estouro de Janela de Contexto)
- **Rust como Motor Determinístico**: O código em Rust executa toda a matemática, rolagens, validação de alcance e aplicação de condições. A IA DM não precisa guardar as regras brutas inteiras ou tabelas gigantes no seu System Prompt.
- **Injeção Sumarizada (`EncounterContext`)**: A IA DM recebe apenas o contexto filtrado do turno atual (`EncounterContext` e IDs dos itens disponíveis), garantindo prompts leves, respostas ultrarrápidas e custo mínimo de tokens.
