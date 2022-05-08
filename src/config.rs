use std::{env, fs};

use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub oauth: Option<OAuthConfig>,
    pub bot: Option<BotAuthConfig>,
}

#[derive(Debug, Deserialize)]
pub struct OAuthConfig {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BotAuthConfig {
    pub token: Option<String>,
}

impl Config {
    pub fn load_config() {
        let proj_dirs = ProjectDirs::from("", "", "discordtools").unwrap();
        let config_file = proj_dirs.config_dir().join("config.toml");

        if let Ok(cfg_data) = fs::read_to_string(config_file) {
            if let Ok(conf) = toml::from_str::<Self>(&cfg_data) {
                cfg_to_env(&conf, "CLIENT_ID", |c: &Config| {
                    c.oauth.as_ref().map(|o| o.client_id.as_ref())
                });
                cfg_to_env(&conf, "CLIENT_SECRET", |c: &Config| {
                    c.oauth.as_ref().map(|o| o.client_secret.as_ref())
                });
                cfg_to_env(&conf, "DISCORD_TOKEN", |c: &Config| {
                    c.bot.as_ref().map(|o| o.token.as_ref())
                });
            }
        }
    }
}

fn cfg_to_env<F>(cfg: &Config, env_name: &'static str, val_fn: F)
where
    F: FnOnce(&Config) -> Option<Option<&String>>,
{
    if let Err(env::VarError::NotPresent) = env::var(env_name) {
        if let Some(Some(val)) = val_fn(&cfg) {
            env::set_var(env_name, val);
        }
    }
}
