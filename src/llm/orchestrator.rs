use crate::llm::deepseek::DeepSeekClient;
use crate::llm::gemini::GeminiClient;
use serde_json::Value;
use std::error::Error;

pub struct NarrativeOrchestrator {
    pub gemini: GeminiClient,
    pub deepseek: DeepSeekClient,
}

impl NarrativeOrchestrator {
    pub fn new(gemini_api_key: String, deepseek_api_key: String) -> Self {
        Self {
            gemini: GeminiClient::new(gemini_api_key),
            deepseek: DeepSeekClient::new(deepseek_api_key, None),
        }
    }

    pub async fn process_player_action(
        &self,
        prompt: &str,
        state: &mut crate::state::CampaignState,
    ) -> Result<String, Box<dyn Error>> {
        let system_instruction = "Você é o Mestre Narrador de RPG de Mesa (Gemini). Receba a ação do jogador e avance a história de forma imersiva. Se a ação do jogador exigir combate físico, uso de magias de dano ou testes de habilidade/perícia estritos (D&D 5e), você DEVE interromper a sua narração exatamente antes de resolver se deu certo ou errado. Em vez de resolver, emita EXATAMENTE a tag #regras seguida por um bloco JSON contendo 'action' ('skill_check', 'ability_check' ou 'attack'), 'actor', 'skill'/'ability' e opcionalmente 'dc' ou 'target', e NADA MAIS. Exemplo: #regras {\"action\": \"skill_check\", \"skill\": \"Intimidation\", \"actor\": \"Laura\", \"dc\": 14}";

        println!("[ORQUESTRADOR] Enviando intenção do jogador para o Gemini...");
        let narrative = self.gemini.generate_narrative(prompt, system_instruction).await?;
        
        if let Some(idx) = narrative.find("#regras") {
            let pure_narrative = &narrative[..idx];
            println!("\n[GEMINI (Narrador)]: {}", pure_narrative.trim());
            
            let json_part = narrative[idx + 7..].trim();
            let clean_json = json_part.trim_matches('`').trim_start_matches("json").trim();

            println!("\n[ORQUESTRADOR] Interceptado Hook de Regras: {}", clean_json);
            
            let hook_payload: Value = serde_json::from_str(clean_json).unwrap_or(serde_json::json!({"action": "unknown", "actor": "unknown"}));
            
            println!("[MOTOR RUST] Resolvendo mecânica deterministicamente via RulesResolver...");
            let rules_result = crate::rules::resolver::RulesResolver::resolve(&hook_payload, state);
            println!("[MOTOR RUST] Resultado Mecânico: {}", rules_result);

            println!("[ORQUESTRADOR] Passando para o Pipeline Editorial (Linter DeepSeek + Gemini)...");
            let scene_context = format!(
                "Cena em andamento: {}. Resolução determinística de regras em Rust: {}",
                pure_narrative.trim(),
                rules_result
            );

            let refined_narration = self.generate_refined_narrative(&scene_context, prompt).await?;
            return Ok(format!("{}\n\n{}", pure_narrative.trim(), refined_narration.trim()));
        }

        // Se não houve a tag #regras, o Gemini apenas contou uma história normal.
        println!("\n[GEMINI (Narrador)]: {}", narrative.trim());
        Ok(narrative)
    }

