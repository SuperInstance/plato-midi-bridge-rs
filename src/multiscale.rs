/// Multi-scale analysis — micro, note, phrase, section, piece.
///
/// Inspired by Penrose tiling inflation/deflation:
/// - MICRO (25ms): onset jitter, velocity micro-variation
/// - NOTE (250ms): pitch, velocity, duration per note
/// - PHRASE (2-8 bars): melodic contour, dynamic arc
/// - SECTION (8-32 bars): harmonic rhythm, structural boundaries
/// - PIECE (full): composer fingerprint
///
/// The inflation ratios between scales approximate φ:
///   MICRO→NOTE ≈ 10 (~2π/φ)
///   NOTE→PHRASE ≈ 8
///   PHRASE→SECTION ≈ 4
///   SECTION→PIECE ≈ 3

#[derive(Debug, Clone)]
pub struct ScaleLevel {
    pub name: &'static str,
    pub window_seconds: f64,
    pub features: Vec<(&'static str, f64)>,
}

impl ScaleLevel {
    pub fn new(name: &'static str, window_seconds: f64) -> Self {
        ScaleLevel {
            name,
            window_seconds,
            features: Vec::new(),
        }
    }

    pub fn add(&mut self, name: &'static str, value: f64) {
        self.features.push((name, value));
    }
}

/// Compute inflation ratio between two adjacent scales.
pub fn inflation_ratio(lower: &ScaleLevel, upper: &ScaleLevel) -> f64 {
    if lower.window_seconds == 0.0 { return 0.0; }
    upper.window_seconds / lower.window_seconds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_creation() {
        let micro = ScaleLevel::new("micro", 0.025);
        assert_eq!(micro.name, "micro");
        assert!((micro.window_seconds - 0.025).abs() < 1e-6);
    }

    #[test]
    fn test_inflation_ratio() {
        let micro = ScaleLevel::new("micro", 0.025);
        let note = ScaleLevel::new("note", 0.25);
        let ratio = inflation_ratio(&micro, &note);
        assert!((ratio - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_feature_addition() {
        let mut phrase = ScaleLevel::new("phrase", 2.0);
        phrase.add("melodic_contour", 0.5);
        phrase.add("dynamic_arc", 0.7);
        assert_eq!(phrase.features.len(), 2);
    }
}
