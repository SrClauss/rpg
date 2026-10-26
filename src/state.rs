use crate::map::grid::BattleGrid;
use crate::narrative::beat::{Beat, Premise};
use crate::narrative::chronicle::ChronicleBook;
use crate::narrative::entity_store::EntityStore;
use crate::narrative::graph::{NarrativeEdge, NarrativeNode};
use crate::narrative::living_scene::LivingScene;
use crate::narrative::memory::ContextualMemory;
use crate::narrative::skeleton::NarrativeSkeleton;
use crate::narrative::telemetry::{EntityStats, EventLog};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PlayerSlot {
    pub player_number: usize, // 1 (Host), 2, 3, 4
    pub player_id: String,
    pub player_name: String,
    pub class_name: String,
    pub player_code: String, // Código de 4 letras exclusivo do jogador
    pub is_host: bool,
}

pub fn generate_4letter_room_code() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let letters = b"ABCDEFGHJKLMNPQRSTUVWXYZ"; // sem I e O para máxima legibilidade
    (0..4)
        .map(|_| letters[rng.gen_range(0..letters.len())] as char)
        .collect()
}

pub fn generate_4letter_player_code() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let letters = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    (0..4)
        .map(|_| letters[rng.gen_range(0..letters.len())] as char)
        .collect()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CampaignManifest {
    pub campaign_id: String,
    pub room_code: String,
    pub host_player_id: String,
    pub player_ids: Vec<String>,
    #[serde(default)]
    pub player_slots: Vec<PlayerSlot>,
    pub premise: String,
    pub literary_style: String,
    pub hidden_skeleton: NarrativeSkeleton,
    pub created_at: String,
    pub status: String, // "lobby", "active", "completed"
    #[serde(default)]
    pub custom_monsters: Vec<serde_json::Value>,
    #[serde(default)]
    pub world_prologue: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CampaignState {
    pub campaign_id: String,
    pub room_code: String,
    pub manifest: Option<CampaignManifest>,
    pub active_scene: Option<LivingScene>,
    #[serde(default)]
    pub world_prologue: Option<String>,

    // 1. Estado Tático
    pub battle_grid: Option<BattleGrid>,

    // 2. Estado Narrativo e Grafos
    pub nodes: Vec<NarrativeNode>,
    pub edges: Vec<NarrativeEdge>,

    // 3. Histórico e Memória
    pub contextual_memory: Vec<ContextualMemory>,
    pub event_logs: Vec<EventLog>,

    // 4. Estatísticas de Partida
    pub entity_stats: HashMap<String, EntityStats>,

    // 5. Premissa e Beats (Motor Narrativo Legado / Compatibilidade)
    pub premise: Option<Premise>,
    pub beats: Vec<Beat>,
    pub current_beat_index: u32,
}

impl Default for CampaignState {
    fn default() -> Self {
        Self::new()
    }
}

impl CampaignState {
    pub fn new() -> Self {
        let campaign_id = uuid::Uuid::new_v4().to_string();
        let room_code = generate_4letter_room_code();

        Self {
            campaign_id,
            room_code,
            manifest: None,
            active_scene: None,
            world_prologue: None,
            battle_grid: None,
            nodes: Vec::new(),
            edges: Vec::new(),
            contextual_memory: Vec::new(),
            event_logs: Vec::new(),
            entity_stats: HashMap::new(),
            premise: None,
            beats: Vec::new(),
            current_beat_index: 0,
        }
    }

    pub fn with_id_and_room(campaign_id: &str, room_code: &str) -> Self {
        Self {
            campaign_id: campaign_id.to_string(),
            room_code: room_code.to_string(),
            manifest: None,
            active_scene: None,
            world_prologue: None,
            battle_grid: None,
            nodes: Vec::new(),
            edges: Vec::new(),
            contextual_memory: Vec::new(),
            event_logs: Vec::new(),
            entity_stats: HashMap::new(),
            premise: None,
            beats: Vec::new(),
            current_beat_index: 0,
        }
    }

    pub fn campaign_dir(&self) -> PathBuf {
        PathBuf::from("data").join("campaigns").join(&self.campaign_id)
    }

    pub fn init_campaign_dir(&self) -> Result<PathBuf, Error> {
        let dir = self.campaign_dir();
        fs::create_dir_all(&dir)?;
        fs::create_dir_all(dir.join("entities").join("world"))?;
        fs::create_dir_all(dir.join("entities").join("players"))?;
        Ok(dir)
    }

    pub fn save_manifest(&self) -> Result<(), Error> {
        let dir = self.init_campaign_dir()?;
        let path = dir.join("manifest.json");

        if let Some(ref manifest) = self.manifest {
            let content = serde_json::to_string_pretty(manifest)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
            Self::atomic_write(path, &content)?;
        }
        Ok(())
    }

    pub fn load_manifest(&mut self) -> Result<Option<CampaignManifest>, Error> {
        let path = self.campaign_dir().join("manifest.json");
        if !path.exists() {
            return Ok(None);
        }
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let manifest: CampaignManifest = serde_json::from_str(&content)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        self.manifest = Some(manifest.clone());
        Ok(Some(manifest))
    }

    pub fn save_active_scene(&self) -> Result<(), Error> {
        let dir = self.init_campaign_dir()?;
        let path = dir.join("active_scene.json");

        if let Some(ref scene) = self.active_scene {
            let content = serde_json::to_string_pretty(scene)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
            Self::atomic_write(path, &content)?;
        } else if path.exists() {
            let _ = fs::remove_file(path);
        }
        Ok(())
    }

    pub fn load_active_scene(&mut self) -> Result<Option<LivingScene>, Error> {
        let path = self.campaign_dir().join("active_scene.json");
        if !path.exists() {
            self.active_scene = None;
            return Ok(None);
        }
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let scene: LivingScene = serde_json::from_str(&content)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        self.active_scene = Some(scene.clone());
        Ok(Some(scene))
    }

    pub fn get_entity_store(&self) -> Result<EntityStore, Error> {
        self.init_campaign_dir()?;
        EntityStore::new(self.campaign_dir())
    }

    pub fn get_chronicle_book(&self) -> Result<ChronicleBook, Error> {
        self.init_campaign_dir()?;
        ChronicleBook::new(self.campaign_dir())
    }

    fn atomic_write<P: AsRef<Path>>(path: P, content: &str) -> Result<(), Error> {
        let p = path.as_ref();
        let mut temp_path = p.to_path_buf();
        temp_path.set_extension("tmp");

        let mut file = File::create(&temp_path)?;
        file.write_all(content.as_bytes())?;
        file.sync_all()?;

        fs::rename(temp_path, p)?;
        Ok(())
    }

    /// Salva o estado inteiro da campanha em um arquivo binário usando `bincode`
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let encoded: Vec<u8> = bincode::serialize(&self)?;

        let mut temp_path = path.as_ref().to_path_buf();
        temp_path.set_extension("tmp");

        let mut file = File::create(&temp_path)?;
        file.write_all(&encoded)?;
        file.sync_all()?;

        fs::rename(temp_path, path)?;

        Ok(())
    }

    /// Carrega o estado da campanha a partir de um arquivo binário `.sav`
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let state: CampaignState = bincode::deserialize(&buffer)?;
        Ok(state)
    }

    /// Inicia um novo Beat na campanha e incrementa o índice atual
    pub fn start_new_beat(&mut self, title: &str, beat_type: crate::narrative::beat::BeatType) -> uuid::Uuid {
        self.current_beat_index += 1;
        let beat = Beat::new(self.current_beat_index, title, beat_type);
        let id = beat.id;
        self.beats.push(beat);
        id
    }

    pub fn current_beat_mut(&mut self) -> Option<&mut Beat> {
        self.beats.last_mut()
    }

    pub fn current_beat(&self) -> Option<&Beat> {
        self.beats.last()
    }

    pub fn record_mutation(
        &mut self,
        entity_name: &str,
        entity_type: crate::narrative::beat::EntityMutationType,
        field_changed: &str,
        old_value: &str,
        new_value: &str,
        reason: &str,
    ) -> Result<(), String> {
        let current_beat = self.beats.last_mut().ok_or_else(|| {
            "Não há nenhum Beat ativo na campanha para registrar mutações.".to_string()
        })?;

        let mutation = crate::narrative::beat::EntityMutation::new(
            entity_name,
            entity_type,
            current_beat.id,
            field_changed,
            old_value,
            new_value,
            reason,
        );

        current_beat.entity_mutations.push(mutation);
        Ok(())
    }

    pub fn close_current_beat(&mut self, summary: &str) -> Result<(), String> {
        let current_beat = self.beats.last_mut().ok_or_else(|| {
            "Não há nenhum Beat ativo para fechar.".to_string()
        })?;

        current_beat.summary = summary.to_string();
        Ok(())
    }

    pub fn get_entity_mutation_history(&self, entity_name: &str) -> Vec<&crate::narrative::beat::EntityMutation> {
        self.beats
            .iter()
            .flat_map(|b| b.entity_mutations.iter())
            .filter(|m| m.entity_name == entity_name)
            .collect()
    }
}
