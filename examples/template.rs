use corvid_pet::templates::{ArtTemplate, TemplateRegistry, example_crow_template};
use corvid_pet::{Mood, Pet, Species};

fn main() {
    println!("Corvid Pet Templates Demo\n");

    // Create a template registry
    let mut registry = TemplateRegistry::new();

    // Add a custom template
    let custom_template = example_crow_template();
    registry.register(custom_template);

    // Show available templates
    println!("Registered templates:");
    for name in registry.list() {
        println!("  - {}", name);
    }
    println!();

    // Create a pet and render with custom template
    let pet = Pet::new("Cyber".to_string(), Species::Crow);

    println!("=== Default Style ===");
    println!("{}", pet.render());
    println!();

    println!("=== Custom Template (Cyber Crow) ===");
    let art = registry.render(Species::Crow, Mood::Happy, Some("Cyber Crow"));
    println!("{}", art);
    println!();

    // JSON serialization (requires serde)
    #[cfg(feature = "persistence")]
    {
        let template = example_crow_template();
        match template.to_json() {
            Ok(json) => {
                println!("=== Template JSON ===");
                println!("{}", json);
                println!();

                // Load it back
                match ArtTemplate::from_json(&json) {
                    Ok(loaded) => {
                        println!("Loaded template: {}", loaded.name);
                        if let Some(art) = loaded.get_mood(Mood::Happy) {
                            println!("Happy mood art:\n{}", art);
                        }
                    }
                    Err(e) => println!("Error loading template: {}", e),
                }
            }
            Err(e) => println!("Error serializing: {}", e),
        }
    }

    // Create your own template
    println!("\n=== Creating Custom Template ===");
    let mut my_template = ArtTemplate::new("My Crow".to_string(), Species::Crow);
    my_template.set_mood(Mood::Happy, "( ◕ ‿ ◕ )".to_string());
    my_template.set_mood(Mood::Sad, "( ◕︵◕ )".to_string());

    registry.register(my_template);

    println!("My Crow - Happy:");
    println!(
        "{}",
        registry.render(Species::Crow, Mood::Happy, Some("My Crow"))
    );
    println!();

    println!("My Crow - Sad:");
    println!(
        "{}",
        registry.render(Species::Crow, Mood::Sad, Some("My Crow"))
    );
}
