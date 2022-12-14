use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub bookname: String,
    pub author: String,
    pub description: String,
    #[serde(rename = "assetsDir")]
    pub assets_dir: String,
    #[serde(rename = "rootDir")]
    pub root_dir: String,
    #[serde(rename = "distDir")]
    pub dist_dir: String,
    pub pages: Vec<Page>,
}

impl Config {
    pub fn new() -> Self {
        let json_config = include_str!("../assets/book.json");
        serde_json::from_str(json_config).unwrap()
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let json_config = fs::read_to_string(path).unwrap();
        serde_json::from_str(&json_config).unwrap()
    }

    pub fn update_author(&mut self, author: String) {
        self.author = author
    }

    pub fn update_bookname(&mut self, name: String) {
        self.bookname = name
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) {
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write(path, json).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub title: String,
    pub path: String,
}
