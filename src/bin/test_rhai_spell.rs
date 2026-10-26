use dotenvy::dotenv;
use rhai::Dynamic;
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

    // 1. String natural da magia (entrada de premissa do usuário)
    let spell_prompt = "Magia: 'Toque da Geada Abissal'. O conjurador toca o alvo. O alvo deve passar em uma Salvaguarda de Constituição CD 14. Se falhar, sofre 3d8 de dano gélido e fica sob a condição 'Congelado' por 1 rodada. Se passar, sofre apenas metade do dano e não fica congelado.";

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
    let ast = match engine.compile(&rhai_code) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("❌ Erro de compilação Rhai: {}", e);
            return Err(e.to_string().into());
        }
    };
    println!("✓ Script Rhai compilado para AST com sucesso pelo Rust!");

    // 4. Instancia entidades de combate (Conjurador, Alvo e Contexto)
    let mut caster = TargetProxy::new("Mago Alistair", 45, 12);
    let mut target = TargetProxy::new("Troll da Caverna", 84, 15);
    let mut ctx = CombatContext::new();

    println!("\n🎯 ESTADO INICIAL DO COMBATE:");
    println!("- Conjurador: {} (PV: {}/{})", caster.name, caster.current_hp, caster.max_hp);
    println!("- Alvo: {} (PV: {}/{}, Condições: {:?})", target.name, target.current_hp, target.max_hp, target.conditions);

    // 5. Executa a função da magia deterministicamente no Rhai
    println!("\n⚡ 4. EXECUTANDO A MAGIA NO RHAI...");
    let mut scope = rhai::Scope::new();
    scope.push("caster", caster.clone());
    scope.push("target", target.clone());
    scope.push("ctx", ctx.clone());

    // Executa a função chamando target = cast_spell(caster, target, ctx);
    let full_script = format!("{}\ntarget = cast_spell(caster, target, ctx);", rhai_code);
    match engine.eval_with_scope::<Dynamic>(&mut scope, &full_script) {
        Ok(_) => println!("✓ Execução concluída sem erros no Rhai!"),
        Err(e) => eprintln!("❌ Erro na execução: {:?}", e),
    }

    // Recupera os objetos mutados pelo Rhai
    let target: TargetProxy = scope.get_value("target").unwrap();
    let ctx: CombatContext = scope.get_value("ctx").unwrap();

    println!("\n📊 5. LOG DE EVENTOS MECÂNICOS DO MOTOR RUST:");
    for log in &ctx.log {
        println!("  {}", log);
    }

    println!("\n🏁 ESTADO FINAL DO ALVO APÓS A MAGIA:");
    println!("- Alvo: {} (PV: {}/{})", target.name, target.current_hp, target.max_hp);
    println!("- Condições ativas: {:?}", target.conditions);

    assert!(target.current_hp < 84, "O PV do alvo deveria ter sido reduzido deterministicamente");
    println!("\n✅ SUCESSO TOTAL! A IA escreveu o algoritmo, e o Rust/Rhai aplicou o dano e a condição perfeitamente!");
    println!("===================================================================");

    Ok(())
}
