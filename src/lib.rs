#![feature(associated_type_defaults)]
#![feature(array_chunks)]
#![feature(array_windows)]
#![feature(iter_array_chunks)]
#![feature(result_option_inspect)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::wildcard_imports,
    clippy::default_trait_access,
    clippy::cast_lossless
)]

pub mod days;
pub mod solver;
pub(crate) mod util;
