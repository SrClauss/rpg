# Plano de Implementação: Validador de Equipamentos Homebrew e Efeitos Narrativos (`validator.rs`)

## Objetivo
Implementar o módulo `validator.rs` para validação de equipamentos Homebrew (JSON). O validador diferencia **Efeitos Mecânicos Estruturados** (validados e calculados 100% no Rust) de **Efeitos Narrativos/Customizados em Texto Livre** (delegados para interpretação da IA DM), garantindo que nenhum JSON malformado ou efeito criativo quebre o motor do jogo.

## Estrutura do Validador em Rust (`HomebrewValidator`)

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidatedEffect {
    /// Efeito mecânico nativo que o Rust calcula (ex: Bônus de Dano, Condição D&D, Cura, Salvaguarda)
    Mechanical(SpecialEffect),
    /// Efeito narrativo/customizado que o Rust repassa para a IA DM interpretar (ex: Amnésia, Viagem no Tempo, Ilusões)
    NarrativeCustom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub item_id: String,
    pub validated_effect: ValidatedEffect,
    pub warnings: Vec<String>,
}
```

## Checklist de Subtarefas

- [x] **Subtarefa 1: Implementar `HomebrewValidator` e `ValidatedEffect` em `src/rules/equipment/validator.rs` com testes**
  - Criar `src/rules/equipment/validator.rs`.
  - Implementar lógica de validação de dados de dano, salvaguardas, condições e classificação de efeitos narrativos.
  - Exportar no módulo em `src/rules/equipment/mod.rs`.
  - Integrar com `EquipmentRegistry::validate_and_register_homebrew`.
  - Escrever testes unitários em `validator.rs` para itens mecânicos e itens narrativos customizados.

## Subtarefa Atual
**Subtarefa 1: Implementar `HomebrewValidator` e `ValidatedEffect` em `src/rules/equipment/validator.rs` com testes**
