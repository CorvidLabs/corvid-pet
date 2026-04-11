use corvid_pet::{Pet, Personality, Species};

fn main() {
    println!("corvid-pet — Life Simulation Demo\n");

    let now = 1_700_000_000u64;

    // Create a pet with simulation
    let mut pet = Pet::new("Pip".to_string(), Species::Magpie)
        .with_simulation(Personality::Mischievous, now);

    println!("A new egg appears...\n");
    show_status(&pet);

    // Time passes — egg hatches
    println!("--- 5 minutes pass ---\n");
    pet.tick(now + 300);
    show_status(&pet);

    // Feed the hatchling
    println!("--- Feeding Pip ---");
    if let Some(result) = pet.feed(now + 301) {
        println!("  {}\n", result.message);
    }
    show_status(&pet);

    // Play with the pet
    println!("--- Playing with Pip ---");
    if let Some(result) = pet.play(now + 400) {
        println!("  {}\n", result.message);
    }
    show_status(&pet);

    // Let it rest
    println!("--- Pip takes a nap ---");
    if let Some(result) = pet.rest(now + 500) {
        println!("  {}\n", result.message);
    }
    show_status(&pet);

    // Time passes — stats decay
    println!("--- 2 hours pass without care ---\n");
    pet.tick(now + 7700);
    show_status(&pet);

    // Pet it to cheer it up
    println!("--- Petting Pip ---");
    if let Some(result) = pet.pet_me(now + 7701) {
        println!("  {}\n", result.message);
    }

    // Show all personality types
    println!("\n=== Personality Types ===\n");
    let personalities = [
        Personality::Curious,
        Personality::Shy,
        Personality::Mischievous,
        Personality::Stoic,
        Personality::Affectionate,
        Personality::Greedy,
    ];

    for p in personalities {
        let pet = Pet::new(String::new(), Species::Crow)
            .with_simulation(p, now);
        println!("  {:?} {} — mood starts: {}", p, pet.name(), pet.mood());
    }
}

fn show_status(pet: &Pet) {
    println!("{}", pet.render());
    println!("  {} | Stage: {:?} | Mood: {}",
        pet.name(),
        pet.life_stage().unwrap(),
        pet.mood(),
    );
    if let Some(stats) = pet.stats() {
        println!("  Hunger: {:.0}  Energy: {:.0}  Happy: {:.0}  Health: {:.0}",
            stats.hunger, stats.energy, stats.happiness, stats.health);
    }
    if let Some(age) = pet.age_display() {
        println!("  Age: {age}");
    }
    println!();
}
