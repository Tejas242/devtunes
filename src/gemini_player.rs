use google_generative_ai_rs::v1::api::{Client, PostResult};
use google_generative_ai_rs::v1::gemini::{
    request::{GenerationConfig, Request, SafetySettings, Tools},
    Model, ResponseType,
};
use google_generative_ai_rs::v1::gemini::{Content, Part, Role};
use log::warn;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::{Duration, Instant};

#[derive(Serialize, Deserialize, Debug)]
struct MelodyPattern {
    notes: Vec<char>,
    durations: Vec<u64>,
    mood: String,
}

pub struct GeminiPlayer {
    client: Client,
    current_pattern: Option<MelodyPattern>,
    last_note_time: Instant,
    pattern_index: usize,
}

impl GeminiPlayer {
    pub fn new(api_key: &str) -> Self {
        // Create a client specifically for the Gemini Pro model with GenerateContent response type
        let client = Client::new_from_model_response_type(
            Model::GeminiPro,
            api_key.to_string(),
            ResponseType::GenerateContent,
        );

        Self {
            client,
            current_pattern: None,
            last_note_time: Instant::now(),
            pattern_index: 0,
        }
    }

    fn extract_json(text: &str) -> Option<String> {
        // Remove all whitespace and normalize the JSON
        let cleaned = text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect::<String>()
            .replace("json", "")
            .replace("```", "")
            .trim()
            .to_string();

        match (cleaned.find('{'), cleaned.rfind('}')) {
            (Some(start), Some(end)) if start <= end => {
                let json = cleaned[start..=end].to_string();
                // Compact the JSON to remove any formatting issues
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json) {
                    return Some(parsed.to_string());
                }
            }
            _ => {}
        }
        None
    }

    pub async fn generate_melody(&mut self, mood: &str) -> Result<(), Box<dyn Error>> {
        let prompt = format!(
            r#"Create a unique musical melody pattern that expresses a {} mood.
                        Generate random notes and durations within these constraints:
                        - notes: use only these letters in any order: a,s,d,f,g,h,j,k,l
                        - notes are mapped like this:
                        'a' => 440.0,  // A4
                        's' => 493.88, // B4
                        'd' => 523.25, // C4
                        'f' => 587.33, // D4
                        'g' => 659.25, // E4
                        'h' => 698.46, // F4
                        'j' => 783.99, // G4
                        'k' => 880.00, // A5
                        'l' => 987.77, // B5
                        - durations: use random numbers(integers) to make the melody better between 100 and 800
                        - create exactly 9 notes with corresponding durations which really match the mood as a melody.
                        Return only a JSON object with this structure:
                        {{
                            "notes": [array of 9 random notes],
                            "durations": [array of 9 random durations],
                            "mood": "{}"
                        }}"#,
            mood, mood
        );

        let content = Content {
            role: Role::User,
            parts: vec![Part {
                text: Some(prompt),
                inline_data: None,
                file_data: None,
                video_metadata: None,
            }],
        };

        let request = Request::new(
            vec![content],
            Vec::<Tools>::new(),
            Vec::<SafetySettings>::new(),
            Some(GenerationConfig {
                temperature: Some(0.9),
                top_p: Some(0.95),
                top_k: Some(40),
                candidate_count: Some(1),
                max_output_tokens: Some(1000),
                stop_sequences: None,
                response_mime_type: None,
            }),
        );

        let response = self.client.post(30, &request).await?;

        match response {
            PostResult::Rest(gemini_response) => {
                if let Some(candidate) = gemini_response.candidates.get(0) {
                    if let Some(part) = candidate.content.parts.get(0) {
                        if let Some(text) = &part.text {
                            if let Some(json_text) = Self::extract_json(text) {
                                match serde_json::from_str::<MelodyPattern>(&json_text) {
                                    Ok(pattern) => {
                                        if pattern.notes.len() == pattern.durations.len()
                                            && pattern
                                                .notes
                                                .iter()
                                                .all(|c| "asdfghjkl".contains(*c))
                                            && pattern
                                                .durations
                                                .iter()
                                                .all(|&d| d >= 200 && d <= 800)
                                        {
                                            self.current_pattern = Some(pattern);
                                            self.pattern_index = 0;
                                            return Ok(());
                                        }
                                    }
                                    Err(e) => {
                                        return Err(format!(
                                            "JSON parsing error: {}. Cleaned JSON was: {}",
                                            e, json_text
                                        )
                                        .into());
                                    }
                                }
                            }
                        }
                    }
                }
                Err("Failed to generate valid melody pattern".into())
            }
            PostResult::Streamed(_) => Err("Streamed response not supported".into()),
            PostResult::Count(_) => Err("Token count response not expected".into()),
        }
    }

    pub fn has_pattern(&self) -> bool {
        self.current_pattern.is_some()
    }

    pub fn get_next_note(&mut self) -> Option<(char, Duration)> {
        if let Some(pattern) = &self.current_pattern {
            let now = Instant::now();

            if self.pattern_index >= pattern.notes.len() {
                self.pattern_index = 0;
                self.last_note_time = now;
            }

            if self.pattern_index == 0
                || now.duration_since(self.last_note_time)
                    >= Duration::from_millis(pattern.durations[self.pattern_index - 1])
            {
                let note = pattern.notes[self.pattern_index];
                let duration = pattern.durations[self.pattern_index];
                self.pattern_index += 1;
                self.last_note_time = now;

                Some((note, Duration::from_millis(duration)))
            } else {
                None
            }
        } else {
            None
        }
    }
}
