use corvid_pet::{Mood, Pet, Species};

fn main() {
    println!("Welcome to corvid-pet!\n");

    // Create pets of different species
    let crow = Pet::new("Corvin".to_string(), Species::Crow);
    let raven = Pet::new("Nevermore".to_string(), Species::Raven);
    let magpie = Pet::new("Shiny".to_string(), Species::Magpie);
    let jay = Pet::new("Jay".to_string(), Species::Jay);

    // Display each pet
    println!("=== Crow ===");
    #[cfg(feature = "color")]
    println!("{}", crow.render_colored());
    #[cfg(not(feature = "color"))]
    println!("{}", crow.render());
    println!();

    println!("=== Raven ===");
    #[cfg(feature = "color")]
    println!("{}", raven.render_colored());
    #[cfg(not(feature = "color"))]
    println!("{}", raven.render());
    println!();

    println!("=== Magpie ===");
    #[cfg(feature = "color")]
    println!("{}", magpie.render_colored());
    #[cfg(not(feature = "color"))]
    println!("{}", magpie.render());
    println!();

    println!("=== Jay ===");
    #[cfg(feature = "color")]
    println!("{}", jay.render_colored());
    #[cfg(not(feature = "color"))]
    println!("{}", jay.render());
    println!();

    // Show mood variations for crow
    println!("=== Crow Mood Changes ===\n");
    let mut pet = Pet::new("Corvin".to_string(), Species::Crow);

    println!("Happy:");
    pet.set_mood(Mood::Happy);
    #[cfg(feature = "color")]
    println!("{}", pet.render_colored());
    #[cfg(not(feature = "color"))]
    println!("{}", pet.render());
    println!("Comment: \"{}\"\n", pet.comment());

    println!("Sad:");
    pet.set_mood(Mood::Sad);
    #[cfg(feature = "color")]
    println!("{}", pet.render_colored());
    #[cfg(not(feature = "color"))]
    println!("{}", pet.render());
    println!("Comment: \"{}\"\n", pet.comment());

    println!("Confused:");
    pet.set_mood(Mood::Confused);
    #[cfg(feature = "color")]
    println!("{}", pet.render_colored());
    #[cfg(not(feature = "color"))]
    println!("{}", pet.render());
    println!("Comment: \"{}\"\n", pet.comment());

    println!("Excited:");
    pet.set_mood(Mood::Excited);
    #[cfg(feature = "color")]
    println!("{}", pet.render_colored());
    #[cfg(not(feature = "color"))]
    println!("{}", pet.render());
    println!("Comment: \"{}\"\n", pet.comment());
}
