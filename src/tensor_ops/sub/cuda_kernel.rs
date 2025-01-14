use super::{BinarySubKernelOp as Binary, ScalarSubKernelOp as Scalar};
use crate::tensor_ops::cuda_kernels::{cuda_binary, cuda_unary};

#[cfg(feature = "f16")]
unsafe impl cudarc::driver::DeviceRepr for Scalar<half::f16> {}
unsafe impl cudarc::driver::DeviceRepr for Scalar<f32> {}
#[cfg(feature = "cuda-f64")]
unsafe impl cudarc::driver::DeviceRepr for Scalar<f64> {}
unsafe impl cudarc::driver::DeviceRepr for Binary {}

const SCALAR_PTX: &str = include_str!(concat!(env!("OUT_DIR"), "/scalar_sub.ptx"));
const BINARY_PTX: &str = include_str!(concat!(env!("OUT_DIR"), "/binary_sub.ptx"));

#[cfg(feature = "f16")]
cuda_unary!(const_df() Scalar<half::f16>, half::f16, SCALAR_PTX, "ssub_fwd_f16", "ssub_bwd_f16");
cuda_unary!(const_df() Scalar<f32>, f32, SCALAR_PTX, "ssub_fwd_f32", "ssub_bwd_f32");
#[cfg(feature = "cuda-f64")]
cuda_unary!(const_df() Scalar<f64>, f64, SCALAR_PTX, "ssub_fwd_f64", "ssub_bwd_f64");
#[cfg(feature = "f16")]
cuda_binary!(
    const_df() Binary,
    half::f16,
    BINARY_PTX,
    "bsub_fwd_f16",
    "bsub_bwd_lhs_f16",
    "bsub_bwd_rhs_f16"
);
cuda_binary!(
    const_df() Binary,
    f32,
    BINARY_PTX,
    "bsub_fwd_f32",
    "bsub_bwd_lhs_f32",
    "bsub_bwd_rhs_f32"
);
#[cfg(feature = "cuda-f64")]
cuda_binary!(
    const_df() Binary,
    f64,
    BINARY_PTX,
    "bsub_fwd_f64",
    "bsub_bwd_lhs_f64",
    "bsub_bwd_rhs_f64"
);
