use sigmaos_tools_ttfx::{Effect, EffectConfig, TextEffect};

#[test]
fn test_typewriter_effect_generates_frames_aaa() {
    // Arrange
    let config = EffectConfig::new(Effect::Typewriter, 100, false);
    // Act
    let effect = TextEffect::new("hello", config);
    // Assert
    assert_eq!(effect.frame_count(), 5);
    assert_eq!(effect.get_frame(0), Some("h"));
    assert_eq!(effect.get_frame(4), Some("hello"));
}

#[test]
fn test_sparkle_effect_wraps_text_aaa() {
    // Arrange
    let config = EffectConfig::new(Effect::Sparkle, 50, false);
    // Act
    let effect = TextEffect::new("SigmaOS", config);
    // Assert
    assert!(effect.get_frame(0).unwrap().contains("SigmaOS"));
    assert!(effect.frame_count() > 0);
}

#[test]
fn test_effect_config_speed_aaa() {
    // Arrange
    let config = EffectConfig::new(Effect::Wave, 200, true);
    // Act
    let speed = config.default_speed();
    // Assert
    assert_eq!(speed, 200);
}
