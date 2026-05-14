/// 109-dimensional musical style vector.
///
/// Dimensions:
/// - 0-127: Pitch histogram (128 bins)
/// - 128-255: Velocity histogram (128 bins) 
/// - 256: Timing consistency (onset deviation std)
/// - 257: Staccato ratio [0, 1]
/// - 258: Average interval (semitones)
/// - 259: Dynamic range (max - min velocity)
/// - 260: Note density (notes/second)
/// - 261: Syncopation index [0, 1]
/// - 262: Harmonic complexity (unique pitch classes/window)
/// - 263: Register breadth (pitch span)
/// - 264-273: Reserved for multi-scale features (10 dims)
///
/// Total: 128 + 128 + 11 = 267 dims planned for v2.0
/// Current: 109 dims (first release)

#[derive(Debug, Clone)]
pub struct StyleVector {
    pub dims: [f64; 109],
}

impl StyleVector {
    pub fn new(data: &[f64]) -> Self {
        let mut dims = [0.0; 109];
        let n = data.len().min(109);
        dims[..n].copy_from_slice(&data[..n]);
        StyleVector { dims }
    }

    /// Cosine similarity with another style vector
    pub fn cosine_similarity(&self, other: &StyleVector) -> f64 {
        let dot: f64 = self.dims.iter().zip(other.dims.iter()).map(|(a, b)| a * b).sum();
        let norm1: f64 = self.dims.iter().map(|a| a * a).sum::<f64>().sqrt();
        let norm2: f64 = other.dims.iter().map(|a| a * a).sum::<f64>().sqrt();
        if norm1 == 0.0 || norm2 == 0.0 { return 0.0; }
        (dot / (norm1 * norm2)).clamp(0.0, 1.0)
    }

    /// Euclidean distance
    pub fn euclidean_distance(&self, other: &StyleVector) -> f64 {
        self.dims.iter().zip(other.dims.iter())
            .map(|(a, b)| (a - b) * (a - b))
            .sum::<f64>()
            .sqrt()
    }

    /// Reduce 109-dim to 5-dim style primitives (pitch, timing, velocity, articulation, timbre)
    pub fn to_5d(&self) -> [f64; 5] {
        let pitch_complexity = self.dims[260] * 12.0;   // note density scaled
        let timing_expressiveness = self.dims[256] * 100.0 * (1.0 + self.dims[261]); // timing × syncopation
        let velocity_energy = self.dims[259] / 127.0;     // dynamic range normalized
        let articulation_clarity = 1.0 - self.dims[257];  // 1 - staccato = legato
        let timbral_breadth = self.dims[263] / 127.0;     // register breadth normalized
        [pitch_complexity, timing_expressiveness, velocity_energy, articulation_clarity, timbral_breadth]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_vector_creation() {
        let data = vec![0.5; 109];
        let v = StyleVector::new(&data);
        assert!((v.dims[0] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_cosine_similarity_identical() {
        let v1 = StyleVector::new(&[1.0; 109]);
        let v2 = StyleVector::new(&[1.0; 109]);
        assert!((v1.cosine_similarity(&v2) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_5d_reduction() {
        let v = StyleVector::new(&[0.5; 109]);
        let d5 = v.to_5d();
        assert_eq!(d5.len(), 5);
    }
}