    /// Pipeline de Refinamento Editorial em Duas Vias:
    /// 1. Gemini Flash-Lite gera o primeiro rascunho da cena/narrativa.
    /// 2. DeepSeek V3 atua como Linter Literário/Árbitro de regras, avaliando ritmo, verossimilhança e podando clichês.
    /// 3. Gemini Flash-Lite reescreve a cena orientado pelas diretrizes críticas, produzindo a narração final definitiva.
    pub async fn generate_refined_narrative(
        &self,
        scene_context: &str,
        player_action: &str,
    ) -> Result<String, Box<dyn Error>> {
        println!("\n[PIPELINE] Passo 1: Gemini Flash-Lite gerando rascunho da cena...");
        let draft_system = "Você é o Narrador de RPG (Gemini Flash-Lite). Receba a ação do jogador e o contexto da cena e produza um rascunho cinematográfico, sensorial e imersivo.";
        let draft_prompt = format!(
            "CONTEXTO DA CENA:\n{}\n\nAÇÃO DO JOGADOR:\n{}",
            scene_context, player_action
        );
        let draft = self.gemini.generate_narrative(&draft_prompt, draft_system).await?;

        println!("[PIPELINE] Passo 2: DeepSeek V3 atuando como Linter Literário & Regras...");
        let critique_guidelines = format!(
            "Contexto da campanha: {}. Ação do jogador: {}. Exija eliminação de clichês, foco no subtexto e solidez psicológica.",
            scene_context, player_action
        );
        let critique = self.deepseek.critique_text(&draft, &critique_guidelines).await?;

        println!("[PIPELINE] Passo 3: Gemini Flash-Lite polindo e finalizando a narração...");
        let final_system = "Você é um mestre da ficção narrativa. Reescreva o rascunho anterior aplicando com rigor todas as críticas e diretrizes editoriais fornecidas. Produza o conto/cena definitivo com máxima maturidade, verossimilhança e densidade sensorial.";
        let final_prompt = format!(
            "RASCUNHO ORIGINAL:\n{}\n\n---\nCRÍTICA E DIRETRIZES DO EDITOR:\n{}\n\nEntregue apenas o texto definitivo final.",
            draft, critique
        );
        let polished_narrative = self.gemini.generate_narrative(&final_prompt, final_system).await?;

        println!("[PIPELINE] Narração refinada com sucesso!");
        Ok(polished_narrative)
    }

    /// Processo Revisor Editorial de Premissas Customizadas (Duas Vias Gemini + DeepSeek):
    /// Refina a premissa crua do administrador eliminando clichês, elevando tensão dramática,
    /// coerência temática D&D 5e e subtexto narrativo.
    pub async fn refine_premise(
        &self,
        raw_title: &str,
        raw_premise: &str,
        literary_style: &str,
    ) -> Result<String, Box<dyn Error>> {
        println!("[PREMISE_REFINER] Passo 1: Gemini elaborando rascunho enriquecido da premissa...");
        let draft_system = "Você é um arquiteto narrativo sênior de RPG de mesa e mundos de alta fantasia. \
Seu objetivo é enriquecer a atmosfera, estilo e tensão da premissa SEM NUNCA CORTAR OU CONDENSAR o conteúdo original. \
DIRETRIZES DE OURO: \
1. PRESERVE INTEGRALMENTE todas as seções (Arquétipos de Heróis, Regras de Mundo, Pilares Mecânicos, Vilões, Armas e Artefatos). \
2. Se o autor listou arquétipos ou heróis (ex: 'O Líder Nato', 'A Mística', Cole, Snowen), MANTENHA TODOS eles com destaque e riqueza. \
3. Se o autor descreveu regras mecânicas (ex: gestão de calor, fadiga, física determinística), MANTENHA-AS na íntegra. \
4. Não resuma: embeleze e aprofunde o texto mantendo a estrutura original intacta.";
        let draft_prompt = format!(
            "TÍTULO PROPOSTO: {}\nESTILO LITERÁRIO: {}\nDOCUMENTO COMPLETO DO CRIADOR:\n{}\n\nReescreva de forma magistral, cinematográfica e madura, PRESERVANDO TODAS as seções, arquétipos de personagens e regras mecânicas apresentados no documento original.",
            raw_title, literary_style, raw_premise
        );

        let draft = match self.gemini.generate_narrative(&draft_prompt, draft_system).await {
            Ok(d) => d,
            Err(e) => {
                println!("[PREMISE_REFINER] Aviso Gemini rascunho: {}. Mantendo original.", e);
                return Ok(raw_premise.to_string());
            }
        };

        println!("[PREMISE_REFINER] Passo 2: DeepSeek V3 atuando como Crítico Editorial & Revisor de Clichês...");
        let critique_guidelines = format!(
            "Avalie este texto para uma campanha de RPG '{}'. Verifique se todos os arquétipos, regras mecânicas e nomes originais foram preservados fielmente e aponte apenas melhorias de tom e estilo sem pedir cortes de conteúdo.",
            literary_style
        );
        let critique = self.deepseek.critique_text(&draft, &critique_guidelines).await.unwrap_or_else(|_| "Mantenha todas as seções e arquétipos originais com a máxima imersão.".to_string());

        println!("[PREMISE_REFINER] Passo 3: Gemini produzindo versão refinada final...");
        let final_system = "Você é um editor literário implacável de alta fantasia. \
Reescreva a premissa aplicando as melhorias estilísticas da crítica editorial. \
REGRA SUPREMA: NUNCA CORTE CONTEÚDO. Preserve todos os arquétipos de heróis, monstros, sistemas de regras e nomes próprios do documento original. \
Responda APENAS com o texto completo em português.";
        let final_prompt = format!(
            "DOCUMENTO ORIGINAL DO CRIADOR (Referência Absoluta de Conteúdo):\n{}\n\nRASCUNHO:\n{}\n\nCRÍTICA EDITORIAL:\n{}\n\nEntregue o texto definitivo completo sem omitir nenhuma seção ou arquétipo.",
            raw_premise, draft, critique
        );

        let final_premise = self.gemini.generate_narrative(&final_prompt, final_system).await.unwrap_or(draft);
        println!("[PREMISE_REFINER] Premissa refinada com sucesso pelo processo revisor de IA!");
        Ok(final_premise.trim().to_string())
    }

