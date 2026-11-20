use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum EntityType {
    Npc,
    Player,
    Location,
    Object,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityDndStats {
    pub armor_class: i32,
    pub max_hp: i32,
    pub current_hp: i32,
    pub temporary_hp: i32,
    pub ability_scores: HashMap<String, i32>, // STR, DEX, CON, INT, WIS, CHA
    pub proficiencies: Vec<String>,
    pub spell_slots_total: HashMap<u8, u8>,
    pub spell_slots_used: HashMap<u8, u8>,
    pub conditions: Vec<String>,
    pub class_resources: HashMap<String, i32>,
}

impl Default for EntityDndStats {
    fn default() -> Self {
        let mut scores = HashMap::new();
        scores.insert("STR".to_string(), 10);
        scores.insert("DEX".to_string(), 10);
        scores.insert("CON".to_string(), 10);
        scores.insert("INT".to_string(), 10);
        scores.insert("WIS".to_string(), 10);
        scores.insert("CHA".to_string(), 10);

        Self {
            armor_class: 10,
            max_hp: 10,
            current_hp: 10,
            temporary_hp: 0,
            ability_scores: scores,
            proficiencies: Vec::new(),
            spell_slots_total: HashMap::new(),
            spell_slots_used: HashMap::new(),
            conditions: Vec::new(),
            class_resources: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TargetSpecificModifier {
    pub target_actor_id: String,
    pub dc_adjustment: i32,
    pub dice_mode: Option<crate::dice::DiceMode>,
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryImpactRecord {
    pub id: String,
    pub source_event_id: String, // Ponteiro para o chronicle_log.jsonl (ex: evt_0001)
    pub timestamp: String,
    pub description: String,
    pub psychological_impact: String,
    pub target_modifiers: Vec<TargetSpecificModifier>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldEntitySheet {
    pub id: String,
    pub name: String,
    pub entity_type: EntityType,
    pub description: String,
    pub dnd_stats: Option<EntityDndStats>,
    pub memory_impacts: Vec<MemoryImpactRecord>,
    pub custom_flags: HashMap<String, serde_json::Value>,
}

impl WorldEntitySheet {
    pub fn new(id: &str, name: &str, entity_type: EntityType, description: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            entity_type,
            description: description.to_string(),
            dnd_stats: None,
            memory_impacts: Vec::new(),
            custom_flags: HashMap::new(),
        }
    }

    pub fn add_impact(
        &mut self,
        source_event_id: &str,
        description: &str,
        psychological_impact: &str,
        target_modifiers: Vec<TargetSpecificModifier>,
    ) {
        let impact = MemoryImpactRecord {
            id: uuid::Uuid::new_v4().to_string(),
            source_event_id: source_event_id.to_string(),
            timestamp: Utc::now().to_rfc3339(),
            description: description.to_string(),
            psychological_impact: psychological_impact.to_string(),
            target_modifiers,
        };
        self.memory_impacts.push(impact);
    }

    /// Calcula os modificadores acumulados quando interagindo com um ator específico
    pub fn get_modifiers_against(&self, actor_id: &str) -> (i32, Option<crate::dice::DiceMode>) {
        let mut total_dc_adj = 0;
        let mut dice_mode = None;

        for impact in &self.memory_impacts {
            for modifier in &impact.target_modifiers {
                if modifier.target_actor_id == actor_id || modifier.target_actor_id == "*" {
                    total_dc_adj += modifier.dc_adjustment;
                    if let Some(mode) = modifier.dice_mode {
                        dice_mode = Some(mode);
                    }
                }
            }
        }

        (total_dc_adj, dice_mode)
    }
}

pub struct EntityStore {
    world_dir: PathBuf,
    players_dir: PathBuf,
}

impl EntityStore {
    pub fn new<P: AsRef<Path>>(campaign_dir: P) -> Result<Self, Error> {
        let base = campaign_dir.as_ref();
        let world_dir = base.join("entities").join("world");
        let players_dir = base.join("entities").join("players");

        fs::create_dir_all(&world_dir)?;
        fs::create_dir_all(&players_dir)?;

        Ok(Self {
            world_dir,
            players_dir,
        })
    }

    fn save_atomic<P: AsRef<Path>>(path: P, content: &str) -> Result<(), Error> {
        let p = path.as_ref();
        let mut temp_path = p.to_path_buf();
        temp_path.set_extension("tmp");

        let mut file = File::create(&temp_path)?;
        file.write_all(content.as_bytes())?;
        file.sync_all()?;

        fs::rename(temp_path, p)?;
        Ok(())
    }

    pub fn save_world_entity(&self, entity: &WorldEntitySheet) -> Result<(), Error> {
        let path = self.world_dir.join(format!("{}.json", entity.id));
        let content = serde_json::to_string_pretty(entity)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        Self::save_atomic(path, &content)
    }

    pub fn get_world_entity(&self, id: &str) -> Result<Option<WorldEntitySheet>, Error> {
        let path = self.world_dir.join(format!("{}.json", id));
        if !path.exists() {
            return Ok(None);
        }
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let entity = serde_json::from_str(&content)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        Ok(Some(entity))
    }

    pub fn list_world_entities(&self) -> Result<Vec<WorldEntitySheet>, Error> {
        let mut entities = Vec::new();
        if !self.world_dir.exists() {
            return Ok(entities);
        }

        for entry in fs::read_dir(&self.world_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(mut file) = File::open(&path) {
                    let mut content = String::new();
                    if file.read_to_string(&mut content).is_ok() {
                        if let Ok(entity) = serde_json::from_str::<WorldEntitySheet>(&content) {
                            entities.push(entity);
                        }
                    }
                }
            }
        }
        Ok(entities)
    }

    pub fn save_player_entity(&self, entity: &WorldEntitySheet) -> Result<(), Error> {
        let path = self.players_dir.join(format!("{}.json", entity.id));
        let content = serde_json::to_string_pretty(entity)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        Self::save_atomic(path, &content)
    }

    pub fn get_player_entity(&self, id: &str) -> Result<Option<WorldEntitySheet>, Error> {
        let path = self.players_dir.join(format!("{}.json", id));
        if !path.exists() {
            return Ok(None);
        }
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let entity = serde_json::from_str(&content)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        Ok(Some(entity))
    }

    pub fn list_player_entities(&self) -> Result<Vec<WorldEntitySheet>, Error> {
        let mut entities = Vec::new();
        if !self.players_dir.exists() {
            return Ok(entities);
        }

        for entry in fs::read_dir(&self.players_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(mut file) = File::open(&path) {
                    let mut content = String::new();
                    if file.read_to_string(&mut content).is_ok() {
                        if let Ok(entity) = serde_json::from_str::<WorldEntitySheet>(&content) {
                            entities.push(entity);
                        }
                    }
                }
            }
        }
        Ok(entities)
    }

    pub fn delete_player_entity(&self, id: &str) -> Result<(), Error> {
        let path = self.players_dir.join(format!("{}.json", id));
        if path.exists() {
            let _ = fs::remove_file(path);
        }
        Ok(())
    }

    pub fn clear_player_entities(&self) -> Result<(), Error> {
        if !self.players_dir.exists() {
            return Ok(());
        }
        for entry in fs::read_dir(&self.players_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let _ = fs::remove_file(path);
            }
        }
        Ok(())
    }
}

