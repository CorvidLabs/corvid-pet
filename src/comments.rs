use rand::{seq::SliceRandom, thread_rng};

use crate::{Mood, Species};

/// Returns a random comment appropriate for the species and mood.
pub fn random_comment(species: Species, mood: Mood) -> String {
    let comments = match species {
        Species::Crow => crow_comments(mood),
        Species::Raven => raven_comments(mood),
        Species::Magpie => magpie_comments(mood),
        Species::Jay => jay_comments(mood),
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

fn raven_comments(mood: Mood) -> &'static [&'static str] {
    match mood {
        Mood::Happy => &[
            "Quoth the Raven: well done.",
            "Nevermore shall your specs be out of date.",
            "Thy code is dark and full of wisdom.",
            "A spec most fair, I do declare.",
        ],
        Mood::Sad => &[
            "Quoth: fix thy imports, lest darkness fall.",
            "Nevermore shall this validation pass...",
            "Thy errors multiply like shadows at dusk.",
            "A grim sight, this broken build.",
        ],
        Mood::Neutral => &[
            "Quoth?",
            "I watch from the shadow of your branch.",
            "What say you, code?",
            "Perched upon your repository, observing.",
        ],
        Mood::Confused => &[
            "Quoth the... what is this strange symbol?",
            "I am but a raven, yet this confuses even me.",
            "Dark magic, this undocumented API.",
            "Quoth??",
        ],
        Mood::Excited => &[
            "Quoth: EXCELLENT NEWS INDEED!",
            "Nevermore shall we lack this feature!",
            "The docs are generated! Rejoice!",
            "What a shiny spec we have wrought!",
        ],
        Mood::Sleepy => &[
            "Zzz... quoth... zzz...",
            "Dreaming of midnight dreary...",
            "Nevermore... shall I stay awake...",
            "Zzz... the code compiles... in my dreams...",
        ],
    }
}

fn magpie_comments(mood: Mood) -> &'static [&'static str] {
    match mood {
        Mood::Happy => &[
            "Ooh! Look at this shiny spec!",
            "This code is as shiny as silver!",
            "I must add this to my collection!",
            "Twit! Twit! So many shiny exports!",
        ],
        Mood::Sad => &[
            "No shiny things found today...",
            "I searched but found only dull code...",
            "Twit... no sparkles here...",
            "Even this error message isn't shiny.",
        ],
        Mood::Neutral => &[
            "Is this shiny? Let me check...",
            "Looking for treasures in your code.",
            "Twit?",
            "Every file might hide something shiny!",
        ],
        Mood::Confused => &[
            "Is THIS shiny? I can't tell...",
            "This code is confusing, not shiny!",
            "Twit?? What does this export do?",
            "I don't think this belongs in my nest...",
        ],
        Mood::Excited => &[
            "SO MANY SHINY NEW SPECS!",
            "IT'S BEAUTIFUL! IT'S PERFECT!",
            "I MUST SHOW THIS TO EVERYONE!",
            "Twit twit! Twit twit! SHINY!",
        ],
        Mood::Sleepy => &[
            "Zzz... shiny... zzz...",
            "Dreaming of silver spoons and specs...",
            "Twit... so sleepy...",
            "Zzz... my nest of code is warm...",
        ],
    }
}

fn jay_comments(mood: Mood) -> &'static [&'static str] {
    match mood {
        Mood::Happy => &[
            "HEY! YOUR SPECS LOOK AMAZING!",
            "CA-CAW! THIS IS THE BEST CODE I'VE SEEN!",
            "EVERYONE NEEDS TO SEE THIS SPEC!",
            "I'M SO HAPPY I COULD SCREAM! AGAIN!",
        ],
        Mood::Sad => &[
            "HEY... SOMETHING'S WRONG...",
            "CAW... YOUR VALIDATION FAILED...",
            "I'M SAD SO I'M MAKING IT EVERYONE'S PROBLEM!",
            "EVEN MY LOUD CAWS CAN'T FIX THIS...",
        ],
        Mood::Neutral => &[
            "HEY! LISTEN!",
            "I'M WATCHING YOUR CODE!",
            "DID YOU KNOW I'M VERY LOUD?",
            "CAW! JUST CHECKING IN!",
        ],
        Mood::Confused => &[
            "HEY! WHAT IS THIS THING?!",
            "I DON'T UNDERSTAND BUT I'M YELLING ANYWAY!",
            "CAW?? WHAT DOES THIS EXPORT DO??",
            "THIS IS CONFUSING AND I'M UPSET ABOUT IT!",
        ],
        Mood::Excited => &[
            "HEY! HEY! NEW SPEC GENERATED!",
            "CA-CAW! THIS IS SO EXCITING!",
            "I'M GOING TO TELL EVERYONE!",
            "THIS IS THE BEST DAY OF MY LIFE!",
        ],
        Mood::Sleepy => &[
            "HEY... ZZZ... I'M TIRED...",
            "CAW... QUIET PLEASE...",
            "I'M RESTING MY VOCAL CORDS...",
            "Zzz... is anyone still listening...?",
        ],
    }
}
