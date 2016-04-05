extern crate yaml_rust;
use std::fs::File;
use yaml_rust::YamlLoader;
use std::io::prelude::*;

pub type SettingsSource = yaml_rust::Yaml;

pub fn read() -> SettingsSource {
    let mut s = String::new();
    let mut f = File::open("config.yml").expect("open config file");
    f.read_to_string(&mut s).expect("read config file");
    let settings_docs = YamlLoader::load_from_str(&s).unwrap();
    if settings_docs.len() != 1
    {
        panic!("Settings file has wrong number of YAML documents, expected 1.");
    }
    assert_eq!(settings_docs.len(), 1);
    settings_docs[0].to_owned()
}
