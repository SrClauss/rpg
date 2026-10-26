use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub role: String, // "admin" | "host" | "player"
    pub plan: String, // "free" | "host_pro"
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPublic {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub plan: String,
    pub created_at: String,
}

impl From<&User> for UserPublic {
    fn from(u: &User) -> Self {
        Self {
            id: u.id.clone(),
            email: u.email.clone(),
            name: u.name.clone(),
            role: u.role.clone(),
            plan: u.plan.clone(),
            created_at: u.created_at.clone(),
        }
    }
}

pub struct UserStore {
    data_dir: String,
    // email (lowercase) -> User
    users_by_email: Arc<RwLock<HashMap<String, User>>>,
    // token -> user_id
    active_tokens: Arc<RwLock<HashMap<String, String>>>,
}

impl UserStore {
    pub fn new(data_dir: &str) -> Self {
        let store = Self {
            data_dir: data_dir.to_string(),
            users_by_email: Arc::new(RwLock::new(HashMap::new())),
            active_tokens: Arc::new(RwLock::new(HashMap::new())),
        };
        store.load_from_disk();
        store.ensure_default_admin();
        store
    }

    fn load_from_disk(&self) {
        let path = Path::new(&self.data_dir);
        if !path.exists() {
            let _ = fs::create_dir_all(path);
            return;
        }

        if let Ok(entries) = fs::read_dir(path) {
            let mut map = HashMap::new();
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&p) {
                        if let Ok(user) = serde_json::from_str::<User>(&content) {
                            map.insert(user.email.to_lowercase(), user);
                        }
                    }
                }
            }
            if let Ok(mut lock) = self.users_by_email.try_write() {
                *lock = map;
            }
        }
    }

    fn ensure_default_admin(&self) {
        let admin_email = std::env::var("ADMIN_DEFAULT_EMAIL").unwrap_or_else(|_| "admin@rpg.local".to_string());
        let admin_pass = std::env::var("ADMIN_DEFAULT_PASSWORD").unwrap_or_else(|_| "admin123".to_string());

        let exists = {
            if let Ok(lock) = self.users_by_email.try_read() {
                lock.contains_key(&admin_email.to_lowercase())
            } else {
                false
            }
        };

        if !exists {
            let _ = self.create_user_internal(
                &admin_email,
                "Administrador Mestre",
                &admin_pass,
                "admin",
                "host_pro",
            );
            println!(">>> [AUTH] Usuário Admin padrão criado: {}", admin_email);
        }
    }

    pub fn hash_password(password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| format!("Erro ao gerar hash de senha: {}", e))
    }

    pub fn verify_password(password: &str, password_hash: &str) -> bool {
        if let Ok(parsed_hash) = PasswordHash::new(password_hash) {
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok()
        } else {
            false
        }
    }

    fn create_user_internal(
        &self,
        email: &str,
        name: &str,
        password: &str,
        role: &str,
        plan: &str,
    ) -> Result<User, String> {
        let email_norm = email.trim().to_lowercase();
        let pass_hash = Self::hash_password(password)?;
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            email: email_norm.clone(),
            name: name.trim().to_string(),
            password_hash: pass_hash,
            role: role.to_string(),
            plan: plan.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        let file_path = format!("{}/{}.json", self.data_dir, user.id);
        let _ = fs::create_dir_all(&self.data_dir);
        let json_str = serde_json::to_string_pretty(&user).map_err(|e| e.to_string())?;
        fs::write(&file_path, json_str).map_err(|e| e.to_string())?;

        if let Ok(mut lock) = self.users_by_email.try_write() {
            lock.insert(email_norm, user.clone());
        }

        Ok(user)
    }

    pub async fn register(
        &self,
        email: &str,
        name: &str,
        password: &str,
        role: Option<&str>,
    ) -> Result<(UserPublic, String), String> {
        let email_norm = email.trim().to_lowercase();
        if email_norm.is_empty() || !email_norm.contains('@') {
            return Err("Endereço de email inválido.".to_string());
        }
        if password.len() < 4 {
            return Err("A senha deve possuir pelo menos 4 caracteres.".to_string());
        }

        {
            let lock = self.users_by_email.read().await;
            if lock.contains_key(&email_norm) {
                return Err("Já existe uma conta cadastrada com este email.".to_string());
            }
        }

        let chosen_role = role.unwrap_or("host");
        // Previne cadastro comum assumir role admin
        let final_role = if chosen_role == "admin" { "host" } else { chosen_role };
        let plan = if final_role == "host" { "host_pro" } else { "free" };

        let pass_hash = Self::hash_password(password)?;
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            email: email_norm.clone(),
            name: name.trim().to_string(),
            password_hash: pass_hash,
            role: final_role.to_string(),
            plan: plan.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        let file_path = format!("{}/{}.json", self.data_dir, user.id);
        let _ = fs::create_dir_all(&self.data_dir);
        let json_str = serde_json::to_string_pretty(&user).map_err(|e| e.to_string())?;
        fs::write(&file_path, json_str).map_err(|e| e.to_string())?;

        {
            let mut lock = self.users_by_email.write().await;
            lock.insert(email_norm, user.clone());
        }

        let token = uuid::Uuid::new_v4().to_string();
        {
            let mut tokens = self.active_tokens.write().await;
            tokens.insert(token.clone(), user.id.clone());
        }

        Ok((UserPublic::from(&user), token))
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<(UserPublic, String), String> {
        let email_norm = email.trim().to_lowercase();
        let user = {
            let lock = self.users_by_email.read().await;
            lock.get(&email_norm).cloned()
        };

        match user {
            Some(u) => {
                if u.role == "admin" {
                    return Err("Contas administrativas devem acessar exclusivamente pelo Portal do Administrador.".to_string());
                }
                if Self::verify_password(password, &u.password_hash) {
                    let token = uuid::Uuid::new_v4().to_string();
                    let mut tokens = self.active_tokens.write().await;
                    tokens.insert(token.clone(), u.id.clone());
                    Ok((UserPublic::from(&u), token))
                } else {
                    Err("Credenciais inválidas: senha incorreta.".to_string())
                }
            }
            None => Err("Nenhuma conta de Host ou Jogador encontrada com o email informado.".to_string()),
        }
    }

    pub async fn login_admin(&self, email: &str, password: &str) -> Result<(UserPublic, String), String> {
        let email_norm = email.trim().to_lowercase();
        let user = {
            let lock = self.users_by_email.read().await;
            lock.get(&email_norm).cloned()
        };

        match user {
            Some(u) => {
                if u.role != "admin" {
                    return Err("Acesso negado: esta credencial não possui privilégios de Administrador.".to_string());
                }
                if Self::verify_password(password, &u.password_hash) {
                    let token = uuid::Uuid::new_v4().to_string();
                    let mut tokens = self.active_tokens.write().await;
                    tokens.insert(token.clone(), u.id.clone());
                    Ok((UserPublic::from(&u), token))
                } else {
                    Err("Credenciais administrativas inválidas: senha incorreta.".to_string())
                }
            }
            None => Err("Nenhuma conta administrativa encontrada com este email.".to_string()),
        }
    }

    pub async fn get_user_by_token(&self, token: &str) -> Option<UserPublic> {
        let tokens = self.active_tokens.read().await;
        if let Some(user_id) = tokens.get(token) {
            let users = self.users_by_email.read().await;
            for u in users.values() {
                if &u.id == user_id {
                    return Some(UserPublic::from(u));
                }
            }
        }
        None
    }

    pub async fn logout(&self, token: &str) {
        let mut tokens = self.active_tokens.write().await;
        tokens.remove(token);
    }

    pub async fn list_users(&self) -> Vec<UserPublic> {
        let users = self.users_by_email.read().await;
        users.values().map(UserPublic::from).collect()
    }
}
