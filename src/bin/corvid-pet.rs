use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use clap::{Parser, Subcommand, ValueEnum};
use corvid_pet::color::{ColorScheme, PetColor};
use corvid_pet::health::{self, RepoHealth};
use corvid_pet::persistence::{self, PetState};
use corvid_pet::{Event, Mood, Personality, Pet, Species};

/// ASCII corvid companion for your terminal and CI/CD pipelines.
///
/// A living mascot that tracks your repo's health and reacts to
/// CI events. Use standalone, in scripts, or as a GitHub Action.
#[derive(Parser)]
#[command(name = "corvid-pet", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Pet name
    #[arg(long, default_value = "Corvin", global = true)]
    name: String,

    /// Output format: text (default) or json
    #[arg(long, default_value = "text", global = true)]
    format: String,

    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,

    /// Body color (e.g. red, green, blue, magenta, cyan, bright-red, etc.)
    #[arg(long, global = true)]
    color: Option<String>,

    /// Thought bubble color (defaults to cyan)
    #[arg(long, global = true)]
    bubble_color: Option<String>,

    /// Use random colors
    #[arg(long, global = true)]
    random_colors: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Display the corvid (default when no subcommand given).
    Show {
        /// Mood to display.
        #[arg(long, value_enum)]
        mood: Option<MoodArg>,
    },

    /// React to a CI/CD event and update health state.
    React {
        /// The event type.
        #[arg(value_enum)]
        event: EventArg,

        /// Health state file path.
        #[arg(long, default_value = ".corvid-pet.json")]
        state: PathBuf,

        /// Optional context (PR number, commit SHA, etc).
        #[arg(long)]
        context: Option<String>,
    },

    /// Show the repo health summary.
    Health {
        /// Health state file path.
        #[arg(long, default_value = ".corvid-pet.json")]
        state: PathBuf,

        /// Output as JSON.
        #[arg(long)]
        json: bool,
    },

    /// Generate a PR comment (markdown).
    Comment {
        /// The event that triggered this comment.
        #[arg(value_enum)]
        event: EventArg,

        /// Health state file path.
        #[arg(long, default_value = ".corvid-pet.json")]
        state: PathBuf,

        /// Context message for the comment body.
        #[arg(long, default_value = "")]
        context: String,
    },

    /// Generate a README badge section.
    Badge {
        /// Health state file path.
        #[arg(long, default_value = ".corvid-pet.json")]
        state: PathBuf,
    },

    /// Initialize a new health state file.
    Init {
        /// Health state file path.
        #[arg(long, default_value = ".corvid-pet.json")]
        state: PathBuf,
    },

    /// Greet with a random corvid message.
    Greet {
        /// Optional name to greet.
        #[arg()]
        who: Option<String>,
    },

    /// Feed your pet.
    Feed,

    /// Play with your pet.
    Play,

    /// Show pet stats and life stage.
    Status,

    /// Run a life simulation tick.
    Sim {
        /// Personality for a new pet (curious, shy, mischievous, stoic, affectionate, greedy).
        #[arg(long, value_enum, default_value = "curious")]
        personality: PersonalityArg,
    },
}

#[derive(Clone, ValueEnum)]
enum MoodArg {
    Happy,
    Sad,
    Neutral,
    Confused,
    Excited,
    Sleepy,
}

impl From<MoodArg> for Mood {
    fn from(m: MoodArg) -> Mood {
        match m {
            MoodArg::Happy => Mood::Happy,
            MoodArg::Sad => Mood::Sad,
            MoodArg::Neutral => Mood::Neutral,
            MoodArg::Confused => Mood::Confused,
            MoodArg::Excited => Mood::Excited,
            MoodArg::Sleepy => Mood::Sleepy,
        }
    }
}

#[derive(Clone, ValueEnum)]
enum EventArg {
    Success,
    Failure,
    Warning,
    Progress,
    Idle,
}

impl From<EventArg> for Event {
    fn from(e: EventArg) -> Event {
        match e {
            EventArg::Success => Event::Success,
            EventArg::Failure => Event::Failure,
            EventArg::Warning => Event::Warning,
            EventArg::Progress => Event::Progress,
            EventArg::Idle => Event::Idle,
        }
    }
}

