

use serde::{ Deserialize };


#[derive(Deserialize)]
pub struct Config {
    key: String
}

impl Config {
    pub fn parse(s: &str) -> Self{
        let config :Self = serde_yaml::from_str(&s).expect("Failed to parse yaml");
        config
    }
}

#[cfg(test)]
mod test {

    use super::*;
    
    #[test]
    fn check_config() {
        let s = "key: test";
        let config = Config::parse(&s);
        assert_eq!(&config.key, "test");
    }
}