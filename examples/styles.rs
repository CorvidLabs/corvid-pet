use corvid_pet::{ArtStyle, Mood, Pet, Species};

fn main() {
    println!("Corvid Pet Styles Demo\n");

    let pet = Pet::new("Corvin".to_string(), Species::Crow);

    // Show all styles
    let styles = [
        ArtStyle::Detailed,
        ArtStyle::Minimal,
        ArtStyle::Blocky,
        ArtStyle::Emoji,
    ];

    for style in &styles {
        println!("=== {} Style ===", style);
        println!("{}", pet.render_with_style(*style));
        println!();
    }

    // Show moods with minimal style
    println!("=== Crow Moods (Minimal Style) ===\n");
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
        println!("{:?}:", mood);
        println!("{}", pet.render_with_style(ArtStyle::Minimal));
        println!();
    }

    // Show all species with blocky style
    println!("=== All Species (Blocky Style) ===\n");
    for species in [Species::Crow, Species::Raven, Species::Magpie, Species::Jay] {
        let pet = Pet::new(String::new(), species);
        println!("{}:", species);
        println!("{}", pet.render_with_style(ArtStyle::Blocky));
        println!();
    }

    // Parse style from string
    println!("=== Parsing Style from String ===");
    let style_str = "minimal";
    match style_str.parse::<ArtStyle>() {
        Ok(style) => println!("Parsed '{}' as {:?}", style_str, style),
        Err(e) => println!("Error parsing '{}': {}", style_str, e),
    }
}
