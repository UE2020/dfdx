use super::NegateKernelOp;
use crate::tensor_ops::cuda_kernels::cuda_unary;

unsafe impl cudarc::driver::DeviceRepr for NegateKernelOp {}

const PTX: &str = include_str!(concat!(env!("OUT_DIR"), "/negate.ptx"));

#[cfg(feature = "f16")]
cuda_unary!(const_df() NegateKernelOp, half::f16, PTX, "negate_fwd_f16", "negate_bwd_f16");
cuda_unary!(const_df() NegateKernelOp, f32, PTX, "negate_fwd_f32", "negate_bwd_f32");
#[cfg(feature = "cuda-f64")]
cuda_unary!(const_df() NegateKernelOp, f64, PTX, "negate_fwd_f64", "negate_bwd_f64");
