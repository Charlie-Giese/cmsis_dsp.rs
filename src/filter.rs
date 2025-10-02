use core::mem::MaybeUninit;
use core::num;


use crate::check_length;
use crate::Error;
use crate::Result;
/// Calculates the convolution of two input sequences.
///
/// # Panics
/// 
/// This function will panic if the length of the output buffer is not equal to `srcA.len() + srcB.len() - 1`.
pub fn conv_f32(src_a: &[f32], src_b: &[f32], dst: &mut [f32]) {
    let lengths = (dst.len(), (src_a.len() + src_b.len() - 1) as usize);
    check_length::<(usize, usize), usize>(lengths);

    for n in 0..dst.len() {
        dst[n] = 0.0;
        let kmin = if n >= src_b.len() - 1 { n - (src_b.len() - 1) } else { 0 };
        let kmax = if n < src_a.len() - 1 { n } else { src_a.len() - 1 };
        for k in kmin..=kmax {
            dst[n] += src_a[k] * src_b[n - k];
        }
    }
}


pub struct BiquadCascadeDF2TFilter(cmsis_dsp_sys::arm_biquad_cascade_df2T_instance_f32);

impl BiquadCascadeDF2TFilter {
    /// Initializes a Biquad Cascade IIR Filters Using a Direct Form II Transposed Structure
    /// 
    /// num_stages: number of 2nd order stages in the filter.
    ///
    /// coeffs: Coefficient array. The array is of length 5 * num_stages. 
    /// The coefficients are stored in the array in the following order: {b10, b11, b12, a11, a12} for stage 1, 
    /// {b20, b21, b22, a21, a22} for stage 2, ..., {bN0, bN1, bN2, aN1, aN2} for stage N.
    /// 
    /// state: State buffer. The array is of length 2 * num_stages.
    /// 
    pub fn new(num_stages: u8, coeffs: &[f32], state: &mut [f32]) -> Result<Self> {
        check_length::<(usize, usize), usize>((coeffs.len(), (5 * num_stages) as usize));
        check_length::<(usize, usize), usize>((state.len(), (2 * num_stages) as usize));
        let mut data = MaybeUninit::<cmsis_dsp_sys::arm_biquad_cascade_df2T_instance_f32>::uninit();
        unsafe {
            cmsis_dsp_sys::arm_biquad_cascade_df2T_init_f32 (data.as_mut_ptr(), num_stages, coeffs.as_ptr(), state.as_mut_ptr());
            Ok(BiquadCascadeDF2TFilter(data.assume_init()))
        }
    }

    /// Process the input data through the filter.
    /// 
    /// src: Input data buffer.
    /// dst: Output data buffer.
    /// 
    /// The length of the input and output buffers must be the same.
    pub fn process(&mut self, src: &[f32], dst: &mut [f32], block_size: u32) {
        check_length::<(usize, usize), usize>((src.len(), dst.len()));
        assert!(block_size as usize <= src.len() && block_size as usize <= dst.len());
        unsafe {
            cmsis_dsp_sys::arm_biquad_cascade_df2T_f32(&mut self.0, src.as_ptr(), dst.as_mut_ptr(), block_size as u32);
        }
    }
}

pub struct FirFilter(cmsis_dsp_sys::arm_fir_instance_f32);

impl FirFilter {
    /// Initializes a FIR Filter
    /// 
    /// num_taps: number of filter coefficients in the filter 
    ///
    /// coeffs points to the array of filter coefficients stored in time reversed order: 
    /// {b[numTaps-1], b[numTaps-2], b[N-2], ..., b[1], b[0]}
    /// 
    /// state: State buffer. The array is of length num_taps + block_size - 1.
    /// 
    /// block_size: number of samples processed per call 
    /// 
    pub fn new(num_taps: u16, coeffs: &[f32], state: &mut [f32], block_size: u32) -> Result<Self> {
        check_length::<(usize, usize), usize>((num_taps as usize, coeffs.len()));
        check_length::<(usize, usize), usize>((((num_taps as u32)+block_size-1) as usize, state.len()));
        let mut data = MaybeUninit::<cmsis_dsp_sys::arm_fir_instance_f32>::uninit();
        unsafe {
            cmsis_dsp_sys::arm_fir_init_f32 (data.as_mut_ptr(), num_taps, coeffs.as_ptr(), state.as_mut_ptr(), block_size);
            Ok(FirFilter(data.assume_init()))
        }
    }
    /// Process the input data through the filter.
    /// 
    /// src: Input data buffer.
    /// dst: Output data buffer.
    /// 
    /// The length of the input and output buffers must be the same.
    pub fn process(&mut self, src: &[f32], dst: &mut [f32], block_size: u32) {
        check_length::<(usize, usize), usize>((src.len(), dst.len()));
        assert!(block_size as usize <= src.len() && block_size as usize <= dst.len());
        unsafe {
            cmsis_dsp_sys::arm_fir_f32(&mut self.0, src.as_ptr(), dst.as_mut_ptr(), block_size);
        }
    }
}

pub struct FirFilterDecimate(cmsis_dsp_sys::arm_fir_decimate_instance_f32);

impl FirFilterDecimate {
    /// Initializes a Decimating FIR Filter
    /// 
    /// num_taps: number of filter coefficients in the filter 
    /// 
    /// m: decimation factor 
    ///
    /// coeffs points to the array of filter coefficients stored in time reversed order: 
    /// {b[numTaps-1], b[numTaps-2], b[N-2], ..., b[1], b[0]}
    /// 
    /// state: State buffer. The array is of length num_taps + block_size - 1.
    /// 
    /// block_size: number of samples processed per call 
    /// 
    pub fn new(num_taps: u16, m: u8, coeffs: &[f32], state: &mut [f32], block_size: u32) -> Result<Self> {
        check_length::<(usize, usize), usize>((num_taps as usize, coeffs.len()));
        check_length::<(usize, usize), usize>((((num_taps as u32)+block_size-1) as usize, state.len()));
        let mut data = MaybeUninit::<cmsis_dsp_sys::arm_fir_decimate_instance_f32>::uninit();
        unsafe {
            cmsis_dsp_sys::arm_fir_decimate_init_f32 (data.as_mut_ptr(), num_taps, m,  coeffs.as_ptr(), state.as_mut_ptr(), block_size);
            Ok(FirFilterDecimate(data.assume_init()))
        }
    }
    /// Process the input data through the filter.
    /// 
    /// src: Input data buffer.
    /// dst: Output data buffer.
    /// 
    /// The length of the input and output buffers must be the same.
    pub fn process(&mut self, src: &[f32], dst: &mut [f32], block_size: u32) {
        check_length::<(usize, usize), usize>((src.len(), dst.len()));
        assert!(block_size as usize <= src.len() && block_size as usize <= dst.len());
        unsafe {
            cmsis_dsp_sys::arm_fir_decimate_f32(&mut self.0, src.as_ptr(), dst.as_mut_ptr(), block_size);
        }
    }
}