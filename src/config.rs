

pub mod qma_config {
    use serde::{ Serialize, Deserialize };
    use serde_yaml;

    #[derive(Serialize, Deserialize)]
    pub struct Config {
        pub index: Index,
        pub fields: Option<Vec<Field>>
    }

    #[derive(Serialize, Deserialize)]
    pub struct Field {
        pub name: String,
        pub accessor: String,
        pub dtype: String,
        pub operation: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Index {
        pub name: String,
        pub accessor: String,
    }

    impl Config {
        pub fn parse(s: &str) -> Self{
            let config :Self = serde_yaml::from_str(s).expect("Failed to parse yaml");
            config
        }
    }


}
#[cfg(test)]
mod test {

    use super::qma_config::*;
    
    #[test]
    fn check_config() {
        let s = "
        index:
            name: key
            accessor: test.key
        fields:
           - name: field1
             accessor: test.value 
             dtype: string
             operation: OpCount
        ";

        let config = Config::parse(s);

        assert_eq!(&config.index.name, "key");
        assert_eq!(&config.index.accessor, "test.key");
        if let Some(fields) = &config.fields {
            assert_eq!(fields[0].name, "field1");
            assert_eq!(fields[0].accessor, "test.value");
            assert_eq!(fields[0].dtype, "string");
            assert_eq!(fields[0].operation, "OpCount");
        } else {
            unreachable!();
        }
    }
}