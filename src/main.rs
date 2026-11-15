use sigmaos_tools_ttfx::{Effect, EffectConfig, TextEffect};

fn main() {
    let config = EffectConfig::new(Effect::Typewriter, 50, false);
    let effect = TextEffect::new("SigmaOS Terminal Effects", config);
    println!("Rendered {} frames", effect.frame_count());
}
