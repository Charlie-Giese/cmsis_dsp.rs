use crate::check_length;
use fixed::types::{I16F48, I18F14, I1F15, I1F31, I1F7, I34F30};


/// Calculates the rms of a sequence of f32 values.
///
/// # Panics
///
/// This function panics if src does not have 'size' elements.
pub fn rms_f32(src: &[f32], size: usize, dst: &mut f32) {
    let length = check_length((src.len(), size));
    unsafe {
        cmsis_dsp_sys::arm_abs_f32(src.as_ptr(), dst, length);
    }
}

/// Calculates the mean of a sequence of f32 values.
///
/// # Panics
///
/// This function panics if src does not have 'size' elements.
pub fn mean_f32(src: &[f32], size: usize, dst: &mut f32) {
    let length = check_length((src.len(), size));
    unsafe {
        cmsis_dsp_sys::arm_mean_f32(src.as_ptr(), length, dst);
    }
}

/// Calculates the variance of a sequence of f32 values.
///
/// # Panics
///
/// This function panics if src does not have 'size' elements.
pub fn var_f32(src: &[f32], size: usize, dst: &mut f32) {
    let length = check_length((src.len(), size));
    unsafe {
        cmsis_dsp_sys::arm_var_f32(src.as_ptr(), length, dst);
    }
}

/// Calculates the standard deviation of a sequence of f32 values.
///
/// # Panics
///
/// This function panics if src does not have 'size' elements.
pub fn std_f32(src: &[f32], size: usize, dst: &mut f32) {
    let length = check_length((src.len(), size));
    unsafe {
        cmsis_dsp_sys::arm_std_f32(src.as_ptr(), length, dst);
    }
}

/// Calculates the maximum value in a sequence of f32 values, along with its' index.
///
/// # Panics
///
/// This function panics if src does not have 'size' elements.
pub fn max_f32(src: &[f32], size: usize, dst_value: &mut f32, dst_index: &mut u32) {
    let length = check_length((src.len(), size));
    unsafe {
        cmsis_dsp_sys::arm_max_f32(src.as_ptr(), length, dst_value, dst_index);
    }
}

/// Calculates the minimum value in a sequence of f32 values, along with its' index.
///
/// # Panics
///
/// This function panics if src does not have 'size' elements.
pub fn min_f32(src: &[f32], size: usize, dst_value: &mut f32, dst_index: &mut u32) {
    let length = check_length((src.len(), size));
    unsafe {
        cmsis_dsp_sys::arm_min_f32(src.as_ptr(), length, dst_value, dst_index);
    }
}

/// Calculates the mean square error value of a sequence of f32 values.
///
/// # Panics
///
/// This function panics if srcs does not have 'size' elements.
pub fn mse_f32(src_a: &[f32], src_b: &[f32], size: usize, dst: &mut f32) {
    let length = check_length((src_a.len(), src_b.len(), size));
    unsafe {
        cmsis_dsp_sys::arm_mse_f32(src_a.as_ptr(), src_b.as_ptr(), length, dst);
    }
}

/// Calculates the mean square error value of a sequence of q31 values.
///
/// # Panics
///
/// This function panics if srcs does not have 'size' elements.
pub fn mse_q31(src_a: &[I1F31], src_b: &[I1F31], size: usize, dst: &mut [I1F31]) {
    let length = check_length((src_a.len(), src_b.len(), size));
    unsafe {
        cmsis_dsp_sys::arm_mse_q31(src_a.as_ptr() as *const _, src_b.as_ptr() as *const _, length, dst.as_mut_ptr() as *mut _);
    }
}

/// Calculates the mean square error value of a sequence of q15 values.
///
/// # Panics
///
/// This function panics if src does not have 'size' elements.
pub fn mse_q15(src_a: &[I1F15], src_b: &[I1F15], size: usize, dst: &mut [I1F15]) {
    let length = check_length((src_a.len(), src_b.len(), size));
    unsafe {
        cmsis_dsp_sys::arm_mse_q15(src_a.as_ptr() as *const _, src_b.as_ptr() as *const _, length, dst.as_mut_ptr() as *mut _);
    }
}

