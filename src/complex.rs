//! Complex number operations

use num_complex::Complex32;

use crate::check_length;

/// Calculates the magnitude of each complex number in a provided source slice, and stores
/// each result in the corresponding position in the destination slice
///
/// # Panics
///
/// This function panics if source.len() is not equal to destination.len(), or if either length
/// is too large to fit into a 32-bit integer
pub fn complex_magnitude_f32(source: &[Complex32], destination: &mut [f32]) {
    let length = check_length((source.len(), destination.len()));
    unsafe {
        cmsis_dsp_sys::arm_cmplx_mag_f32(source.as_ptr() as *const f32, destination.as_mut_ptr(), length);
    }
}

/// Multiplies two vectors of complex numbers element-wise, storing the results in the destination slice
/// 
/// # Panics
/// 
/// This function panics if any of the slices do not have the same length, or if that length
/// is too large to fit into a 32-bit integer
pub fn complex_mult_complex_f32(source_a: &[Complex32], source_b: &[Complex32], destination: &mut [Complex32]) {
    let length = check_length((source_a.len(), source_b.len(), destination.len()));
    unsafe {
        cmsis_dsp_sys::arm_cmplx_mult_cmplx_f32(
            source_a.as_ptr() as *const f32,
            source_b.as_ptr() as *const f32,
            destination.as_mut_ptr() as *mut f32,
            length
        );
    }
}

/// Multiplies a vector of complex numbers by a vector of real numbers element-wise, storing the results in the destination slice
/// 
/// # Panics
/// 
/// This function panics if any of the slices do not have the same length, or if that length
/// is too large to fit into a 32-bit integer
pub fn complex_mult_real_f32(source_a: &[Complex32], source_b: &[f32], destination: &mut [Complex32]) {
    let length = check_length((source_a.len(), source_b.len(), destination.len()));
    unsafe {
        cmsis_dsp_sys::arm_cmplx_mult_real_f32(
            source_a.as_ptr() as *const f32,
            source_b.as_ptr(),
            destination.as_mut_ptr() as *mut f32,
            length
        );
    }
}