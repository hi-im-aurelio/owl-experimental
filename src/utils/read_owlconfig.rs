use std::collections::HashMap;
use std::fs;
use std::io::{self};

pub struct OWLConfig {
    config: HashMap<String, String>,
}

impl OWLConfig {
    // Cria uma nova instância da configuração
    pub fn new(path: &str) -> io::Result<Self> {
        let mut config = HashMap::new();

        let contents = fs::read_to_string(path)?;

        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut parts = line.splitn(2, ' ');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                config.insert(key.to_string(), value.to_string());
            }
        }

        Ok(OWLConfig { config })
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }

    pub fn interval(&self) -> Option<&String> {
        self.get("interval")
    }

    pub fn model(&self) -> Option<&String> {
        self.get("model")
    }

    pub fn remote(&self) -> Option<&String> {
        self.get("remote")
    }
}

static OWLCONFIG_PATH: &str = "./src/.owlconfig";
fn read_owlconfig() {
    let owl_config = OWLConfig::new(OWLCONFIG_PATH);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_read_owlconfig() {
//         let r = read_owlconfig();

//         match r {
//             Ok(config) => {
//                 assert_eq!(config.get("interval"), Some(&"2".to_string()));
//                 assert_eq!(
//                     config.get("model"),
//                     Some(&"/home/user/project/original/path".to_string())
//                 );
//                 assert_eq!(
//                     config.get("remote"),
//                     Some(&"https://github.com/fariosofernando/authentication.git".to_string())
//                 );
//             }
//             Err(err) => panic!("Failed to read config: {}", err),
//         }
//     }
// }
