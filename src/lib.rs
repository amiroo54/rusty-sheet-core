pub mod mods;
pub use mods::{*};
use serde_json::json;

use std::{collections::hash_map::HashMap, fs::{self, File}, io::{BufReader, Write}, ops::BitAndAssign, path::Path};


pub struct Data
{
    pub race_list: HashMap<Uuid, Race>,
    pub class_list: HashMap<Uuid, Class>,
    pub item_list: HashMap<Uuid, Item>,
    pub character_list: HashMap<Uuid, Character>,
}
impl Data {
    pub fn new() -> Self
    {
        Data
        {
            race_list: HashMap::new(),
            class_list: HashMap::new(),
            item_list: HashMap::new(),
            character_list: HashMap::new()
        }
    }
    fn load_file(path: &Path, data: &mut Data)
    {
        if let Ok(file) = File::open(&path) {
            let reader = BufReader::new(file);
            let json_data: serde_json::Value = serde_json::from_reader(reader).expect("Unable to parse JSON data");

            if let Some(races) = json_data.get("races") {
                data.race_list = serde_json::from_value(races.clone()).expect("Unable to parse races data");
            }
            
            if let Some(classes) = json_data.get("classes") {
                data.class_list = serde_json::from_value(classes.clone()).expect("Unable to parse classes data");
            }
            
            if let Some(items) = json_data.get("items") {
                data.item_list = serde_json::from_value(items.clone()).expect("Unable to parse items data");
            }
            
            if let Some(subclasses) = json_data.get("subclasses") {
                let subclass_list: HashMap<Uuid, SubClass> = serde_json::from_value(subclasses.clone()).expect("Unable to parse races data");
                for sub in subclass_list
                {
                    println!("{}", &sub.0);
                    data.class_list
                    .get_mut(&sub.1.class_id)
                    .unwrap()
                    .subclasses.insert(sub.0.clone(), sub.1);
                }
            }

            if let Some(characters) = json_data.get("characters") {
                let mut character_list: HashMap<Uuid, Character> = serde_json::from_value(characters.clone()).expect("Unable to parse characters data");
                for character in character_list.iter_mut()
                {
                    character.1.populate_from_ids(&data.class_list, &data.race_list, &data.item_list);
                }
                data.character_list = character_list;
            }
        } else {
            eprintln!("File not found or could not be opened.");
        }
    }


    pub fn save_file(&self, path: &Path)
    {
        let mut subclass_list: HashMap<Uuid, SubClass> = HashMap::new();
        
        for class in &self.class_list
        {
            for sub in &class.1.subclasses
            {
                subclass_list.insert(sub.0.clone(), sub.1.clone());
            }
        }
        
        let json_data = json!({
            "races": self.race_list, "classes": self.class_list, "items": self.item_list, "subclasses": subclass_list
        });

        let mut file = File::create(path).expect("File creation failed");
        file.write_all(serde_json::to_string_pretty(&json_data).unwrap().as_bytes()).expect("Writing to file failed");
    }

    pub fn save_characters(&self, path: &Path)
    {
        let json_data = json!({
            "characters": self.character_list
        });

        let mut file = File::create(path).expect("File creation failed");
        file.write_all(serde_json::to_string_pretty(&json_data).unwrap().as_bytes()).expect("Writing to file failed");
    }

    pub fn load_files(file_folder: &Path) -> Self
    {
        let mut data = Data::new();
        if let Ok(entries) = fs::read_dir(file_folder) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    // Check if the file is a regular file before processing
                    if file_path.is_file() {
                        // Call the provided function on the file
                        Data::load_file(&file_path, &mut data);
                    }
                }
            }
        }
        data
    }

    pub fn get_race_options(&self) -> Choice
    {
        let mut choice = Choice::new("Race".to_string());
        for race in &self.race_list {
            let binding = race.clone().1.clone();
            let effect = |char: &mut Character|
            {
                char.race = binding;
            };
            let opt = ChoiceOption
            {
                description: race.1.decription.clone(),
                effect: Box::new(effect)  
            };
            choice.options.push(opt);
        }
        choice
    }
}