pub mod mods;
pub use mods::{*};
use serde_json::json;

use std::{collections::hash_map::HashMap, fs::File, io::{BufReader, Write}, path::Path};

pub fn load_file(file_folder: &Path, race_list: &mut HashMap<Uuid, Race>, class_list: &mut HashMap<Uuid, Class>, item_list: &mut HashMap<Uuid, Item>, character_list: &mut HashMap<Uuid, Character>)
{
    let file_path = file_folder.join("data.json");
    if let Ok(file) = File::open(&file_path) {
        let reader = BufReader::new(file);
        let json_data: serde_json::Value = serde_json::from_reader(reader).expect("Unable to parse JSON data");

        if let Some(races) = json_data.get("races") {
            *race_list = serde_json::from_value(races.clone()).expect("Unable to parse races data");
        }
        
        if let Some(classes) = json_data.get("classes") {
            *class_list = serde_json::from_value(classes.clone()).expect("Unable to parse classes data");
        }
        
        if let Some(items) = json_data.get("items") {
            *item_list = serde_json::from_value(items.clone()).expect("Unable to parse items data");
        }
        
        if let Some(characters) = json_data.get("characters") {
            *character_list = serde_json::from_value(characters.clone()).expect("Unable to parse characters data");
            for character in character_list
            {
                character.1.populate_from_ids(&class_list, &race_list, &item_list);
            }
        }
    } else {
        eprintln!("File not found or could not be opened.");
    }
}

pub fn save_file(file_folder: &Path, race_list: &HashMap<Uuid, Race>, class_list: &HashMap<Uuid, Class>, item_list: &HashMap<Uuid, Item>, character_list: &HashMap<Uuid, Character>)
{
    let json_data = json!({
        "races": race_list, "classes": class_list, "items": item_list, "characters": character_list
    });

    let mut file = File::create(file_folder.join("data.json")).expect("File creation failed");
    file.write_all(serde_json::to_string_pretty(&json_data).unwrap().as_bytes()).expect("Writing to file failed");
}