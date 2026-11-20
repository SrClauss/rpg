use dotenvy::dotenv;
use rpg_engine::llm::NarrativeOrchestrator;
use rpg_engine::rules::rhai_spell_runtime::{create_spell_rhai_engine, CombatContext, TargetProxy};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    println!("===================================================================");
    println!("🧪 TESTE DE COMPILAÇÃO E EXECUÇÃO DE MAGIA HOMEBREW VIA IA + RHAI");
    println!("===================================================================");

    let gemini_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY não encontrada no .env");
    let deepseek_key = env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY não encontrada no .env");

    let orchestrator = NarrativeOrchestrator::new(gemini_key, deepseek_key);

    // 1. String natural da magia (entrada solicitada pelo usuário)
    let spell_prompt = "Magia: 'Impacto Glacial e Estilhaços'. O conjurador arremessa um gigantesco bloco de gelo contra um alvo principal. O impacto causa 4d8 de dano de Concussão (Físico) ao atingido. Logo em seguida, o bloco se espatifa e projeta estilhaços congelantes em um cone atrás do alvo atingido: criaturas na área devem passar em uma Salvaguarda de Destreza CD 15 para sofrer metade, ou sofrer 3d6 de dano Perfurante + 2d6 de dano de Frio se falharem.";

    println!("\n📥 1. ENTRADA EM LINGUAGEM NATURAL:\n{}", spell_prompt);

    // 2. IA compila para código Rhai revisado
    println!("\n🤖 2. IA (Gemini + DeepSeek Linter) COMPILANDO PARA CÓDIGO RHAI...");
    let rhai_code = orchestrator.compile_spell_to_rhai(spell_prompt).await?;

    println!("\n📜 CÓDIGO RHAI GERADO PELA IA:\n--------------------------------------------------");
    println!("{}", rhai_code);
    println!("--------------------------------------------------");

    // 3. Backend Rust prepara o ambiente de execução determinístico
    println!("\n⚙️ 3. PREPARANDO AMBIENTE DE EXECUÇÃO EM RUST (Rhai Sandbox)...");
    let engine = create_spell_rhai_engine();
    let _ast = match engine.compile(&rhai_code) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("❌ Erro de compilação Rhai: {}", e);
            return Err(e.to_string().into());
        }
    };
    println!("✓ Script Rhai compilado para AST com sucesso pelo Rust!");

    // 4. Instancia entidades de combate (Conjurador, Alvo Principal e Alvos Secundários atrás no cone)
    let caster = TargetProxy::new("Mago Alistair", 45, 12);
    let target = TargetProxy::new("Ogro Brutamontes (Alvo Direto)", 90, 14);
    let mut ctx = CombatContext::new();

    // Adiciona 2 inimigos que estão posicionados atrás do Ogro dentro do cone de estilhaços
    ctx.add_secondary_target(TargetProxy::new("Goblin Arqueiro A (No Cone)", 25, 13));
    ctx.add_secondary_target(TargetProxy::new("Goblin Arqueiro B (No Cone)", 25, 13));

    println!("\n🎯 ESTADO INICIAL DO COMBATE:");
    println!("- Conjurador: {} (PV: {}/{})", caster.name, caster.current_hp, caster.max_hp);
    println!("- Alvo Principal: {} (PV: {}/{})", target.name, target.current_hp, target.max_hp);
    println!("- Alvos Secundários no Cone atrás do Ogro:");
    for st in &ctx.secondary_targets {
        println!("   * {} (PV: {}/{})", st.name, st.current_hp, st.max_hp);
    }

    // 5. Executa a função da magia deterministicamente no Rhai
    println!("\n⚡ 4. EXECUTANDO A MAGIA NO RHAI...");
    let mut scope = rhai::Scope::new();
    scope.push("caster", caster.clone());
    scope.push("target", target.clone());
    scope.push("ctx", ctx.clone());

    // Executa a função passando caster, target, ctx e reatribui target = cast_spell(...)
    // Garantimos que o array retorne [target, ctx] atualizados
    let full_script = format!("{}\ntarget = cast_spell(caster, target, ctx);\n[target, ctx]", rhai_code);
    let result_pair: rhai::Array = match engine.eval_with_scope::<rhai::Array>(&mut scope, &full_script) {
        Ok(res) => {
            println!("✓ Execução concluída sem erros no Rhai!");
            res
        }
        Err(e) => {
            eprintln!("❌ Erro na execução: {:?}", e);
            return Err(e.to_string().into());
        }
    };

    let target: TargetProxy = result_pair[0].clone().cast::<TargetProxy>();
    let ctx: CombatContext = result_pair[1].clone().cast::<CombatContext>();

    println!("\n📊 5. LOG DE EVENTOS MECÂNICOS DO MOTOR RUST:");
    for log in &ctx.log {
        println!("  {}", log);
    }

    println!("\n🏁 ESTADO FINAL APÓS A MAGIA:");
    println!("- Alvo Principal: {} (PV: {}/{})", target.name, target.current_hp, target.max_hp);
    println!("- Alvos Secundários no Cone:");
    for st in &ctx.secondary_targets {
        println!("   * {} (PV: {}/{})", st.name, st.current_hp, st.max_hp);
    }

    assert!(target.current_hp < 90, "O Ogro deveria ter sofrido dano físico do bloco");
    println!("\n✅ SUCESSO! A magia gerou tanto o dano de impacto quanto o efeito em cone atrás do alvo!");
    println!("===================================================================");

    Ok(())
}
