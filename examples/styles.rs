use corvid_pet::{ArtStyle, Mood, Pet, Species};

#[cfg(feature = "color")]
use corvid_pet::{ColorScheme, PetColor};

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
    println!("corvid-pet — Style & Color Showcase\n");

    let pet = Pet::new("Corvin".to_string(), Species::Crow);
    println!("--- Default Colors ---");
    println!("{}", render(&pet, ArtStyle::Minimal));
    println!();

    // Show mood variations
    println!("========================================");
    println!("  MOOD SHOWCASE");
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

    // Show color variations
    #[cfg(feature = "color")]
    {
        println!("========================================");
        println!("  COLOR SHOWCASE");
        println!("========================================\n");

        let colors: &[(&str, PetColor, PetColor)] = &[
            ("Default (Blue/Cyan)", PetColor::Blue, PetColor::Cyan),
            ("Fire (Red/Yellow)", PetColor::Red, PetColor::Yellow),
            ("Forest (Green/BrightGreen)", PetColor::Green, PetColor::BrightGreen),
            ("Royal (Magenta/BrightMagenta)", PetColor::Magenta, PetColor::BrightMagenta),
            ("Storm (BrightBlack/White)", PetColor::BrightBlack, PetColor::White),
            ("Neon (BrightCyan/BrightYellow)", PetColor::BrightCyan, PetColor::BrightYellow),
        ];

        for (label, body, bubble) in colors {
            let pet = Pet::new("Corvin".to_string(), Species::Crow)
                .with_colors(ColorScheme::new(*body, *bubble));
            println!("{}:", label);
            println!("{}", pet.render_colored());
            println!();
        }

        println!("--- Random Colors ---");
        let random = Pet::new("Lucky".to_string(), Species::Crow).with_random_colors();
        println!(
            "({} body, {} bubble):",
            random.color_scheme().body,
            random.color_scheme().bubble
        );
        println!("{}", random.render_colored());
        println!();
    }
}
