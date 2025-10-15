use clap::{Parser, ValueEnum};
use dev_utils::preprocessed_roots::{export_preprocessed_roots, HashSelection};

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum HashArg {
    Blake,
    Poseidon,
    Both,
}

impl From<HashArg> for HashSelection {
    fn from(value: HashArg) -> Self {
        match value {
            HashArg::Blake => HashSelection::Blake,
            HashArg::Poseidon => HashSelection::Poseidon,
            HashArg::Both => HashSelection::Both,
        }
    }
}

/// Export preprocessed commitment roots for Cairo preprocessed traces.
///
/// Computes the commitment roots for Blake2s and/or Poseidon252 Merkle channels for all
/// log blowup factors in the range 1..=max.
///
/// Performance note: This computation is slow. It is strongly recommended to run with
/// release optimizations:
///
///   cargo run --release --bin export_preprocessed_roots -- [OPTIONS]
///
/// Examples:
///   # Compute both sets of roots up to the default max (2)
///   cargo run --release --bin export_preprocessed_roots
///
///   # Compute only Blake roots up to 4
///   cargo run --release --bin export_preprocessed_roots -- -m 4 --hash blake
#[derive(Parser, Debug)]
#[command(name = "export_preprocessed_roots")]
struct Args {
    /// Maximum log blowup factor (inclusive), computes for 1..=max
    #[arg(short = 'm', long = "max-log-blowup-factor", default_value_t = 2)]
    max_log_blowup_factor: u32,

    /// Hash function to compute roots for: blake, poseidon, or both
    #[arg(long = "hash", value_enum, default_value_t = HashArg::Both)]
    hash: HashArg,
}

fn main() {
    let args = Args::parse();
    export_preprocessed_roots(args.max_log_blowup_factor, args.hash.into());
}