#[derive(Clone, ValueEnum)]
enum PersonalityArg {
    Curious,
    Shy,
    Mischievous,
    Stoic,
    Affectionate,
    Greedy,
}

impl From<PersonalityArg> for Personality {
    fn from(p: PersonalityArg) -> Personality {
        match p {
            PersonalityArg::Curious => Personality::Curious,
            PersonalityArg::Shy => Personality::Shy,
            PersonalityArg::Mischievous => Personality::Mischievous,
            PersonalityArg::Stoic => Personality::Stoic,
            PersonalityArg::Affectionate => Personality::Affectionate,
            PersonalityArg::Greedy => Personality::Greedy,
        }
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn load_or_create(path: &Path, name: &str) -> RepoHealth {
    if path.exists() {
        health::load_health(path).unwrap_or_else(|e| {
            eprintln!("Warning: could not load {}: {}", path.display(), e);
            RepoHealth::new(name.to_string())
        })
    } else {
        RepoHealth::new(name.to_string())
    }
}

/// Loads the pet from persistent storage, or creates a new one with simulation enabled.
fn load_sim_pet(cli: &Cli, personality: Personality) -> Pet {
    let mut pet = match persistence::load_pet(&cli.name) {
        Ok(state) => {
            let mut p = state.to_pet();
            // Apply CLI color overrides.
            if cli.random_colors {
                p = p.with_random_colors();
            } else if let Some(ref body) = cli.color {
                let body_color: PetColor = body.parse().unwrap_or(PetColor::Blue);
                let bubble_color: PetColor = cli
                    .bubble_color
                    .as_deref()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(PetColor::Cyan);
                p.set_color_scheme(ColorScheme::new(body_color, bubble_color));
            }
            p
        }
        Err(_) => {
            let p = make_pet(cli);
            p.with_simulation(personality, now_secs())
        }
    };

    // Tick the simulation to the current time.
    pet.tick(now_secs());
    pet
}

/// Saves the pet to persistent storage.
fn save_sim_pet(pet: &Pet, name: &str) {
    let mut state = PetState::from_pet(pet);
    state.last_saved = Some(now_secs());
    persistence::save_pet(&state, name).unwrap_or_else(|e| {
        eprintln!("Warning: could not save pet state: {}", e);
    });
}

fn make_pet(cli: &Cli) -> Pet {
    let mut pet = Pet::new(cli.name.clone(), Species::Crow);
    if cli.random_colors {
        pet = pet.with_random_colors();
    } else if let Some(ref body) = cli.color {
        let body_color: PetColor = body.parse().unwrap_or_else(|e| {
            eprintln!("Warning: {e}, using default");
            PetColor::Blue
        });
        let bubble_color: PetColor = cli
            .bubble_color
            .as_deref()
            .map(|s| {
                s.parse().unwrap_or_else(|e| {
                    eprintln!("Warning: {e}, using default");
                    PetColor::Cyan
                })
            })
            .unwrap_or(PetColor::Cyan);
        pet = pet.with_colors(ColorScheme::new(body_color, bubble_color));
    }
    pet
}

fn render_pet(pet: &Pet, no_color: bool) {
    if no_color {
        println!("{}", pet.render());
    } else {
        println!("{}", pet.render_colored());
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.no_color {
        // Disable colored output globally.
        colored::control::set_override(false);
    }

    let json_output = cli.format == "json";

    match &cli.command {
        None | Some(Commands::Show { mood: None }) => {
            let pet = make_pet(&cli);
            if json_output {
                let j = serde_json::json!({
                    "art": pet.render(),
                    "mood": pet.mood().to_string(),
                    "species": pet.species().to_string(),
                });
                println!("{}", serde_json::to_string_pretty(&j).expect("serialize"));
            } else {
                render_pet(&pet, cli.no_color);
                println!("\n  {}", pet.comment());
            }
        }

        Some(Commands::Show { mood: Some(mood) }) => {
            let mut pet = make_pet(&cli);
            pet.set_mood(mood.clone().into());
            if json_output {
                let j = serde_json::json!({
                    "art": pet.render(),
                    "mood": pet.mood().to_string(),
                    "species": pet.species().to_string(),
                });
                println!("{}", serde_json::to_string_pretty(&j).expect("serialize"));
            } else {
                render_pet(&pet, cli.no_color);
                println!("\n  {}", pet.comment());
            }
        }

        Some(Commands::React {
            event,
            state,
            context,
        }) => {
            let event: Event = event.clone().into();
            let mut health = load_or_create(state, &cli.name);
            health.record(event, now_secs(), context.clone());
            health::save_health(&health, state).unwrap_or_else(|e| {
                eprintln!("Error saving state: {}", e);
                std::process::exit(1);
            });

            // Show the bird reacting.
            let mut pet = make_pet(&cli);
            pet.react(event);
            if json_output {
                let j = serde_json::json!({
                    "event": format!("{:?}", event),
                    "mood": pet.mood().to_string(),
                    "score": health.score,
                });
                println!("{}", serde_json::to_string_pretty(&j).expect("serialize"));
            } else {
                render_pet(&pet, cli.no_color);
                println!("\n  {}", pet.comment());
                println!("\n{}", health.summary());
            }
        }

        Some(Commands::Health { state, json }) => {
            let health = load_or_create(state, &cli.name);
            if *json || json_output {
                let j = serde_json::to_string_pretty(&health).expect("serialize");
                println!("{}", j);
            } else {
                let mut pet = make_pet(&cli);
                pet.set_mood(health.mood());
                render_pet(&pet, cli.no_color);
                println!("\n  {}", pet.comment());
                println!("\n{}", health.summary());
            }
        }

        Some(Commands::Comment {
            event,
            state,
            context,
        }) => {
            let health = load_or_create(state, &cli.name);
            let event: Event = event.clone().into();
            let comment = health.pr_comment(event, context);
            if json_output {
                let j = serde_json::json!({
                    "comment": comment,
                    "mood": health.mood().to_string(),
                    "event": format!("{:?}", event),
                });
                println!("{}", serde_json::to_string_pretty(&j).expect("serialize"));
            } else {
                println!("{}", comment);
            }
        }

        Some(Commands::Badge { state }) => {
            let health = load_or_create(state, &cli.name);
            if json_output {
                let j = serde_json::json!({
                    "badge": health.readme_badge(),
                    "score": health.score,
                    "mood": health.mood().to_string(),
                });
                println!("{}", serde_json::to_string_pretty(&j).expect("serialize"));
            } else {
                println!("{}", health.readme_badge());
            }
        }

        Some(Commands::Init { state }) => {
            if state.exists() {
                eprintln!(
                    "State file already exists: {}. Use --state to specify a different path.",
                    state.display()
                );
                std::process::exit(1);
            }
            let health = RepoHealth::new(cli.name.clone());
            health::save_health(&health, state).unwrap_or_else(|e| {
                eprintln!("Error creating state: {}", e);
                std::process::exit(1);
            });
            if json_output {
                let j = serde_json::json!({
                    "state": state.display().to_string(),
                    "message": "Initialized corvid-pet state",
                });
                println!("{}", serde_json::to_string_pretty(&j).expect("serialize"));
            } else {
                println!("Initialized corvid-pet state at {}", state.display());
                let pet = make_pet(&cli);
                render_pet(&pet, cli.no_color);
                println!("\n  Ready to watch over your repo!");
            }
        }

        Some(Commands::Greet { who }) => {
            let mut pet = make_pet(&cli);
            pet.set_mood(Mood::Happy);
            if json_output {
                let j = serde_json::json!({
                    "greeting": match who {
                        Some(name) => format!("Caw! Welcome, {}!", name),
                        None => "Caw! Welcome!".to_string(),
                    },
                    "mood": "Happy",
                });
                println!("{}", serde_json::to_string_pretty(&j).expect("serialize"));
            } else {
                render_pet(&pet, cli.no_color);
                match who {
                    Some(name) => println!("\n  Caw! Welcome, {}! 🐦", name),
                    None => println!("\n  Caw! Welcome! 🐦"),
                }
            }
        }

        Some(Commands::Feed) => {
            let mut pet = load_sim_pet(&cli, Personality::Curious);
            let result = pet.feed(now_secs());
            save_sim_pet(&pet, &cli.name);

            if json_output {
                let j = serde_json::json!({
                    "action": "feed",
                    "success": result.as_ref().is_some_and(|r| r.success),
                    "message": result.as_ref().map_or("No simulation active".to_string(), |r| r.message.clone()),
                    "mood": pet.mood().to_string(),
                });
                println!("{}", serde_json::to_string_pretty(&j).expect("serialize"));
            } else {
                render_pet(&pet, cli.no_color);
                match result {
                    Some(r) => println!("\n  {}", r.message),
                    None => println!("\n  No simulation active. Run `corvid-pet sim` first."),
                }
            }
        }

        Some(Commands::Play) => {
            let mut pet = load_sim_pet(&cli, Personality::Curious);
            let result = pet.play(now_secs());
            save_sim_pet(&pet, &cli.name);

            if json_output {
                let j = serde_json::json!({
                    "action": "play",
                    "success": result.as_ref().is_some_and(|r| r.success),
                    "message": result.as_ref().map_or("No simulation active".to_string(), |r| r.message.clone()),
                    "mood": pet.mood().to_string(),
                });
                println!("{}", serde_json::to_string_pretty(&j).expect("serialize"));
            } else {
                render_pet(&pet, cli.no_color);
                match result {
                    Some(r) => println!("\n  {}", r.message),
                    None => println!("\n  No simulation active. Run `corvid-pet sim` first."),
                }
            }
        }

        Some(Commands::Status) => {
            let pet = load_sim_pet(&cli, Personality::Curious);

            if json_output {
                let j = serde_json::json!({
                    "name": pet.name(),
                    "species": pet.species().to_string(),
                    "mood": pet.mood().to_string(),
                    "life_stage": pet.life_stage().map(|s| s.to_string()),
                    "age": pet.age_display(),
                    "stats": pet.stats().map(|s| serde_json::json!({
                        "hunger": s.hunger,
                        "energy": s.energy,
                        "happiness": s.happiness,
                        "health": s.health,
                        "overall": s.overall(),
                    })),
                });
                println!("{}", serde_json::to_string_pretty(&j).expect("serialize"));
            } else {
                render_pet(&pet, cli.no_color);
                if let (Some(stats), Some(stage), Some(age)) =
                    (pet.stats(), pet.life_stage(), pet.age_display())
                {
                    println!();
                    println!("  {} — {} ({})", pet.name(), stage, age);
                    println!("  Mood: {}", pet.mood());
                    println!();
                    println!("  Hunger:    {:>5.1}%  {}", stats.hunger, bar(stats.hunger));
                    println!("  Energy:    {:>5.1}%  {}", stats.energy, bar(stats.energy));
                    println!("  Happiness: {:>5.1}%  {}", stats.happiness, bar(stats.happiness));
                    println!("  Health:    {:>5.1}%  {}", stats.health, bar(stats.health));
                    println!("  Overall:   {:>5.1}%", stats.overall());

                    let critical = stats.critical_needs();
                    if !critical.is_empty() {
                        let names: Vec<_> = critical.iter().map(|n| n.description()).collect();
                        println!("\n  Needs attention: {}", names.join(", "));
                    }
                } else {
                    println!("\n  No simulation active. Run `corvid-pet sim` to start.");
                }
            }
        }

        Some(Commands::Sim { personality }) => {
            let personality: Personality = personality.clone().into();
            let mut pet = load_sim_pet(&cli, personality);
            pet.tick(now_secs());
            save_sim_pet(&pet, &cli.name);

            if json_output {
                let j = serde_json::json!({
                    "name": pet.name(),
                    "life_stage": pet.life_stage().map(|s| s.to_string()),
                    "mood": pet.mood().to_string(),
                    "personality": pet.pet_personality().map(|p| format!("{:?}", p)),
                    "age": pet.age_display(),
                });
                println!("{}", serde_json::to_string_pretty(&j).expect("serialize"));
            } else {
                render_pet(&pet, cli.no_color);
                if let Some(stage) = pet.life_stage() {
                    println!("\n  {} is a {} {}", pet.name(), stage, pet.species());
                    if let Some(age) = pet.age_display() {
                        println!("  Age: {}", age);
                    }
                    println!("  Simulation ticked. State saved.");
                }
            }
        }
    }
}

/// Renders a simple bar chart for a stat value (0-100).
fn bar(value: f32) -> String {
    let filled = (value / 5.0).round() as usize;
    let empty = 20_usize.saturating_sub(filled);
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}
