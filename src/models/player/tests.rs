#[cfg(test)]
mod tests {
    use crate::models::player::{Player, TestInputReader, MockDiceRoller};
    use crate::models::character::{Character, Class};
    use std::collections::HashSet;

    fn create_test_player() -> Player {
        let mut characters = vec![
            Character::new(Class::Fighter),
            Character::new(Class::Wizard),
            Character::new(Class::Rogue),
        ];
        
        // Establecer nombres para los personajes
        let mut existing_names = HashSet::new();
        characters[0].set_name("Aragorn".to_string(), &existing_names);
        existing_names.insert("Aragorn".to_string());
        characters[1].set_name("Gandalf".to_string(), &existing_names);
        existing_names.insert("Gandalf".to_string());
        characters[2].set_name("Legolas".to_string(), &existing_names);
        
        Player::new(characters)
    }

    #[test]
    fn test_handle_level_up_basic() {
        let mut player = create_test_player();
        player.encounters_won = 10;
        let mut input_reader = TestInputReader::new("Aragorn
".to_string());
        let mut dice = MockDiceRoller { value: 6 };
        player.handle_level_up(&mut input_reader, &mut dice);
        let aragorn = player.characters.iter().find(|c| c.name == "Aragorn").unwrap();
        assert_eq!(aragorn.level, 2);
        assert_eq!(player.leveled_up_last_time, Some("Aragorn".to_string()));
    }

    #[test]
    fn test_handle_level_up_with_max_level_others() {
        let mut player = create_test_player();
        player.encounters_won = 10;
        for character in &mut player.characters {
            if character.name != "Aragorn" {
                character.level = 5;
            }
        }
        player.leveled_up_last_time = Some("Aragorn".to_string());
        let mut input_reader = TestInputReader::new("Aragorn
".to_string());
        let mut dice = MockDiceRoller { value: 6 };
        player.handle_level_up(&mut input_reader, &mut dice);
        let aragorn = player.characters.iter().find(|c| c.name == "Aragorn").unwrap();
        assert_eq!(aragorn.level, 2);
    }

    #[test]
    fn test_handle_level_up_without_max_level_others() {
        let mut player = create_test_player();
        player.encounters_won = 10;
        player.leveled_up_last_time = Some("Aragorn".to_string());
        let mut input_reader = TestInputReader::new("Gandalf
".to_string());
        let mut dice = MockDiceRoller { value: 6 };
        player.handle_level_up(&mut input_reader, &mut dice);
        let gandalf = player.characters.iter().find(|c| c.name == "Gandalf").unwrap();
        assert_eq!(gandalf.level, 2);
        let aragorn = player.characters.iter().find(|c| c.name == "Aragorn").unwrap();
        assert_eq!(aragorn.level, 1);
    }

    #[test]
    #[should_panic(expected = "Ya existe un personaje con el nombre: Gandalf")]
    fn test_duplicate_character_names() {
        let mut characters = vec![
            Character::new(Class::Fighter),
            Character::new(Class::Wizard),
            Character::new(Class::Rogue),
        ];
        
        // Establecer nombres duplicados
        let mut existing_names = HashSet::new();
        characters[0].set_name("Gandalf".to_string(), &existing_names);
        existing_names.insert("Gandalf".to_string());
        characters[1].set_name("Gandalf".to_string(), &existing_names); // Duplicado
        existing_names.insert("Gandalf".to_string()); // Esto no fallará, pero el Player::new debería
        characters[2].set_name("Legolas".to_string(), &existing_names);
        
        Player::new(characters); // Aquí es donde debe ocurrir el panic
    }
} 