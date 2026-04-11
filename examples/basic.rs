use corvid_pet::{Event, Mood, Pet, Species};

fn main() {
    println!("corvid-pet — Meet the flock!\n");

    // Show all four species side by side
    for species in [Species::Crow, Species::Raven, Species::Magpie, Species::Jay] {
        let pet = Pet::new(species.default_name(), species);
        println!("=== {} ({}) ===", pet.name(), species.personality());

        #[cfg(feature = "color")]
        println!("{}", pet.render_colored());
        #[cfg(not(feature = "color"))]
        println!("{}", pet.render());

        println!("  \"{}\"", pet.comment());
        println!();
    }

    // Show one species through all moods
    println!("=== Mood Changes (Crow) ===\n");
    let mut crow = Pet::new("Corvin".to_string(), Species::Crow);

    for mood in [
        Mood::Happy,
        Mood::Sad,
        Mood::Neutral,
        Mood::Confused,
        Mood::Excited,
        Mood::Sleepy,
    ] {
        crow.set_mood(mood);
        println!("{}:", mood);

        #[cfg(feature = "color")]
        println!("{}", crow.render_colored());
        #[cfg(not(feature = "color"))]
        println!("{}", crow.render());

        println!("  \"{}\"", crow.comment());
        println!();
    }

    // Demonstrate event reactions
    println!("=== Event Reactions ===\n");
    let mut pet = Pet::new("Nevermore".to_string(), Species::Raven);

    let events = [
        (Event::SpecPassed, "Spec passed!"),
        (Event::SpecFailed, "Spec failed..."),
        (Event::ValidationWarning, "Warnings found"),
        (Event::NewSpecGenerated, "New spec generated!"),
        (Event::Idle, "System idle"),
    ];

    for (event, label) in events {
        pet.react(event);
        println!("{label} → mood: {}", pet.mood());

        #[cfg(feature = "color")]
        println!("{}", pet.render_colored());
        #[cfg(not(feature = "color"))]
        println!("{}", pet.render());

        println!("  \"{}\"", pet.comment());
        println!();
    }
}
