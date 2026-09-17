# Plano de Resolução de Bug: Injeção de Escala no Prompt do Gemini

## Descrição do Bug
**Sintomas:** A imagem gerada pelo Gemini não está recebendo a informação sobre as dimensões do grid tático.
**Comportamento Esperado:** O prompt final enviado à API do Gemini deve conter uma string adicional informando o `width` e `height` do grid para estabelecer a escala (ex: `This terrain represents an area of X by Y D&D units...`).
**Comportamento Atual:** O prompt recebido do DeepSeek está sendo repassado puro para a API do Gemini, perdendo o contexto de escala.

## Causa Raiz Identificada
Durante a restauração do código para a "Grade Infinita" e implementação do sistema de metadados no arquivo `src/gemini.rs` (função `get_or_generate_battle_map_images`), a variável `prompt` original foi passada diretamente para `self.generate_image(prompt)`, em vez de sofrer uma concatenação (`format!`) com as informações de largura e altura. A linha com a regra de negócio foi acidentalmente omitida no patch anterior.

## Abordagens Tentadas/Consideradas
- **Abordagem 1 (Passado):** Fazer a concatenação da string dentro de `get_or_generate_battle_map_images`. (Essa é a correta, mas foi apagada).
- **Abordagem 2:** Pedir para o DeepSeek já injetar a dimensão no seu JSON final. Descartado, pois é mais confiável que o motor Rust injete os números precisos de forma imutável antes da chamada de rede.

## Plano de Ação Passo a Passo
1. **Modificar `src/gemini.rs`:**
   - Localizar a função `get_or_generate_battle_map_images` (aprox. linha 134).
   - Substituir a chamada direta `let image_bytes = self.generate_image(prompt).await?;` por:
     ```rust
     let scaled_prompt = format!(
         "{} This terrain represents an area of {} by {} D&D units (each unit is a 5ft / 1.5m space). Ensure the scale of objects, trees, and obstacles matches this perspective.",
         prompt, width_hexes, height_hexes
     );
     let image_bytes = self.generate_image(&scaled_prompt).await?;
     ```
2. **Validar:**
   - Executar `cargo check` para garantir que as tipagens batem.
   - O código não afetará testes existentes pois o retorno de `get_or_generate_battle_map_images` continua inalterado.
