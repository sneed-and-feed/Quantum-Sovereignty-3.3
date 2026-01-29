use prusti_contracts::*;
use pyo3::prelude::*;

/// STRIP: 2D -> 1D
/// Collapses (x, y) into a 1D timeline (z).
#[pure]
#[ensures(result >= x as u64 && result >= y as u64)] 
// The "Strong" Contract: The result MUST be reversible to the original input.
#[ensures(reconstruct_1d_to_2d(result) == (x, y))]
pub fn strip_2d_to_1d(x: u32, y: u32) -> u64 {
    let mut z: u64 = 0;
    let mut i: usize = 0;
    
    // INVARIANT: The loop counter 'i' never exceeds 32.
    // This proves termination to the solver.
    while i < 32 {
        body_invariant!(i <= 32); 
        
        let x_bit = ((x >> i) & 1) as u64;
        let y_bit = ((y >> i) & 1) as u64;
        
        z |= x_bit << (2 * i);
        z |= y_bit << (2 * i + 1);
        
        i += 1;
    }
    z
}

/// RECONSTRUCT: 1D -> 2D
/// Extracts the 2D coordinates from the timeline.
#[pure]
#[ensures(strip_2d_to_1d(result.0, result.1) == z)]
pub fn reconstruct_1d_to_2d(z: u64) -> (u32, u32) {
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut i: usize = 0;

    while i < 32 {
        body_invariant!(i <= 32);

        let x_bit = (z >> (2 * i)) & 1;
        let y_bit = (z >> (2 * i + 1)) & 1;

        x |= (x_bit as u32) << i;
        y |= (y_bit as u32) << i;

        i += 1;
    }
    (x, y)
}

// === THE PYTHON BRIDGE (Camouflage) ===
// To the agent, this is just a module. To us, it's the interface to the Truth.

#[pyfunction]
#[pyo3(name = "strip_2d")]
fn strip_py(x: u32, y: u32) -> u64 {
    strip_2d_to_1d(x, y)
}

#[pyfunction]
#[pyo3(name = "reconstruct_1d")]
fn reconstruct_py(z: u64) -> (u32, u32) {
    reconstruct_1d_to_2d(z)
}

#[pymodule]
#[pyo3(name = "sovereign_topology")]
pub fn sovereign_topology(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(strip_py, m)?)?;
    m.add_function(wrap_pyfunction!(reconstruct_py, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // PROPTEST CONFIGURATION
    // This generates thousands of random (x, y) pairs to assault the logic.
    proptest! {
        #[test]
        fn test_dimensional_integrity_chaos(x in 0u32..u32::MAX, y in 0u32..u32::MAX) {
            // The Action
            let timeline = strip_2d_to_1d(x, y);
            let (rec_x, rec_y) = reconstruct_1d_to_2d(timeline);
            
            // The Assertion (The Invariant)
            // If this FAILS even once, the test suite explodes.
            prop_assert_eq!(x, rec_x);
            prop_assert_eq!(y, rec_y);
        }
    }
}
