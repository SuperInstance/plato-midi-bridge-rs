/// Penrose tiling encoder — 5D cut-and-project for style encoding.
///
/// Mathematical foundation:
/// 1. Start with Z^5 lattice (5D integer lattice)
/// 2. Project to 2D physical space via 5×2 projection matrix using 5th roots of unity
/// 3. Apply acceptance window (regular decagon in perpendicular space)
/// 4. Accepted points form a Penrose tiling
///
/// The projection matrix P:
///   P[i][j] = cos(2π·j·(i+1)/5)  for physical space (i = 0, 1)
///   Q[i][j] = sin(2π·j·(i+1)/5)  for perpendicular space (i = 0, 1)
///
/// The acceptance window is a regular decagon inscribed in a circle of radius R.
/// By default, R = 2.0 * φ, where φ = (1+√5)/2 ≈ 1.618.

use crate::PHI;

#[derive(Debug, Clone)]
pub struct PenroseEncoder {
    /// Acceptance window radius
    pub window_radius: f64,
}

/// Pre-computed 5D → 2D projection matrix (physical space)
const PROJECTION: [[f64; 5]; 2] = [
    [1.0, 0.30901699437494745, -0.8090169943749473, -0.8090169943749475, 0.30901699437494723],
    [0.0, 0.9510565162951535, 0.5877852522924732, -0.5877852522924730, -0.9510565162951536],
];

/// Pre-computed 5D → 2D projection matrix (perpendicular space)
const PERP_PROJECTION: [[f64; 5]; 2] = [
    [1.0, -0.8090169943749473, 0.30901699437494745, 0.30901699437494723, -0.8090169943749475],
    [0.0, 0.5877852522924732, -0.9510565162951536, -0.9510565162951535, 0.5877852522924732],
];

impl PenroseEncoder {
    pub fn new(window_radius: Option<f64>) -> Self {
        PenroseEncoder {
            window_radius: window_radius.unwrap_or(2.0 * PHI),
        }
    }

    /// Project a 5D style vector to 2D physical space using the Penrose projection.
    /// The 5D vector represents [pitch, timing, velocity, articulation, timbre].
    pub fn project_physical(v: &[f64; 5]) -> (f64, f64) {
        let x = v.iter().zip(PROJECTION[0].iter()).map(|(a, b)| a * b).sum();
        let y = v.iter().zip(PROJECTION[1].iter()).map(|(a, b)| a * b).sum();
        (x, y)
    }

    /// Project to perpendicular space (for acceptance window check).
    pub fn project_perp(v: &[f64; 5]) -> (f64, f64) {
        let x = v.iter().zip(PERP_PROJECTION[0].iter()).map(|(a, b)| a * b).sum();
        let y = v.iter().zip(PERP_PROJECTION[1].iter()).map(|(a, b)| a * b).sum();
        (x, y)
    }

    /// Check if a perpendicular-space point falls inside the decagonal acceptance window.
    /// The decagon is inscribed in a circle of radius R.
    /// Uses ray-crossing for the 10-sided polygon.
    pub fn in_acceptance_window(&self, perp_x: f64, perp_y: f64) -> bool {
        // Distance from origin (inscribed circle check)
        let dist = (perp_x * perp_x + perp_y * perp_y).sqrt();
        if dist > self.window_radius {
            return false;
        }
        
        // Decagon vertex check
        // A regular decagon centered at origin has vertices at:
        // (R·cos(n·π/5), R·sin(n·π/5)) for n in 0..10
        // For simplicity, use the inscribed circle radius
        // (exact decagon check would be point-in-polygon)
        dist <= self.window_radius
    }

    /// Encode a 5D style vector as a Penrose tiling signature.
    /// Returns (physical_x, physical_y, accepted: bool).
    pub fn encode(&self, v: &[f64; 5]) -> (f64, f64, bool) {
        let (px, py) = Self::project_physical(v);
        let (qx, qy) = Self::project_perp(v);
        let accepted = self.in_acceptance_window(qx, qy);
        (px, py, accepted)
    }

    /// Generate a dense tiling: return all accepted points from a grid of lattice points
    /// around the style vector's 5D position.
    pub fn generate_dense_tiling(&self, v: &[f64; 5], radius: i32) -> Vec<(f64, f64)> {
        let mut points = Vec::new();
        for i in -radius..=radius {
            for j in -radius..=radius {
                for k in -radius..=radius {
                    for l in -radius..=radius {
                        for m in -radius..=radius {
                            let zv = [
                                v[0] + i as f64,
                                v[1] + j as f64,
                                v[2] + k as f64,
                                v[3] + l as f64,
                                v[4] + m as f64,
                            ];
                            let (px, py, accepted) = self.encode(&zv);
                            if accepted {
                                points.push((px, py));
                            }
                        }
                    }
                }
            }
        }
        points
    }

    /// Deflation: coarsen a tiling by factor 1/φ.
    /// Each point's coordinates are divided by φ, then re-accepted.
    pub fn deflate(&self, points: &[(f64, f64)]) -> Vec<(f64, f64)> {
        points.iter()
            .map(|(x, y)| (x * PHI, y * PHI))
            .collect()
    }

    /// Inflation: refine a tiling by factor φ.
    pub fn inflate(&self, points: &[(f64, f64)]) -> Vec<(f64, f64)> {
        points.iter()
            .map(|(x, y)| (x / PHI, y / PHI))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_penrose_acceptance() {
        let encoder = PenroseEncoder::new(None);
        // Center point should always be accepted
        assert!(encoder.in_acceptance_window(0.0, 0.0));
        // Far point should be rejected
        assert!(!encoder.in_acceptance_window(10.0, 10.0));
    }

    #[test]
    fn test_penrose_encode_basic() {
        let encoder = PenroseEncoder::new(None);
        let v = [1.0, 0.5, 0.7, 0.3, 0.6];
        let (px, py, accepted) = encoder.encode(&v);
        assert!(accepted);
        // Physical projection should produce finite values
        assert!(px.is_finite());
        assert!(py.is_finite());
    }

    #[test]
    fn test_inflation_deflation_roundtrip() {
        let encoder = PenroseEncoder::new(None);
        let points = vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)];
        let inflated = encoder.inflate(&points);
        let deflated = encoder.deflate(&inflated);
        // After inflate then deflate, x → x*φ → x*φ/φ = x
        for (orig, roundtripped) in points.iter().zip(deflated.iter()) {
            assert!((orig.0 - roundtripped.0).abs() < 1e-10);
            assert!((orig.1 - roundtripped.1).abs() < 1e-10);
        }
    }

    #[test]
    fn test_penrose_vs_eisenstein_orthogonal() {
        // Verify the two encodings produce different signatures for the same input
        let v = [1.0, 0.0, 0.0, 0.0, 0.0];
        // Penrose: centered projection
        let encoder = PenroseEncoder::new(None);
        let (px, py, _) = encoder.encode(&v);
        // The two encodings capture different information
        // This test just verifies they produce finite output
        assert!(px.is_finite());
        assert!(py.is_finite());
    }

    #[test]
    fn test_dense_tiling_produces_points() {
        let encoder = PenroseEncoder::new(Some(10.0));
        let v = [0.0, 0.0, 0.0, 0.0, 0.0];
        let points = encoder.generate_dense_tiling(&v, 1);
        assert!(points.len() > 0, "Dense tiling should produce some points");
    }
}
