use corvid_pet::{Mood, Pet, Species};

fn main() {
    println!("Corvid Pet - Minimal Style Demo\n");

    // Show the crow with all moods
    println!("=== Crow Moods ===\n");
    let mut pet = Pet::new("Corvin".to_string(), Species::Crow);

    for mood in [
        Mood::Happy,
        Mood::Sad,
        Mood::Neutral,
        Mood::Confused,
        Mood::Excited,
        Mood::Sleepy,
    ] {
        pet.set_mood(mood);
        println!("{}:", mood);
        println!("{}\n", pet.render());
    }

    // Show all species render as crow
    println!("=== All Species (All Render as Crow) ===\n");
    for species in [Species::Crow, Species::Raven, Species::Magpie, Species::Jay] {
        let pet = Pet::new(species.default_name(), species);
        println!("{} (renders as crow):", species);
        println!("{}\n", pet.render());
    }
}
