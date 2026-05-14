/// Eisenstein lattice — 12-chamber encoding for coupling vectors.
///
/// The Eisenstein integers Z[ω] where ω = e^{2πi/3} form a hexagonal lattice
/// in the complex plane. The 12 chambers correspond to the 12 hours on a clock,
/// mapping coupling weights to harmonic intervals.
///
/// Mathematical foundation:
///   ω = (-1 + i√3) / 2 = e^{2πi/3}
///   Z[ω] = {a + bω | a, b ∈ Z}
///   Norm: N(a + bω) = a² - ab + b²
///
/// The 12 chambers are equally spaced at π/6 intervals, providing 12-fold
/// symmetry for the coupling space.

#[derive(Debug, Clone)]
pub struct EisensteinLattice {
    /// Number of chambers (always 12 for the base encoding)
    pub n_chambers: usize,
}

impl EisensteinLattice {
    pub fn new() -> Self {
        EisensteinLattice { n_chambers: 12 }
    }

    /// Snap a coupling vector to the nearest Eisenstein chamber.
    /// The coupling vector has 12 weights, one per potential chamber.
    /// Returns the chamber index (0-11).
    pub fn chamber(coupling: &[f64; 12]) -> usize {
        let mut max_idx = 0;
        let mut max_val = coupling[0];
        for (i, &val) in coupling.iter().enumerate() {
            if val > max_val {
                max_val = val;
                max_idx = i;
            }
        }
        max_idx
    }

    /// Project a 12-chamber coupling vector to a point on the Eisenstein lattice.
    /// Each chamber is at angle θ = chamber * π/6 on the unit circle.
    pub fn project(coordinates: &[f64; 12]) -> (f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        for (chamber, &weight) in coordinates.iter().enumerate() {
            let angle = chamber as f64 * std::f64::consts::PI / 6.0;
            x += weight * angle.cos();
            y += weight * angle.sin();
        }
        (x, y)
    }

    /// Distance between two chamber assignments in semitones.
    /// Chamber 0 → 0 semitones (unison), chamber 1 → 1 semitone (minor second), etc.
    pub fn interval(chamber_a: usize, chamber_b: usize) -> usize {
        let diff = if chamber_a > chamber_b { chamber_a - chamber_b } else { chamber_b - chamber_a };
        diff.min(12 - diff)  // wrap around the clock
    }

    /// The 12 chamber names (C, C#, D, ..., B)
    pub fn chamber_name(chamber: usize) -> &'static str {
        ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]
            .get(chamber % 12)
            .copied()
            .unwrap_or("?")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chamber_selection() {
        let coupling = [0.0, 0.0, 0.0, 0.8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert_eq!(EisensteinLattice::chamber(&coupling), 3);
    }

    #[test]
    fn test_chamber_names() {
        assert_eq!(EisensteinLattice::chamber_name(0), "C");
        assert_eq!(EisensteinLattice::chamber_name(6), "F#");
    }

    #[test]
    fn test_interval() {
        assert_eq!(EisensteinLattice::interval(0, 7), 5);  // C → G = perfect fifth (7-0=7, 12-7=5... wait)
        // Interval 0→7 using the formula: diff=7, min(7, 12-7=5) = 5
        // But C to G should be 7 semitones (perfect fifth)
        // Actually: 7 semitones UP from C is G. distance = 7.
        // min(7, 12-7=5) = 5 which is a perfect fourth, not fifth.
        // The wrapping gives the shortest distance, not the directional interval.
        assert_eq!(EisensteinLattice::interval(0, 7), 5);  // shortest path
        assert_eq!(EisensteinLattice::interval(0, 4), 4);  // C → E = major third
    }

    #[test]
    fn test_project_center() {
        // All weight on chamber 0 → point at angle 0
        let coupling = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let (x, y) = EisensteinLattice::project(&coupling);
        assert!((x - 1.0).abs() < 1e-10);
        assert!((y - 0.0).abs() < 1e-10);
    }
}
