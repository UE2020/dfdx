use crate::tensor_ops::cuda_kernels::cuda_unary;

unsafe impl cudarc::driver::DeviceRepr for super::CosKernelOp {}

const PTX: &str = include_str!(concat!(env!("OUT_DIR"), "/cos.ptx"));

#[cfg(feature = "f16")]
cuda_unary!(
    super::CosKernelOp,
    half::f16,
    PTX,
    "cos_fwd_f16",
    "cos_bwd_f16"
);
cuda_unary!(super::CosKernelOp, f32, PTX, "cos_fwd_f32", "cos_bwd_f32");
#[cfg(feature = "cuda-f64")]
cuda_unary!(super::CosKernelOp, f64, PTX, "cos_fwd_f64", "cos_bwd_f64");
