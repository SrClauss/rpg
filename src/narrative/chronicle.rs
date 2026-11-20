use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, ErrorKind, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ChronicleEntryType {
    LiteraryChapter,
    LivingSceneSummary,
    AutonomousEvent,
    MechanicalAction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChronicleEntry {
    pub event_id: String,
    pub timestamp: String,
    pub chapter: u32,
    pub scene_title: String,
    pub entry_type: ChronicleEntryType,
    pub narrative_prose: String,
    pub rules_resolutions: Vec<serde_json::Value>,
    pub involved_entities: Vec<String>,
}

impl ChronicleEntry {
    pub fn new(
        chapter: u32,
        scene_title: &str,
        entry_type: ChronicleEntryType,
        narrative_prose: &str,
        rules_resolutions: Vec<serde_json::Value>,
        involved_entities: Vec<String>,
    ) -> Self {
        Self {
            event_id: String::new(), // preenchido ao anexar
            timestamp: Utc::now().to_rfc3339(),
            chapter,
            scene_title: scene_title.to_string(),
            entry_type,
            narrative_prose: narrative_prose.to_string(),
            rules_resolutions,
            involved_entities,
        }
    }
}

pub struct ChronicleBook {
    log_file_path: PathBuf,
    offset_index: HashMap<String, u64>,
    next_event_number: u64,
}

impl ChronicleBook {
    pub fn new<P: AsRef<Path>>(campaign_dir: P) -> Result<Self, Error> {
        let dir = campaign_dir.as_ref();
        if !dir.exists() {
            std::fs::create_dir_all(dir)?;
        }

        let log_file_path = dir.join("chronicle_log.jsonl");
        let mut offset_index = HashMap::new();
        let mut max_event_number: u64 = 0;

        if log_file_path.exists() {
            let file = File::open(&log_file_path)?;
            let mut reader = BufReader::new(file);
            let mut current_offset = 0u64;
            let mut line = String::new();

            loop {
                let bytes_read = reader.read_line(&mut line)?;
                if bytes_read == 0 {
                    break;
                }

                if let Ok(entry) = serde_json::from_str::<ChronicleEntry>(&line) {
                    if !entry.event_id.is_empty() {
                        offset_index.insert(entry.event_id.clone(), current_offset);
                        if let Some(num_str) = entry.event_id.strip_prefix("evt_") {
                            if let Ok(num) = num_str.parse::<u64>() {
                                if num > max_event_number {
                                    max_event_number = num;
                                }
                            }
                        }
                    }
                }

                current_offset += bytes_read as u64;
                line.clear();
            }
        }

        Ok(Self {
            log_file_path,
            offset_index,
            next_event_number: max_event_number + 1,
        })
    }

    pub fn append_entry(&mut self, entry: &mut ChronicleEntry) -> Result<String, Error> {
        if entry.event_id.is_empty() {
            entry.event_id = format!("evt_{:04}", self.next_event_number);
            self.next_event_number += 1;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file_path)?;

        let current_offset = file.seek(SeekFrom::End(0))?;
        let json_line = serde_json::to_string(&entry)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        writeln!(file, "{}", json_line)?;
        file.flush()?;

        self.offset_index.insert(entry.event_id.clone(), current_offset);

        Ok(entry.event_id.clone())
    }

    /// Busca um evento específico diretamente pelo offset no arquivo em O(1)
    pub fn get_entry_by_id(&self, event_id: &str) -> Result<Option<ChronicleEntry>, Error> {
        let offset = match self.offset_index.get(event_id) {
            Some(off) => *off,
            None => return Ok(None),
        };

        let mut file = File::open(&self.log_file_path)?;
        file.seek(SeekFrom::Start(offset))?;

        let mut reader = BufReader::new(file);
        let mut line = String::new();
        reader.read_line(&mut line)?;

        let entry: ChronicleEntry = serde_json::from_str(&line)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        Ok(Some(entry))
    }

    /// Retorna todas as entradas sequenciais da crônica (para exibição inicial do livro)
    pub fn get_all_entries(&self) -> Result<Vec<ChronicleEntry>, Error> {
        if !self.log_file_path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.log_file_path)?;
        let reader = BufReader::new(file);
        let mut entries = Vec::new();

        for line_result in reader.lines() {
            let line = line_result?;
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(entry) = serde_json::from_str::<ChronicleEntry>(&line) {
                entries.push(entry);
            }
        }

        Ok(entries)
    }

    /// Retorna as N entradas mais recentes utilizando busca direta por offset em O(count)
    pub fn get_recent_entries(&self, count: usize) -> Result<Vec<ChronicleEntry>, Error> {
        if !self.log_file_path.exists() || count == 0 || self.offset_index.is_empty() {
            return Ok(Vec::new());
        }

        // Ordena os event_ids para identificar as últimas N entradas registradas
        let mut event_ids: Vec<&String> = self.offset_index.keys().collect();
        event_ids.sort();

        let start_idx = event_ids.len().saturating_sub(count);
        let target_ids = &event_ids[start_idx..];

        let mut entries = Vec::with_capacity(target_ids.len());
        for id in target_ids {
            if let Some(entry) = self.get_entry_by_id(id)? {
                entries.push(entry);
            }
        }

        Ok(entries)
    }
}

