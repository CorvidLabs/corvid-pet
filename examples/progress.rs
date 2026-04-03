use corvid_pet::{Pet, Species};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Progress Spinner Demo\n");

    let pet = Pet::new("Corvin".to_string(), Species::Crow);
    let mut spinner = pet.spinner("Checking specs...");

    // Simulate work with spinner animation
    for i in 0..20 {
        spinner.tick();
        print!("\x1B[2J\x1B[H"); // Clear screen (optional, for smooth animation)
        println!("{}", spinner.current_frame());
        println!("Progress: {}%", (i + 1) * 5);
        thread::sleep(Duration::from_millis(200));
    }

    // Finish
    println!("\n{}", spinner.finish());
    println!();

    // Show final pet state
    println!("{}", pet.render());
    println!("\nComment: \"{}\"", pet.comment());
}
