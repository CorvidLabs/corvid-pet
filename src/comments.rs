use rand::{seq::SliceRandom, thread_rng};

use crate::{Mood, Species};

/// Returns a random comment appropriate for the species and mood.
pub fn random_comment(species: Species, mood: Mood) -> String {
    let comments = match species {
        Species::Crow => crow_comments(mood),
        Species::Magpie => magpie_comments(mood),
        Species::Raven => raven_comments(mood),
        Species::Jackdaw => jackdaw_comments(mood),
    };

    comments
        .choose(&mut thread_rng())
        .unwrap_or(&"Caw!")
        .to_string()
}

fn magpie_comments(mood: Mood) -> &'static [&'static str] {
    match mood {
        Mood::Happy => &[
            "Ooh, shiny commit!",
            "I'm adding this to my collection!",
            "Chak-chak! What a lovely green build!",
            "This code sparkles! Mine!",
        ],
        Mood::Sad => &[
            "Chak... someone took my shiny build...",
            "Nothing shiny about these errors...",
            "Even my collection can't cheer me up now.",
            "Chak... the CI took my favorite branch.",
        ],
        Mood::Neutral => &[
            "Just sorting through my collection.",
            "Chak?",
            "Hmm, is that a shiny new export?",
            "Cataloguing the codebase, one file at a time.",
        ],
        Mood::Confused => &[
            "Chak?? That's not where I left it!",
            "This doesn't match anything in my collection...",
            "Wait, which branch was I on?",
            "Chak?? Something's different here...",
        ],
        Mood::Excited => &[
            "CHAK CHAK! NEW SHINY CODE!",
            "I must tell the others about this treasure!",
            "Best. Commit. Ever! Into the collection!",
            "Chak-chak-chak! This is magnificent!",
        ],
        Mood::Sleepy => &[
            "Zzz... guarding my stash... zzz...",
            "Just resting on my collection...",
            "Dreaming of shiny new releases...",
            "Zzz... chak... zzz...",
        ],
    }
}

fn raven_comments(mood: Mood) -> &'static [&'static str] {
    match mood {
        Mood::Happy => &[
            "Quoth the Raven: 'Tests pass, evermore.'",
            "The code speaks of wisdom.",
            "A satisfying resolution. As I foresaw.",
            "Kronk. The build is sound.",
        ],
        Mood::Sad => &[
            "Quoth the Raven: 'Nevermore shall this build pass.'",
            "Darkness falls upon the pipeline...",
            "I've seen this failure before. In my dreams.",
            "Kronk... the prophecy was correct.",
        ],
        Mood::Neutral => &[
            "Observing from my perch.",
            "Kronk.",
            "The code reveals its secrets slowly.",
            "I have watched many builds come and go.",
        ],
        Mood::Confused => &[
            "Even my ancient wisdom cannot parse this...",
            "Kronk? This defies prophecy.",
            "The runes of this code are... unclear.",
            "I must consult the elder branches.",
        ],
        Mood::Excited => &[
            "KRONK! A code of legendary quality!",
            "The ancient scrolls spoke of such a build!",
            "In all my years, I've never seen such craft!",
            "KRONK KRONK! The prophecy is fulfilled!",
        ],
        Mood::Sleepy => &[
            "Zzz... dreaming of ancient repositories...",
            "Resting... gathering wisdom...",
            "The night watch can wait...",
            "Zzz... kronk... zzz...",
        ],
    }
}

fn jackdaw_comments(mood: Mood) -> &'static [&'static str] {
    match mood {
        Mood::Happy => &[
            "Kyow! The whole flock is celebrating!",
            "Let's all perch on this green build!",
            "Kyow! Group high-five! ...high-wing?",
            "Tell the colony — this one's a keeper!",
        ],
        Mood::Sad => &[
            "Kyow... the colony is quiet today...",
            "Even my friends can't fix this build...",
            "The flock sends their condolences.",
            "Kyow... gather round, team...",
        ],
        Mood::Neutral => &[
            "Just hanging with the colony.",
            "Kyow?",
            "Checking in on behalf of the flock.",
            "Nothing to report to the colony... yet.",
        ],
        Mood::Confused => &[
            "Kyow?? Let me ask the others...",
            "The colony is debating this one...",
            "Nobody in the flock has seen this before!",
            "Kyow?? Hold a flock meeting!",
        ],
        Mood::Excited => &[
            "KYOW KYOW! FLOCK PARTY!",
            "Everyone come see this! KYOW!",
            "The whole colony is buzzing!",
            "Kyow-kyow-kyow! Rally the flock!",
        ],
        Mood::Sleepy => &[
            "Zzz... roosting with the flock... zzz...",
            "The colony rests as one...",
            "Dreaming of group flights...",
            "Zzz... kyow... zzz...",
        ],
    }
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
