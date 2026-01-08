//! ATC Engine - Prompt building and response handling
//!
//! Constructs context-aware prompts for the LLM based on telemetry.

use crate::ollama::OllamaClient;
use crate::telemetry::Telemetry;

/// ATC Engine - manages the conversation and prompt construction
pub struct AtcEngine {
    ollama: OllamaClient,
    conversation_history: Vec<ConversationEntry>,
    callsign: String,
    aircraft_type: String,
}

#[derive(Debug, Clone)]
pub struct ConversationEntry {
    pub speaker: Speaker,
    pub message: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Speaker {
    Pilot,
    Atc,
}

impl AtcEngine {
    /// Create a new ATC engine
    pub fn new(callsign: impl Into<String>, aircraft_type: impl Into<String>) -> Self {
        Self {
            ollama: OllamaClient::default(),
            conversation_history: Vec::new(),
            callsign: callsign.into(),
            aircraft_type: aircraft_type.into(),
        }
    }
    
    /// Set the Ollama model
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.ollama = OllamaClient::new(model);
        self
    }
    
    /// Build the ATC system prompt
    fn build_system_prompt(&self, telemetry: &Telemetry) -> String {
        let altitude_ft = (telemetry.position.altitude_msl_m * 3.28084) as i32;
        let heading = telemetry.orientation.heading_mag as i32;
        let ground_speed_kts = (telemetry.speed.ground_speed_mps * 1.94384) as i32;
        
        let phase = if telemetry.state.on_ground {
            "on the ground"
        } else if altitude_ft < 1000 {
            "in the pattern"
        } else {
            "in flight"
        };
        
        format!(
            r#"You are an FAA Air Traffic Controller. Respond with proper ATC phraseology.

AIRCRAFT: {callsign} ({aircraft_type})
POSITION: {phase} at {altitude_ft} ft MSL, heading {heading}°, {ground_speed_kts} kts
SQUAWK: {squawk:04}

RULES:
1. Use standard FAA phraseology
2. Be concise - real ATC is brief
3. Include callsign in every transmission
4. If unclear, ask pilot to "say again"

Respond ONLY with what ATC would say. No explanations."#,
            callsign = self.callsign,
            aircraft_type = self.aircraft_type,
            phase = phase,
            altitude_ft = altitude_ft,
            heading = heading,
            ground_speed_kts = ground_speed_kts,
            squawk = telemetry.transponder.code,
        )
    }
    
    /// Process pilot input and generate ATC response
    pub async fn process_pilot_input(
        &mut self,
        pilot_message: &str,
        telemetry: &Telemetry,
    ) -> Result<String, crate::ollama::OllamaError> {
        // Add pilot message to history
        self.conversation_history.push(ConversationEntry {
            speaker: Speaker::Pilot,
            message: pilot_message.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        });
        
        // Build the full prompt
        let system_prompt = self.build_system_prompt(telemetry);
        let history = self.format_history();
        
        let full_prompt = format!(
            "{system_prompt}\n\nCONVERSATION:\n{history}\nPILOT: {pilot_message}\nATC:",
        );
        
        // Get LLM response
        let response = self.ollama.generate(&full_prompt).await?;
        let response = response.trim().to_string();
        
        // Add ATC response to history
        self.conversation_history.push(ConversationEntry {
            speaker: Speaker::Atc,
            message: response.clone(),
            timestamp: chrono::Utc::now().timestamp(),
        });
        
        // Keep history manageable (last 10 exchanges)
        if self.conversation_history.len() > 20 {
            self.conversation_history.drain(0..2);
        }
        
        Ok(response)
    }
    
    /// Format conversation history for the prompt
    fn format_history(&self) -> String {
        self.conversation_history
            .iter()
            .map(|entry| {
                let speaker = match entry.speaker {
                    Speaker::Pilot => "PILOT",
                    Speaker::Atc => "ATC",
                };
                format!("{}: {}", speaker, entry.message)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    /// Get conversation history
    pub fn history(&self) -> &[ConversationEntry] {
        &self.conversation_history
    }
    
    /// Clear conversation history
    pub fn clear_history(&mut self) {
        self.conversation_history.clear();
    }
}
