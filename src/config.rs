use figment::providers::{Env, Format, Toml};
use figment::Figment;
use secrecy::SecretString;
use serde::Deserialize;
use serenity::all::UserId;

static ENV_PREFIX: &'static str = "TG_";
static ENV_CONFIG_FILE: &'static str = "TG_CONFIG_FILE";
static DEFAULT_CONFIG_FILE: &'static str = "toronto-guard.toml";

impl Config {
    pub(crate) fn load() -> figment::error::Result<Config> {
        let config_file = std::env::var(ENV_CONFIG_FILE).unwrap_or(DEFAULT_CONFIG_FILE.into());

        Figment::from(Toml::file(config_file))
            .merge(Env::prefixed(ENV_PREFIX).split("_").lowercase(false))
            .extract()
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub bot: BotConfig,
    pub mason: MasonConfig,
    #[serde(default)]
    pub log: LogConfig,
}

#[derive(Deserialize)]
pub struct BotConfig {
    pub token: SecretString,
}

fn default_user_cooldown() -> u64 { 600 }
fn default_global_cooldown() -> u64 { 300 }

#[derive(Deserialize)]
pub struct MasonConfig {
    pub user_id: UserId,
    #[serde(default = "default_user_cooldown")]
    pub user_cooldown_sec: u64,
    #[serde(default = "default_global_cooldown")]
    pub global_cooldown_sec: u64,
}

#[derive(Deserialize)]
pub struct LogConfig {
    pub filter: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        LogConfig {
            filter: "toronto-guard=info".into(),
        }
    }
}
