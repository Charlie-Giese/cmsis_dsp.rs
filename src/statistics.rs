use crate::check_length;

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

pub fn xa() {}
