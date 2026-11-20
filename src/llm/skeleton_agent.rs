use crate::llm::gemini::GeminiClient;
use crate::narrative::skeleton::{AutonomousWorldEvent, NarrativeSkeleton, SkeletonNode, StoryFocus};
use serde_json::Value;
use std::error::Error;
use std::str::FromStr;

pub struct SkeletonAgent {
    pub gemini: GeminiClient,
}

impl SkeletonAgent {
    pub fn new(api_key: String) -> Self {
        Self {
            gemini: GeminiClient::new(api_key),
        }
    }

    pub fn with_client(gemini: GeminiClient) -> Self {
        Self { gemini }
    }

    /// Gera o Esqueleto Oculto proceduralmente a partir da premissa, estilo, nível de liberdade criativa e foco narrativo (combate vs roleplay vs d20)
    pub async fn generate_skeleton(
        &self,
        premise: &str,
        literary_style: &str,
        ai_creative_freedom: &str,
        story_focus: &str,
    ) -> Result<NarrativeSkeleton, Box<dyn Error>> {
        let typed_focus = StoryFocus::from_str(story_focus).unwrap_or(StoryFocus::D20);
        let freedom_directive = match ai_creative_freedom {
            "strict" => "DIRETRIZ DE LIMITAÇÃO CRIATIVA RÍGIDA: A IA NÃO DEVE inventar monstros ou bosses homebrew próprios além dos explicitamente definidos pelo usuário na premissa. Foque estritamente no cânone da premissa e regras oficiais do SRD 5.1. Se não houver monstros fornecidos, use apenas criaturas oficiais e não invente novos bosses.",
            "free" => "DIRETRIZ DE LIBERDADE CRIATIVA TOTAL: A IA tem total autonomia artística e ousadia para inventar seus próprios Bosses Homebrew arrojados, reviravoltas cósmicas, mecânicas táticas únicas e eventos chocantes, expandindo ativamente a premissa com criatividade ilimitada.",
            _ => "DIRETRIZ DE LIBERDADE EQUILIBRADA (Padrão): A IA respeita fielmente os monstros e temas informados pelo usuário, mas pode introduzir ou adaptar 1 Boss Homebrew complementar com bom senso dramático se enriquecer a trama e fizer sentido temático.",
        };

        let focus_directive = match typed_focus {
            StoryFocus::Combat => "FOCO DA CAMPANHA: COMBATE TÁTICO & MASMORRAS (Hack & Slash / Tático). A história deve ter confrontos frequentes, masmorras perigosas, desafios físicos e chefes desafiadores em cada arco.",
            StoryFocus::Roleplay => "FOCO DA CAMPANHA: ROLEPLAY & INTRIGA (Narrativo & Social). A história foca em diálogos, investigação, diplomacia, segredos e dilemas morais. Evite batalhas desnecessárias; permita resolver conflitos sem sangue.",
            StoryFocus::D20 => "FOCO DA CAMPANHA: D20 DO DESTINO (Equilibrado / Sorte dos Dados). Para cada arco ou ato, role mentalmente 1d20: valores de 1 a 10 favorecem roleplay e investigação sutil; valores de 11 a 20 favorecem batalhas táticas e emboscadas viscerais.",
        };

        let system_instruction = format!(
            "Você é o Arquiteto Mestre de Campanhas de RPG D&D 5e (Gemini Flash-Lite). \
Sua função é gerar o ESQUELETO NARRATIVO OCULTO que guiará a campanha no servidor, totalmente oculto dos jogadores. \
ORGANIZAÇÃO EM ARCOS NARRATIVOS E BOSSES: \
A história DEVE ser dividida em ARCOS NARRATIVOS bem definidos (ex: Arco I, Arco II, Arco III...). Cada arco deve possuir seus atos e eventualmente um Chefe ou Mini-Boss temático para coroar o clímax daquele arco! \
{}\n\
DIRETRIZ DE ESCALONAMENTO DE 20 NÍVEIS E ORÇAMENTO DE XP (D&D 5E): \
Trabalhamos com o sistema completo de 20 Níveis do D&D 5e dividido em 4 Tiers de Jogo: \
- Tier 1 (Níveis 1-4, ND 1-4, ~200 a 1.100 XP por criatura): Conflitos locais e ameaças regionais. \
- Tier 2 (Níveis 5-10, ND 5-10, ~1.800 a 5.900 XP): Heróis do reino, masmorras profundas, monstros míticos. \
- Tier 3 (Níveis 11-16, ND 11-16, ~7.200 a 15.000 XP): Campeões continentais, arquimagos, dragões adultos. \
- Tier 4 (Níveis 17-20, ND 17-24+, ~18.000 a 62.000+ XP): Mestres do Multiverso, divindades, horrores cósmicos e cataclismos épicos. \
Calibre os Níveis de Desafio (ND / CR) e distribuição de chefes proporcionalmente ao Tier escolhido. \
{}\n\
Se a premissa já contiver monstros definidos pelo usuário, incorpore-os fielmente respeitando os atributos fornecidos. \
Você DEVE responder APENAS com um JSON puro (sem markdown, sem explicações), no seguinte formato:
{{
  \"story_focus\": \"{}\",
  \"expected_acts\": [
    {{
      \"id\": \"act_1\",
      \"act_number\": 1,
      \"arc_name\": \"Arco I: A Sombra da Corte\",
      \"arc_number\": 1,
      \"title\": \"Título do Ato\",
      \"synopsis\": \"Sinopse detalhada, densa e cinematográfica do que se espera acontecer, contextualizando a geopolítica do reino, as tensões sociais/políticas regionais e o choque das forças antagônicas\",
      \"expected_milestones\": [\"Marco 1\", \"Marco 2\", \"Marco 3\"],
      \"boss_name\": \"Nome do Chefe deste Arco (ou null se for investigação pura)\",
      \"battle_intensity\": \"Combate Tático\" ou \"Roleplay Primário\" ou \"Equilibrado\",
      \"completed\": false
    }}
  ],
  \"autonomous_world_events\": [
    {{
      \"id\": \"aut_1\",
      \"trigger_chapter\": 2,
      \"description\": \"Evento que ocorre nos bastidores do mundo independente da party\",
      \"executed\": false,
      \"impact_summary\": null
    }}
  ],
  \"homebrew_bosses\": [
    {{
      \"name\": \"Nome do Boss Homebrew\",
      \"challenge_rating\": \"14\",
      \"description\": \"Conceito, história e papel dramático\",
      \"ac\": 17,
      \"hp\": 200,
      \"str\": 12,
      \"dex\": 18,
      \"con\": 18,
      \"int\": 16,
      \"wis\": 14,
      \"cha\": 22,
      \"passives\": [\"Aura de Geada\"],
      \"actions\": [\"Ataque de Adaga\", \"Sopro\"],
      \"legendary_actions\": [\"Passo Rápido (1)\"]
    }}
  ]
}}", focus_directive, freedom_directive, story_focus);

        let prompt = format!(
            "PREMISSA DA HISTÓRIA:\n{}\n\nESTILO / TOM LITERÁRIO:\n{}\n\nFOCO NARRATIVO SOLICITADO: {}\n\nDivida a campanha em 2 a 4 Arcos dramáticos, com 3 a 5 atos no total e chefes intermediários/finais condizentes com o foco escolhido.",
            premise, literary_style, story_focus
        );

        let response = match self.gemini.generate_narrative(&prompt, &system_instruction).await {
            Ok(res) => res,
            Err(e) => {
                eprintln!("[SKELETON_AGENT] Aviso ao chamar Gemini API: {}. Usando geração procedural offline de contingência.", e);
                return Ok(Self::generate_fallback_skeleton(premise, literary_style));
            }
        };

        let clean_json = Self::extract_json(&response);
        let parsed: Result<Value, _> = serde_json::from_str(&clean_json);

        match parsed {
            Ok(v) => {
                let mut skeleton = NarrativeSkeleton::new(premise, literary_style);
                skeleton.story_focus = Some(typed_focus);

                if let Some(acts) = v["expected_acts"].as_array() {
                    for act in acts {
                        let id = act["id"].as_str().unwrap_or("act_unknown").to_string();
                        let act_number = act["act_number"].as_u64().unwrap_or(1) as u32;
                        let title = act["title"].as_str().unwrap_or("Ato Sem Título").to_string();
                        let synopsis = act["synopsis"].as_str().unwrap_or("").to_string();
                        let arc_name = act["arc_name"].as_str().map(|s| s.to_string());
                        let arc_number = act["arc_number"].as_u64().map(|n| n as u32);
                        let boss_name = act["boss_name"].as_str().filter(|s| !s.is_empty() && s != &"null").map(|s| s.to_string());
                        let battle_intensity = act["battle_intensity"].as_str().map(|s| s.to_string());
                        let milestones = act["expected_milestones"]
                            .as_array()
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|m| m.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default();

                        skeleton.expected_acts.push(SkeletonNode {
                            id,
                            act_number,
                            title,
                            synopsis,
                            expected_milestones: milestones,
                            completed: false,
                            arc_name,
                            arc_number,
                            boss_name,
                            battle_intensity,
                        });
                    }
                }

                if let Some(events) = v["autonomous_world_events"].as_array() {
                    for ev in events {
                        let id = ev["id"].as_str().unwrap_or("aut_unknown").to_string();
                        let trigger_chapter = ev["trigger_chapter"].as_u64().unwrap_or(2) as u32;
                        let description = ev["description"].as_str().unwrap_or("").to_string();
                        skeleton.autonomous_world_events.push(AutonomousWorldEvent {
                            id,
                            trigger_chapter,
                            description,
                            executed: false,
                            impact_summary: None,
                        });
                    }
                }

                if let Some(bosses) = v["homebrew_bosses"].as_array() {
                    for b in bosses {
                        skeleton.homebrew_bosses.push(b.clone());
                    }
                }

                if skeleton.expected_acts.is_empty() {
                    Ok(Self::generate_fallback_skeleton(premise, literary_style))
                } else {
                    Ok(skeleton)
                }
            }
            Err(_) => Ok(Self::generate_fallback_skeleton(premise, literary_style)),
        }
    }

    /// Avalia se as ações da Cena Viva corromperam o rumo esperado e recalcula os atos futuros se necessário
    pub async fn evaluate_and_repair(
        &self,
        skeleton: &mut NarrativeSkeleton,
        scene_summary: &str,
        current_chapter: u32,
    ) -> Result<bool, Box<dyn Error>> {
        let current_act = match skeleton.current_act() {
            Some(a) => a.clone(),
            None => return Ok(false),
        };

        let system_instruction = "Você é o Árbitro do Esqueleto Narrativo (Gemini). \
Avalie se o desfecho da cena diverge criticamente do plano esperado a ponto de invalidar os atos futuros planejados. \
Responda APENAS com um JSON puro:
{
  \"is_corrupted\": true/false,
  \"divergence_reason\": \"motivo detalhado se houve desvio radical\",
  \"new_remaining_acts\": [
    {
      \"id\": \"act_x\",
      \"act_number\": 2,
      \"title\": \"Novo Título do Ato\",
      \"synopsis\": \"Nova sinopse adaptada às consequências dos jogadores\",
      \"expected_milestones\": [\"Novo marco 1\", \"Novo marco 2\"],
      \"completed\": false
    }
  ]
}";

        let prompt = format!(
            "ATO ATUAL:\nNúmero: {}\nTítulo: {}\nMarcos Esperados: {:?}\n\nDESFECHO DA CENA VIVA:\n{}\n\nCapítulo Atual: {}",
            current_act.act_number, current_act.title, current_act.expected_milestones, scene_summary, current_chapter
        );

        let response = match self.gemini.generate_narrative(&prompt, system_instruction).await {
            Ok(res) => res,
            Err(_) => return Ok(false),
        };

        let clean_json = Self::extract_json(&response);
        if let Ok(v) = serde_json::from_str::<Value>(&clean_json) {
            if v["is_corrupted"].as_bool().unwrap_or(false) {
                let reason = v["divergence_reason"].as_str().unwrap_or("Desvio radical de narrativa");
                let mut new_acts = Vec::new();

                if let Some(acts) = v["new_remaining_acts"].as_array() {
                    for (i, act) in acts.iter().enumerate() {
                        let act_number = (skeleton.current_act_index + 1 + i) as u32;
                        new_acts.push(SkeletonNode {
                            id: format!("act_rep_{}", act_number),
                            act_number,
                            title: act["title"].as_str().unwrap_or("Ato Adaptado").to_string(),
                            synopsis: act["synopsis"].as_str().unwrap_or("").to_string(),
                            expected_milestones: act["expected_milestones"]
                                .as_array()
                                .map(|arr| {
                                    arr.iter()
                                        .filter_map(|m| m.as_str().map(|s| s.to_string()))
                                        .collect()
                                })
                                .unwrap_or_default(),
                            completed: false,
                            arc_name: act["arc_name"].as_str().map(|s| s.to_string()),
                            arc_number: act["arc_number"].as_u64().map(|n| n as u32),
                            boss_name: act["boss_name"].as_str().map(|s| s.to_string()),
                            battle_intensity: act["battle_intensity"].as_str().map(|s| s.to_string()),
                        });
                    }
                }

                if !new_acts.is_empty() {
                    skeleton.record_corruption(
                        reason,
                        &format!("Ato planejado: {}", current_act.title),
                        &format!("Atos recalculados: {} novos atos", new_acts.len()),
                        new_acts,
                    );
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    fn extract_json(raw: &str) -> String {
        let trimmed = raw.trim();
        if let Some(start) = trimmed.find('{') {
            if let Some(end) = trimmed.rfind('}') {
                return trimmed[start..=end].to_string();
            }
        }
        trimmed.to_string()
    }

    fn generate_fallback_skeleton(premise: &str, literary_style: &str) -> NarrativeSkeleton {
        let mut s = NarrativeSkeleton::new(premise, literary_style);

        s.expected_acts.push(SkeletonNode {
            id: "act_1".to_string(),
            act_number: 1,
            title: "O Chamado e a Primeira Fenda".to_string(),
            synopsis: format!("Apresentação do conflito inicial fundamentado em: {}", premise),
            expected_milestones: vec![
                "Reconhecimento das primeiras anomalias ou ameaças".to_string(),
                "Encontro inicial com habitantes ou forças locais".to_string(),
                "Primeira escolha moral ou confronto revelador".to_string(),
            ],
            completed: false,
            arc_name: Some("Arco I: A Sombra e o Chamado".to_string()),
            arc_number: Some(1),
            boss_name: None,
            battle_intensity: Some("Roleplay & Investigação".to_string()),
        });

        s.expected_acts.push(SkeletonNode {
            id: "act_2".to_string(),
            act_number: 2,
            title: "A Profundidade do Abismo".to_string(),
            synopsis: "Revelação de forças ocultas e escalada das apostas pessoais da party.".to_string(),
            expected_milestones: vec![
                "Infiltração ou confronto em território hostil".to_string(),
                "Descoberta de segredo sobre os bastidores da região".to_string(),
                "Ponto sem retorno para os aventureiros".to_string(),
            ],
            completed: false,
            arc_name: Some("Arco II: A Masmorra e a Queda".to_string()),
            arc_number: Some(2),
            boss_name: Some("Capataz das Profundezas".to_string()),
            battle_intensity: Some("Masmorra & Combate Tático".to_string()),
        });

        s.expected_acts.push(SkeletonNode {
            id: "act_3".to_string(),
            act_number: 3,
            title: "O Clímax e a Resolução do Destino".to_string(),
            synopsis: "Confronto definitivo e determinação do novo equilíbrio das terras.".to_string(),
            expected_milestones: vec![
                "Batalha crucial ou negociação de vida e morte".to_string(),
                "Destino final das entidades e sobreviventes".to_string(),
            ],
            completed: false,
            arc_name: Some("Arco III: O Confronto Final".to_string()),
            arc_number: Some(3),
            boss_name: Some("Antagonista Supremo".to_string()),
            battle_intensity: Some("Combate Épico & Clímax".to_string()),
        });

        s.autonomous_world_events.push(AutonomousWorldEvent {
            id: "aut_1".to_string(),
            trigger_chapter: 2,
            description: "Mobilização de patrulhas e rumores se espalhando pelas fronteiras.".to_string(),
            executed: false,
            impact_summary: None,
        });

        s.autonomous_world_events.push(AutonomousWorldEvent {
            id: "aut_2".to_string(),
            trigger_chapter: 3,
            description: "O evento climático ou oculto atinge seu ápice regional.".to_string(),
            executed: false,
            impact_summary: None,
        });

        s
    }

    /// Analisa a premissa e avalia se há requisitos ou ganchos temáticos recomendados para a Party
    /// ('Pode' no sentido de ter a faculdade de avaliar se faz sentido para a trama, NÃO impor obrigatoriedade cega).
    pub async fn analyze_premise_requirements(
        &self,
        premise: &str,
        literary_style: &str,
        dramatic_slider: u32,
    ) -> Result<Value, Box<dyn Error>> {
        let system_instruction = "Você é o Mestre Consultor e Arquiteto de Regras D&D 5e (SRD 5.1). \
Sua missão é analisar criticamente a PREMISSA da campanha fornecida pelo jogador e avaliar a composição do Grupo de Heróis (Party), bem como DETECTAR SUBCLASSES CUSTOMIZADAS, PATRONOS, MAGIAS OU OVERRIDES/SUBSTITUIÇÕES DE REGRAS citados na narrativa. \
DIRETRIZ DE AVALIAÇÃO DE ROLEPLAY: \
- DISTINÇÃO CRUCIAL ENTRE 'NOME PRÓPRIO OBRIGATÓRIO' E 'COMPOSIÇÃO DE ARQUÉTIPO / CLASSE': \
  * Use `requirement_type: 'character_name'` e preencha `required_name` SOMENTE E EXCLUSIVAMENTE quando a história exigir uma pessoa nominalmente insubstituível cujo NOME PRÓPRIO real é o centro da trama (ex: 'Cole', 'Raven', 'Alistair'). \
  * Se a premissa estiver descrevendo ARQUÉTIPOS, PAPÉIS OU CLASSES (ex: 'O Caçador de Goblins', 'O Atleta', 'O Inventor', 'A Arqueira Elfa', 'O Xamã Anão', 'O Sacerdote Lagarto', 'Um guerreiro experiente'), isso NÃO É UM NOME PRÓPRIO OBRIGATÓRIO! Isso é uma exigência de COMPOSIÇÃO DE CLASSE/PAPEL (`requirement_type: 'class'`). Nesse caso, `required_name` DEVE SER `null`, `required_class` deve ser preenchido (ex: 'Fighter', 'Ranger', 'Druid', 'Cleric') e o arquétipo/título sugerido vai em `suggested_options: ['Guerreiro (Fighter)', 'O Caçador de Goblins']`. \
  * O jogador DEVE ser livre para batizar seu herói com o nome que quiser (ex: 'Hammer', 'Kael', 'Sir Jonathan'), desde que cumpra a classe ou o arquétipo exigido. NUNCA bloqueie o jogador por ter chamado seu guerreiro de 'Hammer' em vez de 'O Caçador de Goblins', a não ser que 'Hammer' seja explicitamente proibido pela história. \
- MAPEAMENTO CANÔNICO D&D 5E (MUITO IMPORTANTE): Em `required_class`, use SEMPRE a classe canônica oficial do SRD 5.1 ('Barbarian', 'Bard', 'Cleric', 'Druid', 'Fighter', 'Monk', 'Paladin', 'Ranger', 'Rogue', 'Sorcerer', 'Warlock', 'Wizard' ou 'Artificer'). Mesmo que a narrativa chame o personagem de 'Xamã', 'Arqueira', 'Alquimista' ou 'Vanguarda', traduza para o equivalente mecânico oficial D&D (ex: Xamã -> Druid; Arqueira -> Ranger; Atleta/Vanguarda -> Fighter; Inventor -> Artificer/Wizard; Sacerdote -> Cleric). \
- Em `required_race`, forneça a raça D&D canônica correspondente ('Elfo', 'Anão', 'Humano', 'Draconato', 'Tiefling', 'Halfling', etc). Deixe `null` se a raça for livre. \
- Em `suggested_options`, para cada opção narrativa fornecida, inclua OBRIGATORIAMENTE entre parênteses a classe canônica em D&D correspondente (ex: 'A Arqueira Elfa (Ranger)', 'O Xamã Anão (Druid)', 'O Caçador de Goblins (Fighter)', 'O Sacerdote Lagarto (Cleric)'). Isso permite que tanto o jogador quanto o motor entendam perfeitamente a regra mecânica. \
- Se a premissa clamar por capacidades específicas para superar a missão (ex: infiltração clama por Ladino (Rogue); mortos-vivos clamam por Clérigo (Cleric)/Paladino (Paladin)), aponte esse requisito como OBRIGATÓRIO (`is_mandatory: true`), mas com `required_name: null`. \
- Se a premissa for heróica, aberta ou versátil sem personagens específicos, aponte que a formação é LIVRE (`has_thematic_requirement: false`). \
DIRETRIZ DE PROTEÇÃO CONTRA SPOILERS (CRUCIAL): \
- Ao descrever `party_requirements` e `party_hooks`, NUNCA REVELE PLOT TWISTS, SEGREDOS OCULTOS, TRAIÇÕES, DUPLICATAS, QUEM É O VERDADEIRO CULPADO OU O DESTINO FINAL DOS PERSONAGENS. \
- Descreva os requisitos e ganchos sob a perspectiva inocente e imediata dos personagens no início da aventura (ex: 'O herói Cole é uma figura central da guarda real convocada ao salão do trono' em vez de falar do espelho roubando sua respiração; 'Um combatente resistente para proteger a comitiva nas terras ermas' em vez de revelar o exército de Snowee). \
DIRETRIZ DE OVERRIDES E SUBCLASSES CUSTOMIZADAS: \
- Se o usuário mencionar novas entidades de pacto, patronos, subclasses ou substituições (ex: 'Mumm-Ra substitui O Grande Antigo do Bruxo', 'Pacto com Mumm-Ra', 'Substitui Berserker por Guerreiro do Sangue'), você DEVE incluir em `subclass_overrides`: \
  - `parent_class`: Classe padrão do D&D (ex: 'Warlock', 'Fighter', 'Rogue', 'Wizard', 'Cleric', etc). \
  - `replaces`: Nome exato ou parcial da subclasse que está sendo substituída (ou null se for adicional). Ex: 'O Grande Antigo' ou 'The Great Old One'. \
  - `name`: Nome da nova subclasse/patrono (ex: 'Pacto de Mumm-Ra, o Ser Eterno'). \
  - `description`: Descrição mecânica e temática da subclasse. \
  - `special_ability`: Nome da habilidade especial marcante concedida no 1º nível da subclasse. \
DIRETRIZ DE MAGIAS CUSTOMIZADAS / HOMEBREW: \
- Se o usuário mencionar magias personalizadas, magias ancestrais ou magias adicionais criadas para classes específicas (ex: 'adiciona uma nova magia de nível 1 para feiticeiro chamada Rajada de Plasma', 'magia homebrew Labareda Negra de 1º círculo'), você DEVE incluir em `custom_spells`: \
  - `name`: Nome da magia (ex: 'Rajada de Plasma'). \
  - `level`: Círculo/Nível da magia de 1 a 9 (ou 0 para truques). \
  - `classes`: Lista de classes que podem conjurar (ex: ['Sorcerer'], ['Wizard', 'Warlock']). \
  - `school`: Escola de magia (ex: 'Evocação', 'Necromancia', 'Abjuração', etc). \
  - `description`: Efeito mecânico e narrativo da magia. \
DIRETRIZ DE MONSTROS, CHEFES E ANTAGONISTAS HOMEBREW: \
- Se a premissa contiver fichas técnicas, monstros, chefes ou antagonistas (ex: 'O Espelho de Obsidiana', 'Snowee, a Princesa Corrompida', 'Dunga, o Algoz', 'Sentinela de Éter', 'Malakor'), OU se a trama sugerir ameaças centrais sem ficha explícita, a IA PODE E DEVE extrair ou criar seus blocos de estatísticas D&D 5e em `custom_monsters`: \
  - `name`: Nome do monstro/antagonista (ex: 'Snowee, a Princesa Corrompida'). \
  - `type`: Tipo de criatura e alinhamento (ex: 'Humanoide Médio (Corrompido), Caótico e Neutro'). \
  - `cr`: Nível de Desafio como número ou string (ex: '9' ou '1/2'). \
  - `ac`: Classe de Armadura (número inteiro, ex: 16). \
  - `hp`: Pontos de Vida (número inteiro, ex: 135). \
  - `speed`: Deslocamento (ex: '9 m' ou '15 m, escalada 15 m'). \
  - `stats`: Objeto com os 6 atributos `{\"str\": 10, \"dex\": 16, \"con\": 16, \"int\": 14, \"wis\": 12, \"cha\": 20}`. \
  - `abilities`: Lista de habilidades passivas e resistências (ex: ['Aura do Coração Congelado', 'Imunidade a dano gélido']). \
  - `actions`: Lista de ações de ataque, magias e ações lendárias (ex: ['Lâmina de Gelo (+7, 2d8+3)', 'Sopro de Estilhaços (8d8 gélido)']). \
  - `description`: Breve descrição do papel tático e dramático do monstro na campanha. \
DIRETRIZ DE NÍVEL INICIAL RECOMENDADO: \
- Avalie o nível de perigo e o Nível de Desafio (ND / CR) dos monstros/chefes envolvidos. Para ameaças locais/iniciais (goblins, bandidos, zumbis), o nível recomendado é 1 ou 2. Se a história começar com criaturas míticas, trolls, golens, demônios ou monstros de ND 5 a 10, o nível DEVE ser recomendado como 5 (Tier 2). Para cataclismos de alta fantasia ou dragões adultos, nível 11+. Retorne em `recommended_starting_level` (número inteiro de 1 a 20). \
DIRETRIZ DE TAGS TEMÁTICAS / CATEGORIAS (OBRIGATÓRIO): \
- As tags DEVEM ser estritamente de UMA ÚNICA PALAVRA cada, sem espaços, em letras minúsculas (ex: [\"fantasia\", \"sombria\", \"goblins\", \"grimdark\", \"terror\", \"masmorra\", \"investigacao\", \"scifi\"]). Você DEVE gerar de 3 a 7 tags temáticas extraídas a partir da premissa E do estilo literário fornecido. \
Entregue APENAS um JSON puro no seguinte formato (subclass_overrides, custom_spells e custom_monsters DEVEM ser retornados sempre, como lista vazia [] se não houverem ou preenchidos com os itens identificados/criados): \
{ \
  \"analysis_summary\": \"Breve diagnóstico da atmosfera e conflito imediato (SEM SPOILERS de reviravoltas ou culpados)\", \
  \"adventure_tone\": \"Tom dramático (ex: Sobrevivência Sombria, Investigação Cósmica, Alta Fantasia Heróica, etc)\", \
  \"tags\": [\"fantasia\", \"sombria\", \"anos80\", \"scifi\", \"heman\"], \
  \"recommended_party_size\": \"1 a 6 jogadores (ou a quantidade exata citada na premissa, ex: '5 jogadores')\", \
  \"recommended_starting_level\": 1, \
  \"has_thematic_requirement\": true/false, \
  \"subclass_overrides\": [], \
  \"custom_spells\": [], \
  \"custom_monsters\": [], \
  \"party_requirements\": [ \
    { \
      \"requirement_type\": \"character_name\" | \"class\" | \"race\" | \"skill\" | \"free\", \
      \"description\": \"Explicação imersiva SEM SPOILERS do porquê desta presença na comitiva\", \
      \"is_mandatory\": true/false, \
      \"required_name\": \"Cole\" (ou null), \
      \"required_class\": \"Fighter\" (ou null - use a classe oficial D&D como Ranger, Cleric, Druid, Fighter, Rogue, Wizard, etc), \
      \"required_race\": \"Humano\" (ou 'Elfo', 'Anão', etc, ou null), \
      \"suggested_options\": [\"Guerreiro (Fighter)\", \"Cole\"] \
    } \
  ], \
  \"party_hooks\": [ \
    \"Gancho 1: Chamado inicial ou juramento que une a comitiva aos acontecimentos\", \
    \"Gancho 2: Vínculo honrado ou dever compartilhado\" \
  ] \
}";

        let prompt = format!(
            "PREMISSA DA CAMPANHA:\n{}\n\nESTILO / TOM LITERÁRIO:\n{}\n\nFOCO DRAMÁTICO (SLIDER 1 A 20): {}/20\n(1=Pura Intriga/Roleplay/Diplomacia, 10=Equilíbrio 50/50, 20=Guerra Tática & Masmorras Letais)\n\nAnalise criticamente esta premissa e gere o parecer de composição e ganchos da party.",
            premise, literary_style, dramatic_slider
        );

        let raw = match self.gemini.generate_narrative(&prompt, system_instruction).await {
            Ok(t) => t,
            Err(e) => {
                eprintln!("[SKELETON_AGENT] Erro na API Gemini para análise de premissa: {}. Usando fallback determinístico.", e);
                return Ok(Self::deterministic_premise_analysis(premise, dramatic_slider));
            }
        };

        let clean = raw
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        match serde_json::from_str::<Value>(clean) {
            Ok(mut v) => {
                let det = Self::deterministic_premise_analysis(premise, dramatic_slider);
                // Se a IA omitiu subclass_overrides, complementa com os overrides determinísticos da premissa
                if v.get("subclass_overrides").is_none() || v["subclass_overrides"].as_array().map_or(true, |a| a.is_empty()) {
                    if let Some(overrides) = det.get("subclass_overrides") {
                        if !overrides.as_array().map_or(true, |a| a.is_empty()) {
                            v["subclass_overrides"] = overrides.clone();
                        }
                    }
                }
                // Se a IA omitiu custom_spells, complementa com as magias determinísticas da premissa
                if v.get("custom_spells").is_none() || v["custom_spells"].as_array().map_or(true, |a| a.is_empty()) {
                    if let Some(spells) = det.get("custom_spells") {
                        if !spells.as_array().map_or(true, |a| a.is_empty()) {
                            v["custom_spells"] = spells.clone();
                        }
                    }
                }
                // Se a IA omitiu custom_monsters, complementa com os monstros determinísticos extraídos da premissa
                if v.get("custom_monsters").is_none() || v["custom_monsters"].as_array().map_or(true, |a| a.is_empty()) {
                    if let Some(monsters) = det.get("custom_monsters") {
                        if !monsters.as_array().map_or(true, |a| a.is_empty()) {
                            v["custom_monsters"] = monsters.clone();
                        }
                    }
                }

                // Se a IA omitiu tags ou retornou vazio, complementa com as tags determinísticas da premissa
                if v.get("tags").is_none() || v["tags"].as_array().map_or(true, |a| a.is_empty()) {
                    if let Some(tags) = det.get("tags") {
                        if !tags.as_array().map_or(true, |a| a.is_empty()) {
                            v["tags"] = tags.clone();
                        }
                    }
                }

                if v.get("recommended_starting_level").is_none() || v["recommended_starting_level"].as_u64().unwrap_or(0) == 0 {
                    let lvl = det.get("recommended_starting_level").and_then(|l| l.as_u64()).unwrap_or(1);
                    v["recommended_starting_level"] = serde_json::json!(lvl);
                }

                // Normaliza todas as tags geradas (pela IA ou determinísticas) para palavras únicas em minúsculas
                if let Some(tags_val) = v.get("tags").and_then(|t| t.as_array()) {
                    let mut normalized_tags = Vec::new();
                    for item in tags_val {
                        if let Some(s) = item.as_str() {
                            for word in s.split_whitespace() {
                                let clean = word
                                    .trim_matches(|c: char| !c.is_alphanumeric())
                                    .to_lowercase();
                                if !clean.is_empty() && !normalized_tags.contains(&clean) {
                                    normalized_tags.push(clean);
                                }
                            }
                        }
                    }
                    if normalized_tags.is_empty() {
                        normalized_tags = Self::extract_deterministic_tags(premise);
                    }
                    v["tags"] = serde_json::json!(normalized_tags);
                }

                // Deduplicação estrita de monstros e ameaças por nome normalizado
                if let Some(monsters) = v.get_mut("custom_monsters").and_then(|m| m.as_array_mut()) {
                    let mut seen = std::collections::HashSet::new();
                    monsters.retain(|item| {
                        let name = item.get("name").and_then(|n| n.as_str()).unwrap_or("").trim().to_lowercase();
                        if name.is_empty() { return false; }
                        // Deduplicação por similaridade ou prefixo (ex: "Dunga, o Algoz Sorridente" vs "Dunga, o Algoz das Sombras")
                        let base_key = if name.contains("dunga") { "dunga".to_string() }
                            else if name.contains("espelho") { "espelho".to_string() }
                            else if name.contains("snowee") { "snowee".to_string() }
                            else { name };
                        seen.insert(base_key)
                    });
                }
                Ok(v)
            }
            Err(_) => Ok(Self::deterministic_premise_analysis(premise, dramatic_slider)),
        }
    }

    pub fn deterministic_premise_analysis(premise: &str, dramatic_slider: u32) -> Value {
        let p_lower = premise.to_lowercase();
        let is_stealth = p_lower.contains("roub") || p_lower.contains("infiltr") || p_lower.contains("furtiv") || p_lower.contains("espi");
        let is_undead = p_lower.contains("mort") || p_lower.contains("zumbi") || p_lower.contains("osso") || p_lower.contains("necro");
        let is_elven_astral = p_lower.contains("elf") || p_lower.contains("estrela") || p_lower.contains("astral") || p_lower.contains("arca");

        let mut reqs = Vec::new();
        let mut has_req = false;

        // Caso 1: Nome específico de personagem citado (ex: "Cole", "Raven", "Arthur")
        if p_lower.contains("cole") {
            has_req = true;
            let cls = if p_lower.contains("guerreir") || p_lower.contains("soldad") { "Guerreiro" } else { "Fighter" };
            reqs.push(serde_json::json!({
                "requirement_type": "character_name",
                "description": "O enredo de roleplay exige a presença de um Guerreiro chamado Cole na comitiva para liderar a missão.",
                "is_mandatory": true,
                "required_name": "Cole",
                "required_class": cls,
                "suggested_options": [cls, "Cole"]
            }));
        }

        // Caso 2: Exigência explícita de classe
        if (p_lower.contains("precisa") || p_lower.contains("exige") || p_lower.contains("obrigator") || p_lower.contains("deve ter")) && !p_lower.contains("cole") {
            if p_lower.contains("guerreir") {
                has_req = true;
                reqs.push(serde_json::json!({
                    "requirement_type": "class",
                    "description": "A premissa exige pelo menos um Guerreiro na comitiva para segurar a vanguarda.",
                    "is_mandatory": true,
                    "required_class": "Guerreiro",
                    "suggested_options": ["Guerreiro", "Fighter"]
                }));
            } else if p_lower.contains("ladin") {
                has_req = true;
                reqs.push(serde_json::json!({
                    "requirement_type": "class",
                    "description": "A premissa exige um Ladino na comitiva para infiltração e desarme de armadilhas.",
                    "is_mandatory": true,
                    "required_class": "Ladino",
                    "suggested_options": ["Ladino", "Rogue"]
                }));
            } else if p_lower.contains("clérig") || p_lower.contains("clerig") {
                has_req = true;
                reqs.push(serde_json::json!({
                    "requirement_type": "class",
                    "description": "A premissa exige um Clérigo na comitiva para canalizar a fé e purificar o mal.",
                    "is_mandatory": true,
                    "required_class": "Clérigo",
                    "suggested_options": ["Clérigo", "Cleric"]
                }));
            } else if p_lower.contains("paladin") {
                has_req = true;
                reqs.push(serde_json::json!({
                    "requirement_type": "class",
                    "description": "A premissa exige um Paladino na comitiva com juramento sagrado e poder divino.",
                    "is_mandatory": true,
                    "required_class": "Paladino",
                    "suggested_options": ["Paladino", "Paladin"]
                }));
            } else if p_lower.contains("barbar") {
                has_req = true;
                reqs.push(serde_json::json!({
                    "requirement_type": "class",
                    "description": "A premissa exige um Bárbaro na comitiva para resistir a danos colossais.",
                    "is_mandatory": true,
                    "required_class": "Bárbaro",
                    "suggested_options": ["Bárbaro", "Barbarian"]
                }));
            } else if p_lower.contains("ranger") || p_lower.contains("patrulh") {
                has_req = true;
                reqs.push(serde_json::json!({
                    "requirement_type": "class",
                    "description": "A premissa exige um Patrulheiro (Ranger) na comitiva para sobrevivência e rastreamento.",
                    "is_mandatory": true,
                    "required_class": "Ranger",
                    "suggested_options": ["Ranger", "Patrulheiro"]
                }));
            } else if p_lower.contains("druid") {
                has_req = true;
                reqs.push(serde_json::json!({
                    "requirement_type": "class",
                    "description": "A premissa exige um Druida na comitiva para comandar as forças da natureza.",
                    "is_mandatory": true,
                    "required_class": "Druida",
                    "suggested_options": ["Druida", "Druid"]
                }));
            } else if p_lower.contains("monk") || p_lower.contains("mong") {
                has_req = true;
                reqs.push(serde_json::json!({
                    "requirement_type": "class",
                    "description": "A premissa exige um Monge na comitiva canalizando disciplina e ki marcial.",
                    "is_mandatory": true,
                    "required_class": "Monge",
                    "suggested_options": ["Monge", "Monk"]
                }));
            } else if p_lower.contains("bard") {
                has_req = true;
                reqs.push(serde_json::json!({
                    "requirement_type": "class",
                    "description": "A premissa exige um Bardo na comitiva para inspiração e manipulação social.",
                    "is_mandatory": true,
                    "required_class": "Bardo",
                    "suggested_options": ["Bardo", "Bard"]
                }));
            } else if p_lower.contains("brux") || p_lower.contains("warlock") {
                has_req = true;
                reqs.push(serde_json::json!({
                    "requirement_type": "class",
                    "description": "A premissa exige um Bruxo (Warlock) na comitiva ligado a entidades arcanas.",
                    "is_mandatory": true,
                    "required_class": "Warlock",
                    "suggested_options": ["Bruxo", "Warlock"]
                }));
            } else if p_lower.contains("feiticeir") || p_lower.contains("sorcerer") {
                has_req = true;
                reqs.push(serde_json::json!({
                    "requirement_type": "class",
                    "description": "A premissa exige um Feiticeiro na comitiva com magia inata no sangue.",
                    "is_mandatory": true,
                    "required_class": "Sorcerer",
                    "suggested_options": ["Feiticeiro", "Sorcerer"]
                }));
            } else if p_lower.contains("mag") {
                has_req = true;
                reqs.push(serde_json::json!({
                    "requirement_type": "class",
                    "description": "A premissa exige um Mago na comitiva para decifração arcana.",
                    "is_mandatory": true,
                    "required_class": "Mago",
                    "suggested_options": ["Mago", "Wizard"]
                }));
            }
        }

        if is_stealth && !reqs.iter().any(|r| r["suggested_options"].as_array().map_or(false, |arr| arr.iter().any(|x| x == "Ladino"))) {
            has_req = true;
            reqs.push(serde_json::json!({
                "requirement_type": "class",
                "description": "A natureza furtiva da missão exige pelo menos um especialista em infiltração (Ladino/Ranger/Monge) para evitar alarmes.",
                "is_mandatory": true,
                "required_class": "Ladino",
                "suggested_options": ["Ladino", "Ranger", "Monge"]
            }));
        } else if is_undead && !reqs.iter().any(|r| r["suggested_options"].as_array().map_or(false, |arr| arr.iter().any(|x| x == "Clérigo"))) {
            has_req = true;
            reqs.push(serde_json::json!({
                "requirement_type": "class",
                "description": "A presença profana de mortos-vivos exige um canalizador de fé divina (Clérigo/Paladino) para expurgar as trevas.",
                "is_mandatory": true,
                "required_class": "Clérigo",
                "suggested_options": ["Clérigo", "Paladino", "Druida"]
            }));
        } else if is_elven_astral && !reqs.iter().any(|r| r["suggested_options"].as_array().map_or(false, |arr| arr.iter().any(|x| x == "Elfo"))) {
            has_req = true;
            reqs.push(serde_json::json!({
                "requirement_type": "race",
                "description": "Ruínas astrais e selos antigos exigem sensibilidade mística élfica (Elfo/Meio-Elfo) para plena decifração.",
                "is_mandatory": true,
                "required_race": "Elfo",
                "suggested_options": ["Elfo", "Meio-Elfo", "Mago"]
            }));
        }

        if reqs.is_empty() {
            reqs.push(serde_json::json!({
                "requirement_type": "free",
                "description": "Composição Livre: A premissa é aberta e versátil. Qualquer grupo de 1 a 4 heróis encontrará desafios sob medida.",
                "is_mandatory": false,
                "suggested_options": ["Guerreiro", "Mago", "Clérigo", "Ladino"]
            }));
        }

        let tone = if dramatic_slider <= 6 {
            "Intriga Política & Exploração Social"
        } else if dramatic_slider >= 14 {
            "Combate Tático & Sobrevivência Letal"
        } else {
            "Aventura Clássica D&D 5e (Equilibrada)"
        };

        let subclass_overrides = crate::llm::campaign_presets::CampaignPresets::get_subclass_overrides(&p_lower);
        let custom_spells = crate::llm::campaign_presets::CampaignPresets::get_custom_spells(&p_lower);
        let custom_monsters = crate::llm::campaign_presets::CampaignPresets::get_custom_monsters(&p_lower);

        let rec_size = if p_lower.contains("cinco") || p_lower.contains("quinteto") {
            "5 aventureiros (Quinteto)"
        } else if p_lower.contains("seis") || p_lower.contains("sexteto") {
            "6 aventureiros (Sexteto)"
        } else {
            "1 a 6 heróis"
        };

        let mut rec_level = 1;
        for m in &custom_monsters {
            if let Some(cr_str) = m.get("cr").and_then(|c| c.as_str()) {
                if let Ok(cr_num) = cr_str.parse::<u32>() {
                    if cr_num >= 9 {
                        rec_level = rec_level.max(9);
                    } else if cr_num >= 5 {
                        rec_level = rec_level.max(5);
                    } else if cr_num >= 3 {
                        rec_level = rec_level.max(3);
                    }
                }
            } else if let Some(cr_num) = m.get("cr").and_then(|c| c.as_u64()) {
                if cr_num >= 9 {
                    rec_level = rec_level.max(9);
                } else if cr_num >= 5 {
                    rec_level = rec_level.max(5);
                } else if cr_num >= 3 {
                    rec_level = rec_level.max(3);
                }
            }
        }
        if p_lower.contains("nível 5") || p_lower.contains("nivel 5") || p_lower.contains("troll") || p_lower.contains("dragão") || p_lower.contains("dragao") || p_lower.contains("demônio") || p_lower.contains("demonio") {
            rec_level = rec_level.max(5);
        }

        let tags = Self::extract_deterministic_tags(premise);

        serde_json::json!({
            "analysis_summary": format!("Premissa ambientada com tom de {}. Foco dramático ajustado em {}/20.", tone, dramatic_slider),
            "adventure_tone": tone,
            "tags": tags,
            "recommended_party_size": rec_size,
            "recommended_starting_level": rec_level,
            "has_thematic_requirement": has_req,
            "subclass_overrides": subclass_overrides,
            "custom_spells": custom_spells,
            "custom_monsters": custom_monsters,
            "party_requirements": reqs,
            "party_hooks": [
                "O grupo compartilha uma ligação com os eventos iniciais ou foi contratado por sua reputação.",
                "Um presságio ou segredo pessoal liga pelo menos um membro ao destino desta região."
            ]
        })
    }

    pub fn extract_deterministic_tags(premise: &str) -> Vec<String> {
        let p_lower = premise.to_lowercase();
        let is_stealth = p_lower.contains("roub") || p_lower.contains("infiltr") || p_lower.contains("furtiv") || p_lower.contains("espi");

        let mut raw_tags = Vec::new();
        if p_lower.contains("80") || p_lower.contains("90") || p_lower.contains("anos80") {
            raw_tags.push("anos80".to_string());
        }
        if p_lower.contains("tecnol") || p_lower.contains("robô") || p_lower.contains("robo") || p_lower.contains("cibern") || p_lower.contains("sci-fi") || p_lower.contains("scifi") {
            raw_tags.push("scifi".to_string());
        }
        if p_lower.contains("corromp") || p_lower.contains("sombr") || p_lower.contains("gétic") || p_lower.contains("gotico") || p_lower.contains("treva") || p_lower.contains("terror") {
            raw_tags.push("sombria".to_string());
        }
        if p_lower.contains("grimdark") || p_lower.contains("dark") || p_lower.contains("brutal") {
            raw_tags.push("grimdark".to_string());
        }
        if p_lower.contains("infanto") || p_lower.contains("juvenil") || p_lower.contains("infantil") || p_lower.contains("prismia") || p_lower.contains("cores") {
            raw_tags.push("infantil".to_string());
        }
        if p_lower.contains("he-man") || p_lower.contains("heman") || p_lower.contains("she-ra") || p_lower.contains("eternia") || p_lower.contains("etheria") {
            raw_tags.push("heman".to_string());
        }
        if p_lower.contains("investig") || p_lower.contains("mistér") || p_lower.contains("mister") {
            raw_tags.push("investigacao".to_string());
        }
        if is_stealth || p_lower.contains("espionagem") {
            raw_tags.push("espionagem".to_string());
        }
        if p_lower.contains("masmorra") || p_lower.contains("dungeon") || p_lower.contains("catacumb") {
            raw_tags.push("dungeon".to_string());
        }
        if raw_tags.is_empty() {
            raw_tags.push("fantasia".to_string());
            raw_tags.push("aventura".to_string());
        }

        let mut normalized = Vec::new();
        for t in raw_tags {
            for word in t.split_whitespace() {
                let clean = word
                    .trim_matches(|c: char| !c.is_alphanumeric())
                    .to_lowercase();
                if !clean.is_empty() && !normalized.contains(&clean) {
                    normalized.push(clean);
                }
            }
        }
        normalized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_deterministic_tags_single_words_lowercase() {
        let premise = "Uma aventura nos anos 80 com robôs e sci-fi em Eternia contra forças sombrias em uma dungeon grimdark";
        let tags = SkeletonAgent::extract_deterministic_tags(premise);

        assert!(!tags.is_empty());
        for tag in &tags {
            // Must have no whitespace
            assert!(!tag.contains(char::is_whitespace), "Tag '{}' contains whitespace", tag);
            // Must be lowercase
            assert_eq!(tag, &tag.to_lowercase(), "Tag '{}' is not lowercase", tag);
        }
        assert!(tags.contains(&"anos80".to_string()));
        assert!(tags.contains(&"scifi".to_string()));
        assert!(tags.contains(&"heman".to_string()));
        assert!(tags.contains(&"sombria".to_string()));
        assert!(tags.contains(&"grimdark".to_string()));
        assert!(tags.contains(&"dungeon".to_string()));
    }

    #[test]
    fn test_extract_deterministic_tags_fallback() {
        let premise = "Um reino pacífico de bardos e menestréis.";
        let tags = SkeletonAgent::extract_deterministic_tags(premise);

        assert_eq!(tags, vec!["fantasia".to_string(), "aventura".to_string()]);
    }
}
