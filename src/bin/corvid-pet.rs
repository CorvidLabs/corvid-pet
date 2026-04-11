use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use clap::{Parser, Subcommand, ValueEnum};
use corvid_pet::{Event, Mood, Pet, Species};
use corvid_pet::health::{self, RepoHealth};

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

    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,
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

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn load_or_create(path: &PathBuf, name: &str) -> RepoHealth {
    if path.exists() {
        health::load_health(path).unwrap_or_else(|e| {
            eprintln!("Warning: could not load {}: {}", path.display(), e);
            RepoHealth::new(name.to_string())
        })
    } else {
        RepoHealth::new(name.to_string())
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.no_color {
        // Disable colored output globally.
        colored::control::set_override(false);
    }

    match cli.command {
        None | Some(Commands::Show { mood: None }) => {
            // Default: show the bird.
            let pet = Pet::new(cli.name, Species::Crow);
            if cli.no_color {
                println!("{}", pet.render());
            } else {
                println!("{}", pet.render_colored());
            }
            println!("\n  {}", pet.comment());
        }

        Some(Commands::Show { mood: Some(mood) }) => {
            let mut pet = Pet::new(cli.name, Species::Crow);
            pet.set_mood(mood.into());
            if cli.no_color {
                println!("{}", pet.render());
            } else {
                println!("{}", pet.render_colored());
            }
            println!("\n  {}", pet.comment());
        }

        Some(Commands::React {
            event,
            state,
            context,
        }) => {
            let event: Event = event.into();
            let mut health = load_or_create(&state, &cli.name);
            health.record(event, now_secs(), context);
            health::save_health(&health, &state).unwrap_or_else(|e| {
                eprintln!("Error saving state: {}", e);
                std::process::exit(1);
            });

            // Show the bird reacting.
            let mut pet = Pet::new(cli.name, Species::Crow);
            pet.react(event);
            if cli.no_color {
                println!("{}", pet.render());
            } else {
                println!("{}", pet.render_colored());
            }
            println!("\n  {}", pet.comment());
            println!("\n{}", health.summary());
        }

        Some(Commands::Health { state, json }) => {
            let health = load_or_create(&state, &cli.name);
            if json {
                let j = serde_json::to_string_pretty(&health).expect("serialize");
                println!("{}", j);
            } else {
                // Show bird with health-derived mood.
                let mut pet = Pet::new(cli.name, Species::Crow);
                pet.set_mood(health.mood());
                if cli.no_color {
                    println!("{}", pet.render());
                } else {
                    println!("{}", pet.render_colored());
                }
                println!("\n  {}", pet.comment());
                println!("\n{}", health.summary());
            }
        }

        Some(Commands::Comment {
            event,
            state,
            context,
        }) => {
            let health = load_or_create(&state, &cli.name);
            let event: Event = event.into();
            let comment = health.pr_comment(event, &context);
            println!("{}", comment);
        }

        Some(Commands::Badge { state }) => {
            let health = load_or_create(&state, &cli.name);
            println!("{}", health.readme_badge());
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
            health::save_health(&health, &state).unwrap_or_else(|e| {
                eprintln!("Error creating state: {}", e);
                std::process::exit(1);
            });
            println!("Initialized corvid-pet state at {}", state.display());

            let pet = Pet::new(cli.name, Species::Crow);
            if cli.no_color {
                println!("{}", pet.render());
            } else {
                println!("{}", pet.render_colored());
            }
            println!("\n  Ready to watch over your repo!");
        }

        Some(Commands::Greet { who }) => {
            let mut pet = Pet::new(cli.name, Species::Crow);
            pet.set_mood(Mood::Happy);
            if cli.no_color {
                println!("{}", pet.render());
            } else {
                println!("{}", pet.render_colored());
            }
            match who {
                Some(name) => println!("\n  Caw! Welcome, {}! 🐦", name),
                None => println!("\n  Caw! Welcome! 🐦"),
            }
        }
    }
}
