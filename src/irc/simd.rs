#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
pub(super) mod x86_sse;

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
pub(super) use x86_sse::*;

#[cfg(all(target_arch = "x86_64", not(target_feature = "sse2")))]
const _: () = {
  compile_error!("cannot use SIMD - your CPU does not support sse2");
};

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub(super) mod arm_neon;

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub(super) use arm_neon::*;

#[cfg(all(target_arch = "aarch64", not(target_feature = "neon")))]
const _: () = {
  compile_error!("cannot use SIMD - your CPU does not support Neon");
};
