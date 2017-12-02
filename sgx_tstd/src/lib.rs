// Copyright (c) 2017 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! # The Rust SGX SDK Standard Library
//!
//! The Rust SGX standard library (previously named as `sgx_tstdc`) is
//! the foundation of portable Rust SGX SDK, a
//! set of minimal and battle-tested shared abstractions for the Rust SGX
//! ecosystem. Similar to Rust's libstd, it offers core types, like [`Vec<T>`] and
//! [`Option<T>`], library-defined [operations on language
//! primitives](#primitives), [standard macros](#macros), [I/O] and
//! [multithreading], among [many other things][other].
//!
//! `std` is available to all Rust crates by default, just as if each one
//! contained an `extern crate sgx_tstd as std;` import at the [crate root]. Therefore the
//! standard library can be accessed in [`use`] statements through the path
//! `std`, as in [`use std::env`], or in expressions through the absolute path
//! `::std`, as in [`::std::env::args`].

#![crate_name = "sgx_tstd"]
#![crate_type = "rlib"]

#![no_std]
#![needs_panic_runtime]
#![allow(non_camel_case_types)]
#![allow(unused_must_use)]
#![allow(unused_features)]

#![feature(alloc)]
#![feature(global_allocator)]
#![feature(allocator_api)]
#![feature(allocator_internals)]
#![feature(allow_internal_unstable)]
#![feature(align_offset)]
#![feature(array_error_internals)]
#![feature(box_syntax)]
#![feature(cfg_target_has_atomic)]
#![feature(char_error_internals)]
#![feature(collections_range)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(const_max_value)]
#![feature(const_atomic_bool_new)]
#![feature(const_atomic_isize_new)]
#![feature(const_atomic_usize_new)]
#![feature(const_atomic_u64_new)]
#![feature(const_atomic_ptr_new)]
#![feature(const_unsafe_cell_new)]
#![feature(const_cell_new)]
#![feature(const_once_new)]
#![feature(const_ptr_null)]
#![feature(const_ptr_null_mut)]
#![feature(core_intrinsics)]
#![feature(fixed_size_array)]
#![feature(dropck_eyepatch)]
#![feature(fn_traits)]
#![feature(fnbox)]
#![feature(fused)]
#![feature(generic_param_attrs)]
#![feature(i128)]
#![feature(i128_type)]
#![feature(int_error_internals)]
#![feature(integer_atomics)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(macro_vis_matcher)]
#![feature(needs_panic_runtime)]
#![feature(never_type)]
#![feature(optin_builtin_traits)]
#![feature(placement_new_protocol)]
#![feature(prelude_import)]
#![feature(rand)]
#![feature(raw)]
#![feature(rustc_attrs)]
#![feature(shared)]
#![feature(sip_hash_13)]
#![feature(slice_concat_ext)]
#![feature(str_internals)]
#![feature(thread_local)]
#![feature(toowned_clone_into)]
#![feature(try_from)]
#![feature(unboxed_closures)]
#![feature(unicode)]
#![feature(unique)]
#![feature(untagged_unions)]
#![feature(unwind_attributes)]
#![feature(slice_patterns)]
#![feature(panic_unwind)]
#![feature(libc)]

#![default_lib_allocator]

#[global_allocator]
static ALLOC: sgx_alloc::System = sgx_alloc::System;
// Explicitly import the prelude. The compiler uses this same unstable attribute
// to import the prelude implicitly when building crates that depend on std.
#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

// We want to reexport a few macros from core but libcore has already been
// imported by the compiler (via our #[no_std] attribute) In this case we just
// add a new crate name so we can attach the reexports to it.
#[macro_reexport(assert, assert_eq, assert_ne, debug_assert, debug_assert_eq,
                 debug_assert_ne, unreachable, unimplemented, write, writeln, try)]
extern crate core as __core;

#[allow(unused_imports)]
#[macro_use]
#[macro_reexport(vec, format)]
extern crate alloc;
extern crate std_unicode;
#[cfg(feature = "backtrace")]
extern crate libc;

// We always need an unwinder currently for backtraces
#[cfg(feature = "backtrace")]
extern crate unwind;

// compiler-rt intrinsics
extern crate compiler_builtins;

extern crate sgx_alloc;
#[macro_reexport(cfg_if, __cfg_if_items, __cfg_if_apply)]
extern crate sgx_types;
#[macro_reexport(global_ctors_object)]
extern crate sgx_trts;
extern crate sgx_tprotected_fs;

// The standard macros that are not built-in to the compiler.
#[macro_use]
mod macros;

// The Rust prelude
pub mod prelude;

// Public module declarations and reexports

pub use core::any;
pub use core::cell;
pub use core::clone;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::hash;
pub use core::intrinsics;
pub use core::iter;
pub use core::marker;
pub use core::mem;
pub use core::ops;
pub use core::ptr;
pub use core::raw;
pub use core::result;
pub use core::option;
pub use core::isize;
pub use core::i8;
pub use core::i16;
pub use core::i32;
pub use core::i64;
pub use core::i128;
pub use core::usize;
pub use core::u8;
pub use core::u16;
pub use core::u32;
pub use core::u64;
pub use core::u128;
pub use alloc::boxed;
pub use alloc::rc;
pub use alloc::borrow;
pub use alloc::fmt;
pub use alloc::slice;
pub use alloc::str;
pub use alloc::string;
pub use alloc::vec;
pub use std_unicode::char;

pub mod f32;
pub mod f64;

#[macro_use]
pub mod thread;
pub mod ascii;
pub mod collections;
pub mod env;
pub mod error;
pub mod ffi;
pub mod fs;
pub mod io;
pub mod num;
pub mod os;
pub mod panic;
pub mod path;
pub mod sync;
pub mod heap;
pub mod enclave;

// Platform-abstraction modules
mod sys_common;
mod sys;

// Private support modules
mod panicking;
mod cpuid;
mod memchr;

// The runtime entry point and a few unstable public functions used by the
// compiler
pub mod rt;

#[cfg(feature = "backtrace")]
pub mod backtrace;

pub use cpuid::*;
pub use self::thread::{rsgx_thread_self, rsgx_thread_equal};


