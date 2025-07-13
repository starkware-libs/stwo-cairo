//! invokes relation_tracker on debug artifacts (mem, trace, etc.)

use std::collections::HashMap;
use std::env::args;
use std::path::PathBuf;

use cairo_air::preprocessed::PreProcessedTrace;
use clap::Parser;
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;
use stwo_cairo_adapter::vm_import::adapt_vm_output;
use stwo_cairo_adapter::{log_prover_input, ProverInput};
use stwo_cairo_prover::debug_tools::relation_tracker::relation_tracker;
use stwo_cairo_prover::witness::prelude::M31;
use stwo_constraint_framework::relation_tracker::{RelationSummary, RelationTrackerEntry};

/// Command line arguments for relation_tracker.
/// Example command line:
///     ```
///     RUSTFLAGS="-C target-cpu=native" cargo run -r --bin relation_tracker -- --apriv \
///      absolute/path/to/apriv --apui absolute/path/to/apui
///     ```
#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    apui: PathBuf,
    #[clap(long)]
    apriv: PathBuf,
    #[clap(long)]
    summary_path: PathBuf,
    #[clap(long)]
    blame_path: Option<PathBuf>,
}

fn main() {
    tracing_subscriber::fmt().init();
    let Args {
        apui,
        apriv,
        summary_path,
        blame_path,
    } = Args::parse_from(args());

    let prover_input: ProverInput = adapt_vm_output(apui.as_path(), apriv.as_path()).unwrap();
    log_prover_input(&prover_input);

    let relation_entries = relation_tracker(prover_input, PreProcessedTrace::canonical());
    let summary = RelationSummary::summarize_relations(&relation_entries)
        .cleaned()
        .0;
    log::info!("Entries per relation:");
    for (relation, values) in &summary {
        log::info!("{}: {}", relation, values.len());
    }

    // Serialize summary to file
    tracing::info!("Serializing summary to file");
    let summary_json = sonic_rs::to_string(&summary).unwrap();
    std::fs::write(summary_path, summary_json).unwrap();

    if let Some(blame_path) = blame_path {
        let blame_map = reduce_blame(summary, relation_entries);
        let blame_json = sonic_rs::to_string(&blame_map).unwrap();
        std::fs::write(blame_path, blame_json).unwrap();
    }
}

type BlameMap = HashMap<String, HashMap<Vec<M31>, Vec<((String, u32, u32), u32)>>>;

#[allow(clippy::type_complexity)]
fn reduce_blame(
    mut cleaned_summary: Vec<(String, Vec<(Vec<M31>, M31)>)>,
    relation_entries: HashMap<String, Vec<RelationTrackerEntry>>,
) -> BlameMap {
    // Sort the summarized entries.
    for (_, values) in &mut cleaned_summary {
        values.sort_by(|a, b| a.0.cmp(&b.0));
    }

    // Reduce the entry map to only the 'bad' entries in the 'bad' relations.
    let mut reduced_map = HashMap::new();
    for (relation, entries) in relation_entries {
        let bad_entries = cleaned_summary
            .iter()
            .find(|(bad_relation, _)| bad_relation == &relation);
        let bad_entries = if let Some((_, bad_entries)) = bad_entries {
            bad_entries
        } else {
            continue;
        };

        let blame_vec: Vec<_> = entries
            .par_chunks(1 << 15)
            .flat_map(|chunk| {
                let mut blame_vec = vec![];
                for entry in chunk {
                    if bad_entries
                        .binary_search_by(|(bad_value, _)| bad_value.cmp(&entry.values))
                        .is_ok()
                    {
                        blame_vec.push(entry.clone());
                    }
                }
                blame_vec
            })
            .collect();
        reduced_map.insert(relation, blame_vec);
    }

    // Create the blame map.
    let mut blame_map = HashMap::new();
    for (relation_name, blame_vec) in reduced_map {
        let mut relation_blame = HashMap::new();
        blame_vec
            .into_iter()
            .group_by(|a| a.values.clone())
            .into_iter()
            .for_each(|(value_vector, blame_group)| {
                let mut group_blame = vec![];
                let blame_group = blame_group.collect_vec();
                for culprit in blame_group {
                    let location = culprit.location;
                    let mult = culprit.mult.0;
                    group_blame.push((location, mult));
                }
                assert!(relation_blame.insert(value_vector, group_blame).is_none());
            });
        blame_map.insert(relation_name, relation_blame);
    }

    blame_map
}
