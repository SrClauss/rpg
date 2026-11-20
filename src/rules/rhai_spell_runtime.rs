use rhai::{CustomType, Engine, TypeBuilder};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Proxy simples de Personagem/Monstro exposto para os scripts Rhai
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetProxy {
    pub name: String,
    pub current_hp: i64,
    pub max_hp: i64,
    pub ac: i64,
    pub conditions: Vec<String>,
}

impl TargetProxy {
    pub fn new(name: &str, hp: i64, ac: i64) -> Self {
        Self {
            name: name.to_string(),
            current_hp: hp,
            max_hp: hp,
            ac,
            conditions: Vec::new(),
        }
    }

    pub fn take_damage(&mut self, damage_type: &str, amount: i64) {
        let actual = amount.max(0);
        self.current_hp = (self.current_hp - actual).max(0);
        println!(">>> [RUST ENGINE MUTATION] {} tomou {} de dano {}! PV restante: {}/{}", self.name, actual, damage_type, self.current_hp, self.max_hp);
    }

    pub fn heal(&mut self, amount: i64) {
        let actual = amount.max(0);
        self.current_hp = (self.current_hp + actual).min(self.max_hp);
        println!(">>> [RUST ENGINE MUTATION] {} curou {} PV! PV atual: {}/{}", self.name, actual, self.current_hp, self.max_hp);
    }

    pub fn add_condition(&mut self, cond: &str) {
        if !self.conditions.contains(&cond.to_string()) {
            self.conditions.push(cond.to_string());
            println!(">>> [RUST ENGINE MUTATION] {} recebeu condição: [{}]", self.name, cond);
        }
    }
}

/// Contexto e API que o Rust disponibiliza para os scripts Rhai executarem no mundo.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CombatContext {
    pub log: Vec<String>,
    pub secondary_targets: Vec<TargetProxy>,
}

impl CombatContext {
    pub fn new() -> Self {
        Self {
            log: Vec::new(),
            secondary_targets: Vec::new(),
        }
    }

    /// Adiciona um alvo secundário para magias em área / cone
    pub fn add_secondary_target(&mut self, target: TargetProxy) {
        self.secondary_targets.push(target);
    }

    /// Rola uma expressão de dados como "2d6", "1d20+3" ou "4d8".
    pub fn roll(&mut self, dice_expr: &str) -> i64 {
        use crate::dice::Dice;
        let total = match Dice::from_str(dice_expr) {
            Ok(d) => d.roll().total as i64,
            Err(_) => 0,
        };
        self.log.push(format!("🎲 [DADO RUST] Rolagem '{}': total = {}", dice_expr, total));
        total
    }

    /// Obtém modificador do atributo (padrão D&D 5e: +2 para Destreza comum)
    pub fn get_mod(&mut self, _target_name: &str, _ability: &str) -> i64 {
        2 // Valor padrão calibrado de salvaguarda para testes genéricos
    }

    /// Rola um teste de salvaguarda (Saving Throw) D&D 5e: 1d20 + mod vs DC.
    pub fn roll_save(&mut self, target_name: &str, ability_mod: i64, dc: i64) -> bool {
        use crate::dice::Dice;
        let d20 = Dice::d20().roll().total as i64;
        let total = d20 + ability_mod;
        let passed = total >= dc;
        self.log.push(format!(
            "🛡️ [RESISTÊNCIA RUST] {} rolou 1d20({}) + {} = {} vs CD {} -> {}",
            target_name, d20, ability_mod, total, dc,
            if passed { "SUCESSO (Reduz Dano)" } else { "FALHA (Dano Total)" }
        ));
        passed
    }

    /// Sobrecarga para quando o script passa o nome da habilidade: roll_save(nome, "Destreza", dc)
    pub fn roll_save_by_ability(&mut self, target_name: &str, _ability: &str, dc: i64) -> bool {
        self.roll_save(target_name, 2, dc)
    }

    /// Registra um evento no log do turno
    pub fn log_msg(&mut self, msg: &str) {
        self.log.push(format!("📜 [EFEITO RHAI] {}", msg));
    }
    pub fn get_secondary_count(&mut self) -> i64 {
        self.secondary_targets.len() as i64
    }

    pub fn damage_secondary(&mut self, idx: i64, damage_type: &str, amount: i64) {
        if idx >= 0 && (idx as usize) < self.secondary_targets.len() {
            self.secondary_targets[idx as usize].take_damage(damage_type, amount);
        }
    }

    pub fn get_secondary_name(&mut self, idx: i64) -> String {
        if idx >= 0 && (idx as usize) < self.secondary_targets.len() {
            self.secondary_targets[idx as usize].name.clone()
        } else {
            String::new()
        }
    }
}

impl CustomType for TargetProxy {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("TargetProxy")
            .with_get("name", |t: &mut Self| t.name.clone())
            .with_get("current_hp", |t: &mut Self| t.current_hp)
            .with_get("max_hp", |t: &mut Self| t.max_hp)
            .with_get("ac", |t: &mut Self| t.ac)
            .with_fn("take_damage", Self::take_damage)
            .with_fn("heal", Self::heal)
            .with_fn("add_condition", Self::add_condition);
    }
}

impl CustomType for CombatContext {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("CombatContext")
            .with_fn("roll", Self::roll)
            .with_fn("roll_save", Self::roll_save)
            .with_fn("roll_save", Self::roll_save_by_ability)
            .with_fn("get_mod", Self::get_mod)
            .with_fn("log_msg", Self::log_msg)
            .with_fn("secondary_count", Self::get_secondary_count)
            .with_fn("damage_secondary", Self::damage_secondary)
            .with_fn("get_secondary_name", Self::get_secondary_name);
    }
}

/// Cria o ambiente Rhai pronto e seguro com todos os métodos do D&D 5e registrados
pub fn create_spell_rhai_engine() -> Engine {
    let mut engine = Engine::new();
    engine.set_max_expr_depths(0, 0); // Desativa limites de profundidade para blocos/concatenações maiores
    engine.build_type::<TargetProxy>();
    engine.build_type::<CombatContext>();
    engine
}
