//! Backing library for bughunt

#![cfg_attr(feature = "cargo-clippy", allow(clippy::cargo))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::complexity))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::correctness))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::perf))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![cfg_attr(feature = "cargo-clippy", feature(tool_lints))]

#[deny(bad_style)]
#[deny(future_incompatible)]
#[deny(missing_docs)]
#[deny(nonstandard_style)]
#[deny(rust_2018_compatibility)]
#[deny(rust_2018_idioms)]
#[deny(unused)]
#[deny(warnings)]
extern crate clap;

use clap::ArgMatches;
use std::{env, process};

/// The Bughunt project type, root
pub struct BugHunt {}

impl BugHunt {
    pub fn new() -> Self {
        BugHunt {}
    }

    pub fn build(&self, _args: &ArgMatches) -> () {
        let cargo_path = env!("CARGO");

        let mut rustflags: String = "--cfg fuzzing \
                                     -C debug-assertions \
                                     -C overflow_checks \

                                     -C passes=sancov \
                                     -C llvm-args=-sanitizer-coverage-trace-pc-guard \
                                     -C llvm-args=-sanitizer-coverage-level=4 \
                                     -C llvm-args=-sanitizer-coverage-prune-blocks=0 \
                                     -C llvm-args=-sanitizer-coverage-trace-divs \

                                     -C opt-level=0 \
                                     -C debuginfo=2 \
                                     -C target-cpu=native"
            .to_string();

        // add user provided flags
        let other_flags = env::var("RUSTFLAGS").unwrap_or_default();
        if !other_flags.is_empty() {
            rustflags.push_str(" ");
            rustflags.push_str(&other_flags);
        }

        let mut cmd = process::Command::new(cargo_path);
        cmd.arg("build").arg("--release");
        // .arg("--target")
        // .arg(target_triple);

        let status = cmd.env("RUSTFLAGS", &rustflags).status().unwrap();
        process::exit(status.code().unwrap_or(1));
    }
}
