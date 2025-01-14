use super::TanhKernelOp;
use crate::tensor_ops::cuda_kernels::cuda_unary;

unsafe impl cudarc::driver::DeviceRepr for TanhKernelOp {}

const PTX: &str = include_str!(concat!(env!("OUT_DIR"), "/tanh.ptx"));

#[cfg(feature = "f16")]
cuda_unary!(df(f(x)) TanhKernelOp, half::f16, PTX, "tanh_fwd_f16", "tanh_bwd_f16");
cuda_unary!(df(f(x)) TanhKernelOp, f32, PTX, "tanh_fwd_f32", "tanh_bwd_f32");
#[cfg(feature = "cuda-f64")]
cuda_unary!(df(f(x)) TanhKernelOp, f64, PTX, "tanh_fwd_f64", "tanh_bwd_f64");
