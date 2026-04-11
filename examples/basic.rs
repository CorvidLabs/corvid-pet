use corvid_pet::{Event, Mood, Pet, Species};

#[cfg(feature = "color")]
use corvid_pet::{ColorScheme, PetColor};

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

    // Demonstrate custom colors
    #[cfg(feature = "color")]
    {
        println!("=== Custom Colors ===\n");

        let custom = Pet::new("Ruby".to_string(), Species::Crow)
            .with_colors(ColorScheme::new(PetColor::Red, PetColor::Yellow));
        println!("Red body, yellow bubble:");
        println!("{}", custom.render_colored());
        println!();

        let green = Pet::new("Jade".to_string(), Species::Crow)
            .with_colors(ColorScheme::new(PetColor::Green, PetColor::BrightGreen));
        println!("Green body, bright-green bubble:");
        println!("{}", green.render_colored());
        println!();

        let random = Pet::new("Lucky".to_string(), Species::Crow).with_random_colors();
        println!("Random colors ({} body, {} bubble):", random.color_scheme().body, random.color_scheme().bubble);
        println!("{}", random.render_colored());
        println!();
    }
}
