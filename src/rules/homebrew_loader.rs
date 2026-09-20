use base64::{prelude::*, Engine as Base64Engine};
use rhai::Engine;
use serde::{de::DeserializeOwned, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

pub const SECRET_KEY: &[u8] = b"RpgEngineSuperSecretKey2026!@";

/// Encripta o código Rhai em texto limpo para o formato ilegível da engine
pub fn encrypt_script(plaintext: &str) -> String {
    let mut xored = Vec::with_capacity(plaintext.len());
    for (i, &b) in plaintext.as_bytes().iter().enumerate() {
        xored.push(b ^ SECRET_KEY[i % SECRET_KEY.len()]);
    }
    BASE64_STANDARD.encode(xored)
}

/// Decripta o arquivo `.hb` embaralhado de volta para código Rhai nativo
pub fn decrypt_script(cipher_b64: &str) -> Result<String, String> {
    let decoded = BASE64_STANDARD
        .decode(cipher_b64)
        .map_err(|e| format!("Base64 error: {}", e))?;
    let mut xored = Vec::with_capacity(decoded.len());
    for (i, &b) in decoded.iter().enumerate() {
        xored.push(b ^ SECRET_KEY[i % SECRET_KEY.len()]);
    }
    String::from_utf8(xored).map_err(|e| format!("UTF8 error: {}", e))
}

/// Cria uma engine Rhai pré-configurada
pub fn create_rhai_engine() -> Engine {
    let mut engine = Engine::new();
    engine.set_max_expr_depths(0, 0); // Desativa limites de profundidade para mapas grandes
    engine
}

/// Lê um .hb ENCRIPTADO, decripta na memória, executa no Rhai, extrai a Struct do D&D e salva ela direto em binário .chb
pub fn compile_to_binary<T: DeserializeOwned + Serialize>(
    source_path: &str,
    dest_path: &str,
    init_func_name: &str,
) -> Result<(), String> {
    let mut file = File::open(source_path).map_err(|e| format!("Erro abrir .hb: {}", e))?;
    let mut cipher_text = String::new();
    file.read_to_string(&mut cipher_text)
        .map_err(|e| format!("Erro ler .hb: {}", e))?;

    let plain_rhai = decrypt_script(&cipher_text)?;

    let engine = create_rhai_engine();
    match engine.compile(plain_rhai) {
        Ok(ast) => {
            let mut scope = rhai::Scope::new();
            match engine.call_fn::<rhai::Dynamic>(&mut scope, &ast, init_func_name, ()) {
                Ok(dynamic_val) => {
                    let parsed_struct: T = rhai::serde::from_dynamic(&dynamic_val)
                        .map_err(|e| format!("Erro ao converter para Rust: {}", e))?;

                    let encoded = bincode::serialize(&parsed_struct)
                        .map_err(|e| format!("Erro no bincode: {}", e))?;

                    let mut file = File::create(dest_path)
                        .map_err(|e| format!("Erro ao criar arquivo: {}", e))?;
                    file.write_all(&encoded)
                        .map_err(|e| format!("Erro ao gravar binário: {}", e))?;

                    Ok(())
                }
                Err(e) => Err(format!("Erro de execução Rhai: {}", e)),
            }
        }
        Err(e) => Err(format!("Erro ao compilar Rhai: {}", e)),
    }
}

/// Lê uma pasta e carrega todos os scripts .chb (estruturas binárias puras)
pub fn load_homebrew_from_dir<T: DeserializeOwned>(
    dir_path: &str,
    _init_func_name: &str,
) -> Vec<T> {
    let mut results = Vec::new();
    let path = Path::new(dir_path);
    if !path.exists() || !path.is_dir() {
        return results;
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if let Some(ext) = file_path.extension() {
                if ext == "chb" {
                    let mut file = match File::open(&file_path) {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("Erro ao abrir arquivo binário {:?}: {}", file_path, e);
                            continue;
                        }
                    };

                    let mut buffer = Vec::new();
                    if let Err(e) = file.read_to_end(&mut buffer) {
                        eprintln!("Erro ao ler binário {:?}: {}", file_path, e);
                        continue;
                    }

                    // Desserializa diretamente a Struct limpa
                    let obj: T = match bincode::deserialize(&buffer) {
                        Ok(a) => a,
                        Err(e) => {
                            eprintln!("Erro ao desserializar obj em {:?}: {}", file_path, e);
                            continue;
                        }
                    };
                    results.push(obj);
                }
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::equipment::armor::Armor;
    use crate::rules::equipment::weapons::Weapon;
    use crate::rules::spells::spell::Spell;

    #[test]
    fn test_compile_and_load_homebrew() {
        // Como o sistema espera .hb encriptados, vamos pegar o código aberto dos arquivos criados, encriptá-los, e regravar no arquivo.

        let encrypt_file = |path: &str| {
            let plain = fs::read_to_string(path).unwrap();
            // Evita re-encriptar se já estiver em base64 (não possui espaços/quebras de linha na primeira palavra)
            if !plain.starts_with("fn ") {
                return;
            }

            let cipher = encrypt_script(&plain);
            let mut f = File::create(path).unwrap();
            f.write_all(cipher.as_bytes()).unwrap();
        };

        encrypt_file("data/homebrew/spells/lamina_do_trovao.hb");
        encrypt_file("data/homebrew/weapons/laser_sword.hb");
        encrypt_file("data/homebrew/armors/mithral_plate.hb");

        // Compilar spells (agora ele lê o .hb encriptado e gera o .chb binário)
        let res = compile_to_binary::<Spell>(
            "data/homebrew/spells/lamina_do_trovao.hb",
            "data/homebrew/spells/lamina_do_trovao.chb",
            "get_spell",
        );
        assert!(res.is_ok(), "Falha ao compilar spell: {:?}", res);

        // Compilar weapons
        let res2 = compile_to_binary::<Weapon>(
            "data/homebrew/weapons/laser_sword.hb",
            "data/homebrew/weapons/laser_sword.chb",
            "get_weapon",
        );
        assert!(res2.is_ok(), "Falha ao compilar weapon: {:?}", res2);

        // Compilar armors
        let res3 = compile_to_binary::<Armor>(
            "data/homebrew/armors/mithral_plate.hb",
            "data/homebrew/armors/mithral_plate.chb",
            "get_armor",
        );
        assert!(res3.is_ok(), "Falha ao compilar armor: {:?}", res3);
    }
}
