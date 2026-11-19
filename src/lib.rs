#![forbid(unsafe_code)]
//! ttfx - Terminal text effects engine for SigmaOS

#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    FadeIn,
    FadeOut,
    Typewriter,
    Matrix,
    Sparkle,
    Wave,
}

#[derive(Debug, Clone)]
pub struct EffectConfig {
    pub effect: Effect,
    pub speed_ms: u64,
    pub repeat: bool,
}

impl EffectConfig {
    pub fn new(effect: Effect, speed_ms: u64, repeat: bool) -> Self {
        Self {
            effect,
            speed_ms,
            repeat,
        }
    }
    pub fn default_speed(&self) -> u64 {
        self.speed_ms
    }
}

pub struct TextEffect {
    pub text: String,
    pub config: EffectConfig,
    pub frames: Vec<String>,
}

impl TextEffect {
    pub fn new(text: &str, config: EffectConfig) -> Self {
        let frames = Self::generate_frames(text, &config);
        Self {
            text: text.to_string(),
            config,
            frames,
        }
    }

    fn generate_frames(text: &str, config: &EffectConfig) -> Vec<String> {
        match config.effect {
            Effect::Typewriter => (1..=text.len()).map(|i| text[..i].to_string()).collect(),
            Effect::FadeIn => vec![
                format!("\x1b[2m{}\x1b[0m", text),
                format!("\x1b[1m{}\x1b[0m", text),
                text.to_string(),
            ],
            Effect::FadeOut => vec![
                text.to_string(),
                format!("\x1b[1m{}\x1b[0m", text),
                format!("\x1b[2m{}\x1b[0m", text),
            ],
            Effect::Matrix => text
                .chars()
                .enumerate()
                .map(|(i, _)| {
                    let fill: String = (0..i).map(|_| '*').collect();
                    format!("{}{}", fill, &text[i..])
                })
                .collect(),
            Effect::Sparkle => vec![
                format!("✨ {} ✨", text),
                format!("⭐ {} ⭐", text),
                text.to_string(),
            ],
            Effect::Wave => (0..text.len())
                .map(|i| {
                    let mut chars: Vec<char> = text.chars().collect();
                    if i < chars.len() {
                        chars[i] = chars[i].to_uppercase().next().unwrap_or(chars[i]);
                    }
                    chars.into_iter().collect()
                })
                .collect(),
        }
    }

    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    pub fn get_frame(&self, index: usize) -> Option<&str> {
        self.frames.get(index).map(|s| s.as_str())
    }
}


#[cfg(test)]
mod ttfx_extra_tests {
    use super::*;

    #[test]
    fn test_ttfx_fade_in_effect_aaa() {
        // Arrange
        let config = EffectConfig::new(Effect::FadeIn, 50, false);
        // Act
        let effect = TextEffect::new("SIGMA", config);
        // Assert
        assert!(effect.frame_count() > 0);
        assert!(effect.get_frame(999).is_none());
    }

    #[test]
    fn test_ttfx_empty_text_effect_aaa() {
        // Arrange
        let config = EffectConfig::new(Effect::Typewriter, 100, false);
        // Act
        let effect = TextEffect::new("", config);
        // Assert
        assert_eq!(effect.frame_count(), 0);
    }

    #[test]
    fn test_ttfx_repeat_flag_preservation_aaa() {
        // Arrange & Act
        let config = EffectConfig::new(Effect::Matrix, 25, true);
        // Assert
        assert!(config.repeat);
        assert_eq!(config.speed_ms, 25);
    }
}
