use config::Config;
use lazy_static::lazy_static;

// 定义配置文件路径
const CONFIG_TOML_PATH: &str = ".config/aigit/config.toml";

lazy_static! {
    // 只读全局变量示例
    pub static ref G_CONFIG: Config = init_aigit_config().unwrap();
    pub static ref G_AI_PLATFORM: String = G_CONFIG.get_string("platform").expect("Failed to get 'platform' from config.toml");
    pub static ref G_AI_MODEL: String = G_CONFIG.get_string("model").expect("Failed to get 'model' from config.toml");
    pub static ref G_AI_API_KEY: String = G_CONFIG.get_string("api_key").expect("Failed to get 'api_key' from config.toml");
}

// 读取 ${HOME}/.config/aigit/config.toml 配置文件
fn init_aigit_config() -> Result<Config, Box<dyn std::error::Error>>  {
    // 读取环境变量 HOME
    let home_dir = std::env::var("HOME")?;
    let config_path = format!("{}/{}", home_dir, CONFIG_TOML_PATH);

    let config: Config = config::Config::builder()
        .add_source(config::File::with_name(config_path.as_str()))
        .build().expect("Failed to load ${HOME}/.config/aigit/config.toml");

    return Ok(config);
}