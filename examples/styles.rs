use corvid_pet::{ArtStyle, Mood, Pet, Species};

fn main() {
    println!("corvid-pet — Art Style Comparison\n");

    let species_list = [Species::Crow, Species::Raven, Species::Magpie, Species::Jay];
    let styles = [ArtStyle::Minimal, ArtStyle::Detailed];

    for style in styles {
        println!("========================================");
        println!("  Style: {} ", style.name().to_uppercase());
        println!("========================================\n");

        for species in species_list {
            let pet = Pet::new(species.default_name(), species);
            println!("--- {} ---", species);
            println!("{}", pet.render_with_style(style));
            println!();
        }
    }

    // Show mood variations in Detailed style
    println!("========================================");
    println!("  DETAILED MOOD SHOWCASE (Magpie)");
    println!("========================================\n");

    let mut pet = Pet::new("Shiny".to_string(), Species::Magpie);
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
        println!("{}", pet.render_with_style(ArtStyle::Detailed));
        println!();
    }
}
