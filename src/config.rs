pub struct Config {
    pub version: String,
}

impl Config {}

pub fn load() -> Config {
    // TODO Load config.
    Config {
        version: "0.9.0".to_string(),
    }
}
