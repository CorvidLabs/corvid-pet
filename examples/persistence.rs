use corvid_pet::{Mood, Personality, Pet, PetState, Species};

fn main() {
    println!("corvid-pet — Persistence Demo\n");

    // Create a pet with simulation enabled
    let now = 1_700_000_000u64;
    let mut pet =
        Pet::new("Corvin".to_string(), Species::Crow).with_simulation(Personality::Curious, now);

    // Interact with the pet
    pet.tick(now + 300); // Hatch from egg
    pet.feed(now + 301).unwrap();
    pet.play(now + 400).unwrap();
    pet.set_mood(Mood::Happy);

    println!("Original pet:");
    println!("  Name: {}", pet.name());
    println!("  Species: {}", pet.species());
    println!("  Mood: {}", pet.mood());
    println!("  Stage: {:?}", pet.life_stage());
    if let Some(stats) = pet.stats() {
        println!("  Hunger: {:.0}%", stats.hunger);
        println!("  Energy: {:.0}%", stats.energy);
        println!("  Happiness: {:.0}%", stats.happiness);
    }
    println!("{}\n", pet.render());

    // Serialize to PetState
    let state = PetState::from_pet(&pet);

    // Convert to JSON (requires persistence feature)
    #[cfg(feature = "persistence")]
    {
        let json = serde_json::to_string_pretty(&state).unwrap();
        println!("Serialized state:\n{}\n", json);

        // Deserialize back
        let loaded: PetState = serde_json::from_str(&json).unwrap();
        let restored = loaded.to_pet();

        println!("Restored pet:");
        println!("  Name: {}", restored.name());
        println!("  Species: {}", restored.species());
        println!("  Mood: {}", restored.mood());
        println!("  Stage: {:?}", restored.life_stage());
        println!("{}", restored.render());
    }

    // Save/load to disk (requires persistence feature)
    #[cfg(feature = "persistence")]
    {
        use corvid_pet::persistence;

        let pet_id = "demo-corvin";
        println!("\nSaving to disk as '{pet_id}'...");
        persistence::save_pet(&state, pet_id).unwrap();
        println!("Saved!");

        let loaded = persistence::load_pet(pet_id).unwrap();
        let restored = loaded.to_pet();
        println!(
            "Loaded from disk: {} the {} ({})",
            restored.name(),
            restored.species(),
            restored.mood()
        );

        // List all saved pets
        let pets = persistence::list_pets().unwrap();
        println!("Saved pets: {:?}", pets);

        // Clean up
        persistence::delete_pet(pet_id).unwrap();
        println!("Cleaned up demo save.");
    }

    #[cfg(not(feature = "persistence"))]
    {
        println!("(Run with --features persistence for JSON serialization and disk save/load)");
        println!("\nWithout the feature, PetState round-trip still works:");
        let restored = state.to_pet();
        println!(
            "  Restored: {} the {} ({})",
            restored.name(),
            restored.species(),
            restored.mood()
        );
        println!("  Stage: {:?}", restored.life_stage());
    }
}
