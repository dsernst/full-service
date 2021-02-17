// Copyright (c) 2020-2021 MobileCoin Inc.

#![feature(proc_macro_hygiene, decl_macro)]
// Required because hashbrown is at 0.9.1 and has build issues otherwise
#![feature(ptr_offset_from)]

pub mod config;
mod db;
mod error;
mod service;

pub use db::WalletDb;
pub use service::{wallet, WalletService};

extern crate alloc;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[allow(unused_imports)] // Needed for json!
#[macro_use]
extern crate rocket_contrib;
#[allow(unused_imports)] // Needed for embedded_migrations!
#[macro_use]
extern crate diesel_migrations;

#[cfg(any(test, feature = "test_utils"))]
mod test_utils;