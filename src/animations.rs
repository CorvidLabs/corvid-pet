use crate::{Mood, Species};

/// Iterator yielding animation frames for a corvid pet.
#[derive(Debug)]
pub struct Animation {
    species: Species,
    mood: Mood,
    animation_type: AnimationType,
    current_frame: usize,
    max_frames: usize,
}

#[derive(Debug, Clone, Copy)]
enum AnimationType {
    Blink,
    Hop,
}

impl Animation {
    /// Creates a new blink animation.
    pub fn blink(species: Species, mood: Mood) -> Self {
        Self {
            species,
            mood,
            animation_type: AnimationType::Blink,
            current_frame: 0,
            max_frames: 4,
        }
    }

    /// Creates a new hop animation.
    pub fn hop(species: Species, mood: Mood) -> Self {
        Self {
            species,
            mood,
            animation_type: AnimationType::Hop,
            current_frame: 0,
            max_frames: 6,
        }
    }

    /// Returns the next animation frame, or None if animation is complete.
    pub fn next_frame(&mut self) -> Option<String> {
        if self.is_finished() {
            return None;
        }

        let frame = match self.animation_type {
            AnimationType::Blink => self.blink_frame(),
            AnimationType::Hop => self.hop_frame(),
        };

        self.current_frame += 1;
        Some(frame)
    }

    /// Returns true if the animation has completed all frames.
    pub fn is_finished(&self) -> bool {
        self.current_frame >= self.max_frames
    }

    fn blink_frame(&self) -> String {
        use crate::moods;

        match self.current_frame {
            0 | 3 => moods::ascii_art(self.species, self.mood),
            1 | 2 => moods::ascii_art_closed_eyes(self.species, self.mood),
            _ => moods::ascii_art(self.species, self.mood),
        }
    }

    fn hop_frame(&self) -> String {
        use crate::moods;

        let base_art = moods::ascii_art(self.species, self.mood);
        let lines: Vec<&str> = base_art.lines().collect();

        match self.current_frame {
            0 | 5 => base_art,
            1 | 3 => {
                // Slight hop - indent by 2
                let shifted: Vec<String> = lines
                    .iter()
                    .map(|line| format!("  {}", line))
                    .collect();
                shifted.join("\n")
            }
            2 => {
                // Peak hop - indent by 4
                let shifted: Vec<String> = lines
                    .iter()
                    .map(|line| format!("    {}", line))
                    .collect();
                shifted.join("\n")
            }
            4 => {
                // Landing - indent by 1
                let shifted: Vec<String> = lines
                    .iter()
                    .map(|line| format!(" {}", line))
                    .collect();
                shifted.join("\n")
            }
            _ => base_art,
        }
    }
}

impl Iterator for Animation {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_frame()
    }
}

/// Progress spinner with animated pet companion.
#[derive(Debug)]
pub struct Spinner {
    species: Species,
    mood: Mood,
    message: String,
    frame_index: usize,
    animation_frames: Vec<String>,
}

impl Spinner {
    /// Creates a new spinner with the given message.
    pub fn new(species: Species, mood: Mood, message: String) -> Self {
        // Pre-generate animation frames for cycling
        let animation_frames = generate_spinner_frames(species, mood);

        Self {
            species,
            mood,
            message,
            frame_index: 0,
            animation_frames,
        }
    }

    /// Advances the spinner to the next frame.
    pub fn tick(&mut self) {
        self.frame_index = (self.frame_index + 1) % self.animation_frames.len();
    }

    /// Updates the spinner message.
    pub fn set_message(&mut self, message: &str) {
        self.message = message.to_string();
    }

    /// Returns the current frame with the message.
    pub fn current_frame(&self) -> String {
        let pet_frame = &self.animation_frames[self.frame_index];
        format!("{}\n  {}", pet_frame, self.message)
    }

    /// Finishes the spinner and returns the completion string.
    pub fn finish(&mut self) -> String {
        format!("{}\n  ✓ {}", self.animation_frames[0], self.message)
    }

    /// Finishes the spinner and returns the pet render.
    pub fn finish_with_pet(&mut self) -> String {
        crate::moods::ascii_art(self.species, self.mood)
    }
}

fn generate_spinner_frames(species: Species, mood: Mood) -> Vec<String> {
    use crate::moods;

    // Create a sequence of frames for the spinner
    vec![
        moods::ascii_art(species, mood),
        moods::ascii_art_closed_eyes(species, mood),
        moods::ascii_art(species, mood),
        moods::ascii_art_closed_eyes(species, mood),
    ]
}
