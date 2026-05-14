//! Plato-MIDI Bridge — Style decomposition for MIDI with Eisenstein lattices and Penrose tilings.
//!
//! ## Core Types
//!
//! - `StyleVector` — 109-dimensional musical style vector
//! - `EisensteinLattice` — 12-chamber hexagonal lattice encoding
//! - `PenroseEncoder` — 5D cut-and-project tiling encoding
//! - `ScaleLevel` — Multi-scale analysis (micro, note, phrase, section, piece)
//!
//! ## Usage
//!
//! ```rust
//! use plato_midi_bridge::{StyleVector, EisensteinLattice, PenroseEncoder};
//!
//! // Create a style vector from raw features
//! let style = StyleVector::new(&[
//!     0.5, 0.3, 0.8, 0.1, 0.6,  // pitch dims
//!     // ... 109 dims total
//! ]);
//!
//! // Encode in Eisenstein lattice
//! let chamber = EisensteinLattice::chamber(&style, &[0.1, 0.2, 0.3, 0.4, 0.5, 0.6]);
//!
//! // Encode in Penrose tiling
//! let penrose = PenroseEncoder::encode(&style);
//! ```

mod eisenstein;
mod penrose;
mod style;
mod multiscale;

pub use eisenstein::EisensteinLattice;
pub use penrose::PenroseEncoder;
pub use style::StyleVector;
pub use multiscale::ScaleLevel;

use std::f64::consts::{PI, SQRT_2};

/// Golden ratio φ = (1 + √5) / 2
pub const PHI: f64 = 1.6180339887498948482045868343656381177;

/// The 5th roots of unity for Penrose projection
pub const FIFTH_ROOTS: [(f64, f64); 5] = [
    (1.0, 0.0),
    (0.30901699437494745, 0.9510565162951535),   // e^{2πi/5}
    (-0.8090169943749473, 0.5877852522924732),    // e^{4πi/5}
    (-0.8090169943749475, -0.5877852522924730),   // e^{6πi/5}
    (0.30901699437494723, -0.9510565162951536),   // e^{8πi/5}
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phi_precision() {
        assert!((PHI - 1.618033988749895).abs() < 1e-14);
    }

    #[test]
    fn test_fifth_roots_length() {
        assert_eq!(FIFTH_ROOTS.len(), 5);
        for (re, im) in &FIFTH_ROOTS {
            let mag = (re * re + im * im).sqrt();
            assert!((mag - 1.0).abs() < 1e-14, "Root magnitude should be 1, got {}", mag);
        }
    }
}
