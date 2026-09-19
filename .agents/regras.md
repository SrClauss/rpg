# Regras e Diretrizes para Implementação do RPG Engine

*(Arquivo de Regras de Negócio limpo a pedido do usuário. Novas regras e diretrizes acordadas durante as discussões `#regras` serão documentadas aqui.)*

### Abordagem Dupla para Equipamentos (SRD vs Homebrew)
- **Equipamentos SRD 5.1 (Oficiais)**: Implementados nativamente (hardcoded) em Rust (`src/rules/equipment/`), abrangendo todas as Armaduras, Armas, Equipamentos de Aventura, Ferramentas, Kits e Montarias do SRD.
- **Equipamentos Homebrew (Customizados)**: Suportados dinamicamente via registro (`EquipmentRegistry` / `load_json`), permitindo que usuários e mestres adicionem itens customizados via arquivos JSON sem recompilar o código.
