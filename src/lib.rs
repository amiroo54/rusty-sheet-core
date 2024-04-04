pub mod mods;
pub use mods::{*};
use serde_json::json;

use std::{collections::hash_map::HashMap, fs::{self, File}, io::{BufReader, Write}, path::Path};

fn load_file(path: &Path, race_list: &mut HashMap<Uuid, Race>, class_list: &mut HashMap<Uuid, Class>, item_list: &mut HashMap<Uuid, Item>, subclass_list: &mut HashMap<Uuid, SubClass>, character_list: &mut HashMap<Uuid, Character>)
{
    if let Ok(file) = File::open(&path) {
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
        
        if let Some(subclasses) = json_data.get("subclasses") {
            *subclass_list = serde_json::from_value(subclasses.clone()).expect("Unable to parse races data");
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


pub fn save_file(path: &Path, race_list: &HashMap<Uuid, Race>, class_list: &HashMap<Uuid, Class>, item_list: &HashMap<Uuid, Item>, subclass_list: &HashMap<Uuid, SubClass>)
{
    let json_data = json!({
        "races": race_list, "classes": class_list, "items": item_list, "subclasses": subclass_list
    });

    let mut file = File::create(path).expect("File creation failed");
    file.write_all(serde_json::to_string_pretty(&json_data).unwrap().as_bytes()).expect("Writing to file failed");
}



pub fn save_characters(path: &Path, character_list: &HashMap<Uuid, Character>)
{
    let json_data = json!({
        "characters": character_list
    });

    let mut file = File::create(path).expect("File creation failed");
    file.write_all(serde_json::to_string_pretty(&json_data).unwrap().as_bytes()).expect("Writing to file failed");
}

pub fn load_files(file_folder: &Path, race_list: &mut HashMap<Uuid, Race>, class_list: &mut HashMap<Uuid, Class>, item_list: &mut HashMap<Uuid, Item>, subclass_list: &mut HashMap<Uuid, SubClass>, character_list: &mut HashMap<Uuid, Character>)
{
    if let Ok(entries) = fs::read_dir(file_folder) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                // Check if the file is a regular file before processing
                if file_path.is_file() {
                    // Call the provided function on the file
                    load_file(&file_path, race_list, class_list, item_list, subclass_list, character_list);
                }
            }
        }
    }
}