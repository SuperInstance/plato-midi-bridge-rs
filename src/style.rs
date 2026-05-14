/// 109-dimensional musical style vector.
use crate::EisensteinLattice;

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

    /// Cosine similarity
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

    /// Reduce to 5-dim style primitives using positions 0-108:
    /// pitch_complexity: avg of dims 0-47 (first 48 pitch bins)
    /// timing_expressiveness: dims 48-55 (timing-related features)
    /// velocity_energy: dims 56-63 (velocity-related)
    /// articulation_clarity: dims 64-71 (articulation)
    /// timbral_breadth: dims 72-79 (timbre/register)
    pub fn to_5d(&self) -> [f64; 5] {
        let pitch = self.dims[..48].iter().sum::<f64>() / 48.0 * 12.0;
        let timing = self.dims[48..56].iter().sum::<f64>() / 8.0 * 100.0;
        let velocity = self.dims[56..64].iter().sum::<f64>() / 8.0;
        let articulation = 1.0 - (self.dims[64..72].iter().sum::<f64>() / 8.0);
        let timbre = self.dims[72..80].iter().sum::<f64>() / 8.0;
        [pitch, timing, velocity, articulation, timbre]
    }

    /// Reduce to 12-dim Eisenstein coupling vector
    pub fn to_12d(&self) -> [f64; 12] {
        let mut coupling = [0.0; 12];
        for i in 0..12 {
            coupling[i] = self.dims[i.min(108)];
        }
        coupling
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EisensteinLattice;

    #[test]
    fn test_style_vector_creation() {
        let data = vec![0.5; 109];
        let v = StyleVector::new(&data);
        assert!((v.dims[0] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_cosine_similarity_identical() {
        let data: Vec<f64> = (0..109).map(|i| (i as f64) / 109.0).collect();
        let v1 = StyleVector::new(&data);
        let v2 = StyleVector::new(&data);
        assert!((v1.cosine_similarity(&v2) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_5d_reduction() {
        let data: Vec<f64> = (0..109).map(|i| (i as f64) / 50.0).collect();
        let v = StyleVector::new(&data);
        let d5 = v.to_5d();
        assert_eq!(d5.len(), 5);
        for val in &d5 {
            assert!(val.is_finite());
        }
    }

    #[test]
    fn test_eisenstein_chamber_from_style() {
        let data: Vec<f64> = (0..109).map(|i| if i < 12 { (i+1) as f64 / 12.0 } else { 0.0 }).collect();
        let v = StyleVector::new(&data);
        let coupling = v.to_12d();
        let chamber = EisensteinLattice::chamber(&coupling);
        assert!(chamber < 12);
    }

    #[test]
    fn test_orthogonal_vectors_distant() {
        let v1 = StyleVector::new(&[1.0; 109]);
        let v2 = StyleVector::new(&[0.0; 109]);
        let sim = v1.cosine_similarity(&v2);
        assert!((sim - 0.0).abs() < 1e-10);
    }
}
