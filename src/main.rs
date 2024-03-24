#![deny(clippy::unwrap_used)]
#![warn(clippy::shadow_same)]
#![warn(clippy::shadow_reuse)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::nursery)]

mod cli;
mod generate;
mod newpost;

use std::path::{Path, PathBuf};

fn main() {
    let args = cli::parse();

    println!("{:?}", args);
}

fn find_content_dir(root: &Path, lang: &str) -> PathBuf {
    let mut result = root.to_owned();
    result.push("content");
    result.push(lang);

    result
}
