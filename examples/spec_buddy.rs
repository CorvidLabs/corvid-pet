use corvid_pet::Species;
use corvid_pet::integrations::specsync::{SpecSyncCompanion, ValidationOutcome};

fn main() {
    println!("Spec Buddy - spec-sync integration demo\n");

    let mut companion = SpecSyncCompanion::new(Species::Crow);

    println!("Starting validation...\n");

    // Simulate spec validation with outcomes
    let outcomes = vec![
        ValidationOutcome::Success,
        ValidationOutcome::Success,
        ValidationOutcome::Warning,
        ValidationOutcome::Success,
        ValidationOutcome::Failure,
        ValidationOutcome::Generated,
        ValidationOutcome::Idle,
    ];

    for (i, outcome) in outcomes.iter().enumerate() {
        println!("Step {}: {:?}", i + 1, outcome);

        let old_mood = companion.pet().mood();
        companion.react_to_validation(*outcome);
        let new_mood = companion.pet().mood();

        println!("  Mood: {:?} -> {:?}", old_mood, new_mood);

        #[cfg(feature = "color")]
        println!("  {}", companion.pet().render_colored());
        #[cfg(not(feature = "color"))]
        println!("  {}", companion.pet().render());

        println!("  \"{}\"", companion.pet().comment());
        println!("  {}\n", companion.summary());
    }

    // Alternative: Using react_to_results
    println!("=== Using react_to_results ===\n");
    let mut companion2 = SpecSyncCompanion::new(Species::Raven);

    companion2.react_to_results(0, 0); // Success
    println!(
        "Results(0 errors, 0 warnings): {} - {}",
        companion2.pet().mood(),
        companion2.summary()
    );

    companion2.react_to_results(0, 2); // Warning
    println!(
        "Results(0 errors, 2 warnings): {} - {}",
        companion2.pet().mood(),
        companion2.summary()
    );

    companion2.react_to_results(1, 0); // Failure
    println!(
        "Results(1 error, 0 warnings): {} - {}",
        companion2.pet().mood(),
        companion2.summary()
    );

    // Summary
    println!("\n=== Final State ===");
    println!("Name: {}", companion.pet().name());
    println!("Species: {}", companion.pet().species());
    println!("Mood: {}", companion.pet().mood());
    println!("Validations processed: {}", companion.validation_count());
}
