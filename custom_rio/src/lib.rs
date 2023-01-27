//! Custom rio a reproduction in a live of
//! my experimental runtime RIO
//! that lives https://github.com/vincenzopalazzo/rio/tree/main/rt/src
//!
//! author: Vincenzo Palazzo <vincenzopalazzodev@gmail.com>
#![feature(once_cell)]
pub mod runtime;
mod runtime_impl;
pub mod task;
use runtime_impl::CustomRio;
