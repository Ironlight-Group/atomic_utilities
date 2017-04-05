#![cfg_attr(use_asm, feature(asm))]
//! This library contains a hodgepodge of various utilities
//! for writing fast lockfree code.
//! The current modules are:
//!
//!  1. artificial_dep: Used for creating fake data dependencies and eliding fences
//!
//!  2. fence_rmw: Used for writing RMW operations that act as a full SeqCst fence in an efficient manner
//!
//!  See each module for more documentation

pub mod artificial_dep;
pub mod fence_rmw;
