//! Live interactive pet example.
//!
//! Run with: `cargo run --example live --features live`
//!
//! Controls:
//! - h/s/n/c/e/z - Change mood (Happy/Sad/Neutral/Confused/Excited/Sleepy)
//! - ? - Toggle help display
//! - b - Toggle auto-blink
//! - q/ESC - Quit

use corvid_pet::live::SimpleLivePet;
use corvid_pet::{Pet, Species};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pet = Pet::new("Corvin".to_string(), Species::Crow);

    // Use simple interactive mode (synchronous)
    let mut live = SimpleLivePet::new(pet);
    live.run_interactive()?;

    Ok(())
}
