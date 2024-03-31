mod tests {
    use std::{collections::HashMap, env::temp_dir};

    use rusty_sheet_core::*;
    #[test]
    fn test_character_creation() {
        let race = Race::new("Elf".to_string());
        let starting_class = Class::new("Warrior".to_string(), Dice::D8);
        let base_stats = Stats::new();

        let character = Character::new("Winston".to_string(), race, starting_class, base_stats);
        
        assert_eq!(character.name, "Winston");
        assert_eq!(character.race.name, "Elf");
        assert_eq!(character.classes[0].class.name, "Warrior");
        assert_eq!(character.base_stat.strength, 10);
        assert_eq!(character.base_stat.dexterity, 10);
    }

    #[test]
    fn test_save_load_json() {
        let temp_dir = temp_dir();
        println!("{}", &temp_dir.to_str().unwrap());

        let mut race_list: HashMap<Uuid, Race> = HashMap::new();
        let mut class_list: HashMap<Uuid, Class> = HashMap::new();
        let mut item_list: HashMap<Uuid, Item> = HashMap::new();
        let mut character_list: HashMap<Uuid, Character> = HashMap::new();

        let race = Race::new("Dwarf".to_string());
        let starting_class = Class::new("Rogue".to_string(), Dice::D6);
        let item = Item::new("Greataxe".to_string());
        let base_stats = Stats::new();
        let character = Character::new("Mobin".to_string(), race.clone(), starting_class.clone(), base_stats);

        race_list.insert(race.id, race);
        class_list.insert(starting_class.id, starting_class);
        item_list.insert(item.id, item);
        character_list.insert(character.id, character);

        save_file(&temp_dir, &race_list, &class_list, &item_list, &character_list);

        let mut loaded_race_list: HashMap<Uuid, Race> = HashMap::new();
        let mut loaded_class_list: HashMap<Uuid, Class> = HashMap::new();
        let mut loaded_item_list: HashMap<Uuid, Item> = HashMap::new();
        let mut loaded_character_list: HashMap<Uuid, Character> = HashMap::new();

        load_file(&temp_dir, &mut loaded_race_list, &mut loaded_class_list, &mut loaded_item_list, &mut loaded_character_list);

        assert!(compare_hashmaps(&race_list, &loaded_race_list));
        assert!(compare_hashmaps(&class_list, &loaded_class_list));
        assert!(compare_hashmaps(&item_list, &loaded_item_list));
        assert!(compare_hashmaps(&character_list, &loaded_character_list));
        
    }

    fn compare_hashmaps <T: PartialEq> (map1: &HashMap<Uuid, T>, map2: &HashMap<Uuid, T>) -> bool {
        if map1.len() != map2.len() {
            return false;
        }
    
        for (key, value) in map1 {
            if map2.get(key) != Some(value) {
                return false;
            }
        }
    
        true
    }

    #[test]
    fn test_default_character() {
        let default_character = Character::default();
        
        assert_eq!(default_character.name, "");
        assert_eq!(default_character.race.name, "");
        assert_eq!(default_character.classes[0].class.name, "");
        assert_eq!(default_character.base_stat.strength, 10);
        assert_eq!(default_character.base_stat.dexterity, 10);
    }

    #[test]
    fn action_test() {
        let mut race = Race::new("Wood Elf".to_string());
        let mut race_actions = Vec::<Action>::new();
        race_actions.push(Action::new("Fleet of Foot. Your base walking speed increases to 35 feet.".to_string(), 1, ActionType::Trait));
        race_actions.push(Action::new("Mask of the Wild. You can attempt to hide even when you are only lightly obscured by foliage, heavy rain, falling snow, mist, and other natural phenomena.".to_string(), 1, ActionType::Trait));
        race.actions = race_actions;
        
        let mut class = Class::new("Druid".to_string(), Dice::D8);
        let mut class_actions = Vec::<Action>::new();
        class_actions.push(Action::new("You know Druidic, the secret language of druids. You can speak the language and use it to leave hidden messages. You and others who know this language automatically spot such a message. Others spot the message's presence with a successful DC 15 Wisdom (Perception) check but can't decipher it without magic.".to_string(), 1, ActionType::Trait));
        class_actions.push(Action::new("Starting at 2nd level, you can use your action to magically assume the shape of a beast that you have seen before. You can use this feature twice. You regain expended uses when you finish a short or long rest.".to_string(), 2, ActionType::Action));
        class.actions = class_actions;
        
        let mut character = Character::new("Fox".to_string(), race, class, Stats::new());

        assert_eq!(character.get_possible_actions().len(), 3);
    
        character.classes[0].lvl = 2;
        assert_eq!(character.get_possible_actions().len(), 4);
    }

}
