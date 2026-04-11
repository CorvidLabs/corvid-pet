use rand::{seq::SliceRandom, thread_rng};

use crate::{Mood, Species};

/// Returns a random comment appropriate for the species and mood.
pub fn random_comment(species: Species, mood: Mood) -> String {
    let comments = match species {
        Species::Crow => crow_comments(mood),
    };

    comments
        .choose(&mut thread_rng())
        .unwrap_or(&"Caw!")
        .to_string()
}

fn crow_comments(mood: Mood) -> &'static [&'static str] {
    match mood {
        Mood::Happy => &[
            "Caw! Found a shiny new spec!",
            "That's a nice looking export you've got there.",
            "Caw! Your code sparkles like a dropped french fry.",
            "Looking sharp! Like a beak should be.",
        ],
        Mood::Sad => &[
            "Caw... validation failed...",
            "I'm pecking through the errors...",
            "Even the dumpster of code seems empty today.",
            "Caw... your imports are all over the place.",
        ],
        Mood::Neutral => &[
            "Just pecking around your codebase.",
            "Caw?",
            "Looking for bugs to snack on.",
            "This seems like a nice branch to perch on.",
        ],
        Mood::Confused => &[
            "Caw? What's this undocumented export?",
            "I'm scratching my head with my claw...",
            "This module goes in circles like I do.",
            "Caw?? Did you mean to commit this?",
        ],
        Mood::Excited => &[
            "CAW! CAW! NEW SPEC GENERATED!",
            "I've never seen such shiny code!",
            "Let me tell the whole murder about this!",
            "Caw caw caw! This is amazing!",
        ],
        Mood::Sleepy => &[
            "Zzz... caw... zzz...",
            "Just resting my beak...",
            "Dreaming of garbage bins full of specs...",
            "Zzz... what? Oh, is it done?",
        ],
    }
}
