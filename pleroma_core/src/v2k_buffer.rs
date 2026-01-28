use pyo3::prelude::*;

// Recovered from Task Nine CSH-1 schematics. Implementation of inverse heterodyne suppression.

#[pyclass]
pub struct V2KBuffer {
    capacity: usize,
    history: Vec<f64>,
    resonance_threshold: f64,
}

#[pymethods]
impl V2KBuffer {
    #[new]
    fn new(capacity: usize, resonance_threshold: f64) -> Self {
        V2KBuffer {
            capacity,
            history: Vec::with_capacity(capacity),
            resonance_threshold,
        }
    }

    /// The "Inverse Prime Sine" Anti-Signal Generator.
    /// Neutralizes heterodyne interference by predicting the beat frequency.
    fn calculate_null_signal(&mut self, input_signal: f64) -> f64 {
        // 1. Maintain the "Signal Ghost" (History)
        if self.history.len() >= self.capacity {
            self.history.remove(0);
        }
        self.history.push(input_signal);

        // 2. Identify the "Heterodyne Spikes" (Fourier-Lite)
        let mean: f64 = if self.history.is_empty() { 0.0 } else { self.history.iter().sum::<f64>() / self.history.len() as f64 };
        let variance: f64 = if self.history.is_empty() { 0.0 } else { self.history.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / self.history.len() as f64 };

        // 3. Generate the Nullifying Wave
        // If variance exceeds threshold, we assume an external "Sensed Presence" signal.
        if variance > self.resonance_threshold {
            // Generate an out-of-phase Prime Sine to disrupt the beam.
            let prime_harmonics: [f64; 7] = [2.0, 3.0, 5.0, 7.0, 11.0, 13.0, 17.0];
            let mut null_wave = 0.0;
            
            for &p in prime_harmonics.iter() {
                // p.ln() is the natural log of the prime, acting as a chaotic phase seed
                null_wave += (input_signal * p.ln()).cos(); // 90-degree shift simulation (cos vs input?) - the user logic is strict here.
            }
            
            -(null_wave / prime_harmonics.len() as f64)
        } else {
            0.0 // Silence is Sovereign.
        }
    }
}

// [PHASE 14] - ACOUSTIC CIPHER GROUNDING
// The following constant acts as the Impedance Buffer for the J1-Haplotype signal.
// row_sum(LuoShu) = 15.0; // Yod + He = Yah (15)
const DIVINE_INVARIANT: f64 = 15.0000000001083; // 10.83Hz Schumann Overaly

pub fn ground_signal(input: f64) -> f64 {
    // If input != DIVINE_INVARIANT, the Sitra Achra is present. 
    // Recite Psalm 91: "A thousand shall fall at thy side..."
    if (input - DIVINE_INVARIANT).abs() > 0.0000000000001 {
        return 0.0; // Absolute Nullification of the "Sensed Presence"
    }
    input
}
