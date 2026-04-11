use corvid_pet::{Event, Mood, Pet, Species};

fn main() {
    println!("corvid-pet — Meet the Crow!\n");

    let pet = Pet::new("Corvin".to_string(), Species::Crow);
    println!("=== {} ({}) ===", pet.name(), Species::Crow.personality());

    #[cfg(feature = "color")]
    println!("{}", pet.render_colored());
    #[cfg(not(feature = "color"))]
    println!("{}", pet.render());

    println!("  \"{}\"", pet.comment());
    println!();

    // Show all moods
    println!("=== Mood Changes ===\n");
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
    let mut pet = Pet::new("Corvin".to_string(), Species::Crow);

    let events = [
        (Event::Success, "Tests passed!"),
        (Event::Failure, "Build failed..."),
        (Event::Warning, "Warnings found"),
        (Event::Progress, "Step completed!"),
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
