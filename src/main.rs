#![deny(clippy::unwrap_used)]
#![warn(clippy::shadow_same)]
#![warn(clippy::shadow_reuse)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::nursery)]

mod cli;

fn main() {
    let args = cli::parse();

    println!("{:?}", args);
}
