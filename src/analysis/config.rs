use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::env::current_exe;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process;
use toml;

#[derive(Deserialize, Debug)]
pub struct Bst {
    pub hostname: String,
    pub bst_db: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Fst {
    pub hostname: String,
    pub lcd_db: PathBuf,
    pub diag_db: PathBuf,
    pub key_db: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct TomlConfig {
    pub current_exe_path:PathBuf,
    pub current_config_path:PathBuf,
    pub current_db_path:PathBuf,
    pub port: i32,
    pub bst1: Bst,
    pub bst2: Bst,
    pub fst1: Fst,
    pub fst2: Fst,
}

impl TomlConfig {
    pub fn get_hostname(&self, line: &str) -> Option<&String> {
        match line {
            "bst1" => Some(&self.bst1.hostname),
            "bst2" => Some(&self.bst2.hostname),
            "fst1" => Some(&self.fst1.hostname),
            "fst2" => Some(&self.fst2.hostname),
            _ => None,
        }
    }

    pub fn get_db(&self, line: &str, station: &str) -> Option<&PathBuf> {
        match (line, station) {
            ("bst1", "BST") => Some(&self.bst1.bst_db),
            ("bst2", "BST") => Some(&self.bst2.bst_db),
            ("fst1", "LCDLED") => Some(&self.fst1.lcd_db),
            ("fst1", "DIAG") => Some(&self.fst1.diag_db),
            ("fst1", "KEYPAD") => Some(&self.fst1.key_db),
            ("fst2", "LCDLED") => Some(&self.fst2.lcd_db),
            ("fst2", "DIAG") => Some(&self.fst2.diag_db),
            ("fst2", "KEYPAD") => Some(&self.fst2.key_db),
            _ => None,
        }
    }
    pub fn get_all_db(&self) -> [(&'static str, &String, &PathBuf); 8] {
        [
            ("PCBDG", &self.bst1.hostname, &self.bst1.bst_db),
            ("PCBDG", &self.bst2.hostname, &self.bst2.bst_db),
            ("PCBINT", &self.fst1.hostname, &self.fst1.lcd_db),
            ("PCBINT", &self.fst1.hostname, &self.fst1.diag_db),
            ("PCBINT", &self.fst1.hostname, &self.fst1.key_db),
            ("PCBINT", &self.fst2.hostname, &self.fst2.lcd_db),
            ("PCBINT", &self.fst2.hostname, &self.fst2.diag_db),
            ("PCBINT", &self.fst2.hostname, &self.fst2.key_db),
        ]
    }
}

pub static CONFIG: OnceCell<TomlConfig> = OnceCell::new();

pub fn init_config() {
    let exe_path = current_exe().unwrap();
    println!("path of this running program: {exe_path:?}       --exist");
    let toml_path = exe_path.parent().unwrap().join("config.toml");
    print!("expected path of config.toml: {toml_path:?}");
    if !toml_path.exists() {
        println!("  --Error: not exist !");
        process::exit(0);
    } else {
        println!("  --exist");
    }
    let db_dir = exe_path.parent().unwrap().join("db");
    print!("expected path of db file dir: {db_dir:?}");
    if !db_dir.exists() {
        println!("  --Error: not exist !");
        process::exit(0);
    } else {
        println!("           --exist");
    }
    let mut toml = match File::open(&toml_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: open config.toml file fail, {}", e);
            process::exit(0);
        }
    };
    let mut contents = String::new();

    if let Err(e) = toml.read_to_string(&mut contents) {
        eprintln!("Error: read config.toml content fail, {}", e);
        process::exit(0);
    }

    let mut config: TomlConfig = match toml::from_str(&contents) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: deserialize config.toml fail: {}", e);
            process::exit(0);
        }
    };
    config.bst1.bst_db = db_dir.join("bst1").join(config.bst1.bst_db);
    config.bst2.bst_db = db_dir.join("bst2").join(config.bst2.bst_db);
    config.fst1.lcd_db = db_dir.join("fst1").join(config.fst1.lcd_db);
    config.fst1.diag_db = db_dir.join("fst1").join(config.fst1.diag_db);
    config.fst1.key_db = db_dir.join("fst1").join(config.fst1.key_db);
    config.fst2.lcd_db = db_dir.join("fst2").join(config.fst2.lcd_db);
    config.fst2.diag_db = db_dir.join("fst2").join(config.fst2.diag_db);
    config.fst2.key_db = db_dir.join("fst2").join(config.fst2.key_db);
    config.current_exe_path = exe_path;
    config.current_config_path = toml_path;
    config.current_db_path = db_dir;
    // println!("toml config is: {:?}", config);
    CONFIG.get_or_init(|| config);
}
