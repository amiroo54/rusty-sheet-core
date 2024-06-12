mod tests {
    use std::{collections::HashMap, env::temp_dir, fs};

    use rusty_sheet_core::*;
    #[test]
    fn test_character_creation() {
        let race = Race::new("Elf".to_string());
        let starting_class = Class::new("Fighter".to_string(), Dice::D8, 3);
        let base_stats = Stats::new();

        let character = Character::new("Winston".to_string(), race, starting_class, base_stats);
        
        assert_eq!(character.name, "Winston");
        assert_eq!(character.race.name, "Elf");
        assert_eq!(character.classes[0].class.name, "Fighter");
        assert_eq!(character.base_stat.strength, 10);
        assert_eq!(character.base_stat.dexterity, 10);
    }

    #[test]
    fn test_save_load_json() {
        let temp_dir = temp_dir().join("RustySheet");

        if !fs::metadata(&temp_dir).is_ok() {
            let _ = fs::create_dir(&temp_dir);
        }


        let mut data = Data::new();

        let race = Race::new("Dwarf".to_string());
        let mut starting_class = Class::new("Rogue".to_string(), Dice::D6, 3);
        let subclass = starting_class.create_subclass("Thief".to_string());
        let item = Item::new("Greataxe".to_string());
        let base_stats = Stats::new();
        let character = Character::new("Mobin".to_string(), race.clone(), starting_class.clone(), base_stats);

        data.race_list.insert(race.id, race);
        data.class_list.insert(starting_class.id, starting_class);
        data.item_list.insert(item.id, item);
        data.character_list.insert(character.id, character);

        data.save_file(&temp_dir.join("Player's Handbook.sheet"));
        data.save_characters(&temp_dir.join("characters.char"));

        let loaded_data = Data::load_files(&temp_dir);

        assert!(compare_hashmaps(&data.race_list, &loaded_data.race_list));
        assert!(compare_hashmaps(&data.class_list, &loaded_data.class_list));
        assert!(compare_hashmaps(&data.item_list, &loaded_data.item_list));
        assert!(compare_hashmaps(&data.character_list, &loaded_data.character_list));
        
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
        
        let mut class = Class::new("Druid".to_string(), Dice::D8, 2);
        let mut class_actions = Vec::<Action>::new();
        class_actions.push(Action::new("You know Druidic, the secret language of druids. You can speak the language and use it to leave hidden messages. You and others who know this language automatically spot such a message. Others spot the message's presence with a successful DC 15 Wisdom (Perception) check but can't decipher it without magic.".to_string(), 1, ActionType::Trait));
        class_actions.push(Action::new("Starting at 2nd level, you can use your action to magically assume the shape of a beast that you have seen before. You can use this feature twice. You regain expended uses when you finish a short or long rest.".to_string(), 2, ActionType::Action));
        class.actions = class_actions;
        
        let mut character = Character::new("Fox".to_string(), race, class, Stats::new());
        character.classes[0].lvl_up();
        assert_eq!(character.get_possible_actions().len(), 3);
    
        character.classes[0].lvl = 2;
        assert_eq!(character.get_possible_actions().len(), 4);
    }


    #[test]
    fn lvl_up_test()
    {
        let race = Race::new("Dragonborn".to_string());
        let mut starting_class = Class::new("Paladin".to_string(), Dice::D8, 3);
        starting_class.create_subclass("Devotion".to_string());
        starting_class.create_subclass("Oathbreaker".to_string());
        
        let base_stats = Stats::new();

        let mut character = Character::new("Winston".to_string(), race, starting_class, base_stats);
        

        let first_choices = character.classes[0].lvl_up();
        assert_eq!(character.classes[0].lvl, 1);
        let second_choices = character.classes[0].lvl_up();
        assert_eq!(character.classes[0].lvl, 2);        
        let third_choices = character.classes[0].lvl_up();
        assert_eq!(character.classes[0].lvl, 3);

        assert_eq!(first_choices.len(), 0);
        assert_eq!(second_choices.len(), 0);
        assert_eq!(third_choices.len(), 1);
        assert_eq!(third_choices[0].options.len(), 2)
        
    }

    #[test]
    fn choice_test()
    {
        let dragonborn = Race::new("Dragonborn".to_string());
        let warforged = Race::new("Warforged".to_string());
        let mut clreic = Class::new("cleric".to_string(), Dice::D8, 1);
        let forge = clreic.create_subclass("Forge domain".to_string());
        let life = clreic.create_subclass("Life domain".to_string());
    
        let mut data = Data::new();
        data.race_list.insert(dragonborn.id, dragonborn);
        data.race_list.insert(warforged.id, warforged);
        data.class_list.insert(clreic.id, clreic);

        let options = data.get_race_options();
        assert_eq!(options.options.len(), 2);

        
        
    }
}