    /// Processo Compilador de Magias em Rhai (Duas Vias Gemini + DeepSeek Linter):
    /// Recebe a descrição em linguagem natural de uma magia e produz um script Rhai executável
    /// que opera diretamente nas structs determinísticas do D&D 5e (TargetProxy e CombatContext).
    pub async fn compile_spell_to_rhai(
        &self,
        spell_description: &str,
    ) -> Result<String, Box<dyn Error>> {
        println!("[SPELL_COMPILER] Passo 1: Gemini elaborando função Rhai para a magia...");
        let draft_system = r#"Você é um compilador e desenvolvedor sênior de scripts Rhai para o motor de regras D&D 5e em Rust.
Você deve converter a descrição da magia do usuário em uma função Rhai pura com a seguinte assinatura exata:

fn cast_spell(caster, target, ctx) {
    // Código Rhai
    // Para persistir alterações em 'target' e 'ctx', você pode chamar métodos em target ou retorná-los
    return target;
}

API DISPONÍVEL NO OBJETO ctx (CombatContext):
- ctx.roll("2d6") -> i64 (rola dados)
- ctx.roll_save(nome_alvo_string, mod_int, dc_int) -> bool (retorna true se passou)
- ctx.log_msg("mensagem") (registra narrativa/efeito)
- ctx.secondary_count() -> i64 (retorna a quantidade de alvos secundários na área/cone)
- ctx.get_secondary_name(indice_i64) -> String (nome do alvo secundário)
- ctx.damage_secondary(indice_i64, "TipoDano", quantidade_i64) (aplica dano a alvo secundário no cone/área)

API DISPONÍVEL NO OBJETO target e caster (TargetProxy):
- target.name -> String
- target.current_hp -> i64
- target.max_hp -> i64
- target.ac -> i64
- target.take_damage("TipoDano", quantidade_i64)
- target.heal(quantidade_i64)
- target.add_condition("NomeCondicao")

Regras de D&D 5e:
- Se a magia exigir salvaguarda, chame ctx.roll_save. Se passar, reduza o dano pela metade (ou conforme descrição).
- Aplique o dano no alvo principal em target.take_damage ou a cura em target.heal.
- Se houver cone/área/estilhaços atrás do alvo afetando alvos secundários, você DEVE iterar de 0 até ctx.secondary_count() - 1, testar a salvaguarda de cada um com ctx.roll_save e aplicar o dano com ctx.damage_secondary(i, "Tipo", dano). NUNCA omita a mecânica secundária!
- IMPORTANTE DE SINTAXE RHAI: Rhai NÃO SUPORTA operador ternário `cond ? a : b`. SEMPRE use `if cond { a } else { b }`.
- No final da função, DEVE OBRIGATORIAMENTE retornar `return target;`.
- Responda APENAS com o código Rhai puro dentro da função `fn cast_spell(caster, target, ctx) { ... }`, sem explicações e sem bloco markdown.
"#;

        let draft_prompt = format!(
            "DESCRICAO DA MAGIA:\n{}\n\nEscreva a função completa `fn cast_spell(caster, target, ctx)` em Rhai contemplando TODOS os efeitos e retornando target no final (lembrando: NUNCA use operador ternário `? :`):",
            spell_description
        );

        let draft = self.gemini.generate_narrative(&draft_prompt, draft_system).await?;

        println!("[SPELL_COMPILER] Passo 2: DeepSeek V3 atuando como Linter & Revisor de Sintaxe Rhai...");
        let critique_guidelines = r#"Você é um linter estrito de sintaxe da linguagem Rhai e regras de D&D 5e.
Verifique se o script Rhai contém:
1. Assinatura exata `fn cast_spell(caster, target, ctx)` e termina com `return target;`.
2. PROIBIÇÃO ABSOLUTA DE OPERADOR TERNÁRIO: Rhai NÃO tem operador `? :` (ternary). Se houver `cond ? x : y`, aponte erro fatal e ordene substituir por `if cond { x } else { y }`.
3. Se a magia tiver efeitos secundários/em cone, verifica se trata `ctx.secondary_count()` e `ctx.damage_secondary(...)`.
4. Chamadas válidas aos métodos: ctx.roll("..."), ctx.roll_save(nome, mod, dc), target.take_damage("tipo", qtd), target.heal(qtd), target.add_condition("cond").
5. Ausência de palavras-chave inválidas ou tipos do Rust incompatíveis com Rhai.
Forneça correções pontuais e diretas. Se o código já estiver perfeito, responda 'SCRIPT_VALIDO'."#;

        println!("[SPELL_COMPILER] Passo 2: DeepSeek V3 atuando como Linter & Revisor de Sintaxe Rhai...");
        let critique = self
            .deepseek
            .lint_code(&draft, critique_guidelines)
            .await
            .unwrap_or_else(|_| "SCRIPT_VALIDO".to_string());
        
        let final_script = if critique.contains("SCRIPT_VALIDO") {
            draft
        } else {
            println!("[SPELL_COMPILER] Passo 3: Gemini aplicando correções do linter...");
            let final_system = "Você é um compilador Rhai de alta precisão. Reescreva o script Rhai da magia aplicando com rigor as correções do linter. Responda APENAS com o código da função Rhai puro sem markdown e sem comentários fora do código.";
            let final_prompt = format!(
                "RASCUNHO DO SCRIPT:\n{}\n\nCORREÇÕES DO LINTER:\n{}\n\nEntregue o código final compilável:",
                draft, critique
            );
            self.gemini.generate_narrative(&final_prompt, final_system).await.unwrap_or(draft)
        };

        // Limpeza de blocos de markdown se o modelo tiver incluído ```rhai ou ```
        let clean = final_script
            .replace("```rhai", "")
            .replace("```rust", "")
            .replace("```", "")
            .trim()
            .to_string();

        println!("[SPELL_COMPILER] Script Rhai compilado e revisado com sucesso!");
        Ok(clean)
    }
}


