use rand::Rng;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiceMode {
    #[serde(alias = "normal", alias = "Normal")]
    Normal,
    #[serde(alias = "advantage", alias = "vantagem", alias = "Advantage", alias = "Vantagem")]
    Advantage,
    #[serde(alias = "disadvantage", alias = "desvantagem", alias = "Disadvantage", alias = "Desvantagem")]
    Disadvantage,
}

impl DiceMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            DiceMode::Normal => "normal",
            DiceMode::Advantage => "advantage",
            DiceMode::Disadvantage => "disadvantage",
        }
    }
}

impl std::fmt::Display for DiceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for DiceMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "normal" => Ok(DiceMode::Normal),
            "advantage" | "vantagem" => Ok(DiceMode::Advantage),
            "disadvantage" | "desvantagem" => Ok(DiceMode::Disadvantage),
            other => Err(format!("DiceMode inválido: {}", other)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollResult {
    pub rolls: Vec<i32>,
    pub selected_roll: i32,
    pub modifier: i32,
    pub total: i32,
    pub is_critical_hit: bool,
    pub is_critical_fail: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dice {
    pub sides: i32,
    pub quantity: u8,
    pub modifier: i32,
    pub mode: DiceMode,
}

impl Dice {
    pub fn new(sides: i32, quantity: u8, modifier: i32, mode: DiceMode) -> Self {
        Self {
            sides,
            quantity,
            modifier,
            mode,
        }
    }

    pub fn d20() -> Self {
        Self::new(20, 1, 0, DiceMode::Normal)
    }

    pub fn roll(&self) -> RollResult {
        if self.sides <= 0 || self.quantity == 0 {
            return RollResult {
                rolls: vec![],
                selected_roll: 0,
                modifier: self.modifier,
                total: self.modifier,
                is_critical_hit: false,
                is_critical_fail: false,
            };
        }

        let mut rng = rand::thread_rng();

        let mut roll_single_set = |qty: u8, sides: i32| -> (Vec<i32>, i32) {
            let mut set = Vec::with_capacity(qty as usize);
            let mut sum = 0;
            for _ in 0..qty {
                let val = rng.gen_range(1..=sides);
                set.push(val);
                sum += val;
            }
            (set, sum)
        };

        match self.mode {
            DiceMode::Normal => {
                let (rolls, sum) = roll_single_set(self.quantity, self.sides);
                let is_crit_hit =
                    self.sides == 20 && self.quantity == 1 && rolls.first() == Some(&20);
                let is_crit_fail =
                    self.sides == 20 && self.quantity == 1 && rolls.first() == Some(&1);

                RollResult {
                    rolls,
                    selected_roll: sum,
                    modifier: self.modifier,
                    total: sum + self.modifier,
                    is_critical_hit: is_crit_hit,
                    is_critical_fail: is_crit_fail,
                }
            }
            DiceMode::Advantage => {
                let (rolls1, sum1) = roll_single_set(self.quantity, self.sides);
                let (rolls2, sum2) = roll_single_set(self.quantity, self.sides);

                let (selected_roll, selected_rolls) = if sum1 >= sum2 {
                    (sum1, rolls1.clone())
                } else {
                    (sum2, rolls2.clone())
                };

                let mut all_rolls = rolls1;
                all_rolls.extend(rolls2);

                let is_crit_hit = self.sides == 20 && selected_rolls.first() == Some(&20);
                let is_crit_fail = self.sides == 20 && selected_rolls.first() == Some(&1);

                RollResult {
                    rolls: all_rolls,
                    selected_roll,
                    modifier: self.modifier,
                    total: selected_roll + self.modifier,
                    is_critical_hit: is_crit_hit,
                    is_critical_fail: is_crit_fail,
                }
            }
            DiceMode::Disadvantage => {
                let (rolls1, sum1) = roll_single_set(self.quantity, self.sides);
                let (rolls2, sum2) = roll_single_set(self.quantity, self.sides);

                let (selected_roll, selected_rolls) = if sum1 <= sum2 {
                    (sum1, rolls1.clone())
                } else {
                    (sum2, rolls2.clone())
                };

                let mut all_rolls = rolls1;
                all_rolls.extend(rolls2);

                let is_crit_hit = self.sides == 20 && selected_rolls.first() == Some(&20);
                let is_crit_fail = self.sides == 20 && selected_rolls.first() == Some(&1);

                RollResult {
                    rolls: all_rolls,
                    selected_roll,
                    modifier: self.modifier,
                    total: selected_roll + self.modifier,
                    is_critical_hit: is_crit_hit,
                    is_critical_fail: is_crit_fail,
                }
            }
        }
    }
}

/// Suporte a parsing da notação NdX+M / NdX-M (ex: "1d20+5", "2d6+3", "1d8-1", "d20")
impl FromStr for Dice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cleaned = s.trim().to_lowercase();
        let parts: Vec<&str> = cleaned.split('d').collect();

        if parts.len() != 2 {
            return Err(format!("Notação de dado inválida: '{}'", s));
        }

        let quantity: u8 = if parts[0].is_empty() {
            1
        } else {
            parts[0]
                .parse()
                .map_err(|_| format!("Quantidade de dados inválida em '{}'", s))?
        };

        let rest = parts[1];
        let (sides_str, modifier) = if let Some(pos) = rest.find('+') {
            let (sides, modif) = rest.split_at(pos);
            let m: i32 = modif[1..]
                .parse()
                .map_err(|_| format!("Modificador inválido em '{}'", s))?;
            (sides, m)
        } else if let Some(pos) = rest.find('-') {
            let (sides, modif) = rest.split_at(pos);
            let m: i32 = modif[1..]
                .parse()
                .map_err(|_| format!("Modificador inválido em '{}'", s))?;
            (sides, -m)
        } else {
            (rest, 0)
        };

        let sides: i32 = sides_str
            .parse()
            .map_err(|_| format!("Lados do dado inválidos em '{}'", s))?;

        Ok(Dice::new(sides, quantity, modifier, DiceMode::Normal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_roll_range() {
        let d20 = Dice::new(20, 1, 5, DiceMode::Normal);
        for _ in 0..100 {
            let res = d20.roll();
            assert!((6..=25).contains(&res.total));
            assert_eq!(res.modifier, 5);
        }
    }

    #[test]
    fn test_advantage_roll() {
        let d20_adv = Dice::new(20, 1, 0, DiceMode::Advantage);
        for _ in 0..100 {
            let res = d20_adv.roll();
            assert!((1..=20).contains(&res.total));
            assert_eq!(res.rolls.len(), 2);
        }
    }

    #[test]
    fn test_disadvantage_roll() {
        let d20_dis = Dice::new(20, 1, -2, DiceMode::Disadvantage);
        for _ in 0..100 {
            let res = d20_dis.roll();
            assert!((-1..=18).contains(&res.total));
            assert_eq!(res.rolls.len(), 2);
        }
    }

    #[test]
    fn test_dice_parse_notation() {
        let dice1: Dice = "1d20+5".parse().unwrap();
        assert_eq!(
            dice1,
            Dice {
                sides: 20,
                quantity: 1,
                modifier: 5,
                mode: DiceMode::Normal
            }
        );

        let dice2: Dice = "2d6-1".parse().unwrap();
        assert_eq!(
            dice2,
            Dice {
                sides: 6,
                quantity: 2,
                modifier: -1,
                mode: DiceMode::Normal
            }
        );

        let dice3: Dice = "d20".parse().unwrap();
        assert_eq!(
            dice3,
            Dice {
                sides: 20,
                quantity: 1,
                modifier: 0,
                mode: DiceMode::Normal
            }
        );
    }

    #[test]
    fn test_critical_hit_and_fail() {
        let res_crit = RollResult {
            rolls: vec![20],
            selected_roll: 20,
            modifier: 3,
            total: 23,
            is_critical_hit: true,
            is_critical_fail: false,
        };
        assert!(res_crit.is_critical_hit);
        assert!(!res_crit.is_critical_fail);
    }
}
