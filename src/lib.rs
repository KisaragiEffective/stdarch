//! SIMD support
//!
//! This crate provides the fundamentals of supporting SIMD in Rust. This crate
//! should compile on all platforms and provide `simd` and `vendor` modules at
//! the top-level. The `simd` module contains *portable vector types* which
//! should work across all platforms and be implemented in the most efficient
//! manner possible for the platform at hand. The `vendor` module contains
//! vendor intrinsics that operate over these SIMD types, typically
//! corresponding to a particular CPU instruction
//!
//! ```rust
//! extern crate stdsimd;
//! use stdsimd::simd::u32x4;
//!
//! fn main() {
//!     let a = u32x4::new(1, 2, 3, 4);
//!     let b = u32x4::splat(10);
//!     assert_eq!(a + b, u32x4::new(11, 12, 13, 14));
//! }
//! ```
//!
//! > **Note**: This crate is *nightly only* at the moment, and requires a
//! > nightly rust toolchain to compile.
//!
//! ## Portability
//!
//! The `simd` module and its types should be portable to all platforms. The
//! runtime characteristics of these types may vary per paltform and per CPU
//! feature enabled, but they should always have the most optimized
//! implementation for the target at hand.
//!
//! The `vendor` module provides no portability guarantees. The `vendor` module
//! is per CPU architecture currently and provides intrinsics corresponding to
//! functions for that particular CPU architecture. Note that the functions
//! provided in this module are intended to correspond to CPU instructions and
//! have no runtime support for whether you CPU actually supports the
//! instruction.
//!
//! CPU target feature detection is done via the `cfg_feature_enabled!` macro at
//! runtime. This macro will detect at runtime whether the specified feature is
//! available or not, returning true or false depending on the current CPU.
//!
//! ```
//! #![feature(cfg_target_feature)]
//!
//! #[macro_use]
//! extern crate stdsimd;
//!
//! fn main() {
//!     if cfg_feature_enabled!("avx2") {
//!         println!("avx2 intrinsics will work");
//!     } else {
//!         println!("avx2 intrinsics will not work, they may generate SIGILL");
//!     }
//! }
//! ```
//!
//! # Status
//!
//! This crate is intended for eventual inclusion into the standard library, but
//! some work and experimentation is needed to get there! First and foremost you
//! can help out by kicking the tires on this crate and seeing if it works for
//! your use case! Next up you can help us fill out the [vendor
//! intrinsics][vendor] to ensure that we've got all the SIMD support necessary.
//!
//! The language support and status of SIMD is also still a little up in the air
//! right now, you may be interested in a few issues along these lines:
//!
//! * [Overal tracking issue for SIMD support](https://github.com/rust-lang/rust/issues/27731)
//! * [`cfg_target_feature` tracking issue](https://github.com/rust-lang/rust/issues/29717)
//! * [SIMD types currently not sound](https://github.com/rust-lang/rust/issues/44367)
//! * [`#[target_feature]` improvements](https://github.com/rust-lang/rust/issues/44839)
//!
//! [vendor]: https://github.com/rust-lang-nursery/stdsimd/issues/40

#![allow(dead_code)]
#![feature(
    const_fn, link_llvm_intrinsics, platform_intrinsics, repr_simd, simd_ffi,
    target_feature, cfg_target_feature, i128_type, asm, const_atomic_usize_new
)]
#![cfg_attr(test, feature(proc_macro))]

#[cfg(test)]
extern crate assert_instr;

/// Platform independent SIMD vector types and operations.
pub mod simd {
    pub use v128::*;
    pub use v256::*;
    pub use v512::*;
    pub use v64::*;
}

/// Platform dependent vendor intrinsics.
pub mod vendor {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub use x86::*;

    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    pub use arm::*;
}

#[macro_use]
mod macros;
mod simd_llvm;
mod v128;
mod v256;
mod v512;
mod v64;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[macro_use]
mod x86;

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
mod arm;
