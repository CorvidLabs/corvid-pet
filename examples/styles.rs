use corvid_pet::{ArtStyle, Mood, Pet, Species};

fn render(pet: &Pet, style: ArtStyle) -> String {
    #[cfg(feature = "color")]
    {
        pet.render_colored_with_style(style)
    }
    #[cfg(not(feature = "color"))]
    {
        pet.render_with_style(style)
    }
}

fn main() {
    println!("corvid-pet — Minimal Style Showcase\n");

    let pet = Pet::new("Corvin".to_string(), Species::Crow);
    println!("--- Crow (Minimal) ---");
    println!("{}", render(&pet, ArtStyle::Minimal));
    println!();

    // Show mood variations
    println!("========================================");
    println!("  MOOD SHOWCASE (Crow)");
    println!("========================================\n");

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
        println!("{}", render(&pet, ArtStyle::Minimal));
        println!();
    }
}
