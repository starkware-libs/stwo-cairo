use std::{collections::HashMap, sync::Mutex};

use itertools::Itertools;
use once_cell::sync::Lazy;
use prover_types::simd::N_LANES;
use stwo_prover::core::{backend::simd::{conversion::Pack, m31::PackedM31}, fields::m31::M31};

pub mod memory;
pub mod opcodes;
pub mod range_check_unit;
pub mod range_check_vector;
pub mod verifyinstruction;

pub fn pack_values<T: Pack>(values: &[T]) -> Vec<T::SimdType> {
    values
        .array_chunks::<N_LANES>()
        .map(|c| T::pack(*c))
        .collect()
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct VectorEntry {
    values: Vec<u32>,
}

#[derive(Debug)]
pub struct RelationTracker {
    relations: HashMap<String, Vec<(VectorEntry, String)>>,
    yields: HashMap<String, Vec<(VectorEntry, String, usize)>>,
}

impl RelationTracker {
    // Creates a new RelationTracker
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            relations: HashMap::new(),
            yields: HashMap::new(),
        }
    }

    // Adds elements to a specific relation, attributing them to a given blame (user).
    pub fn add_to_relation(&mut self, relation_name: &str, input: &[M31], blame: &str) {
        let mut values = input.iter().map(|v| v.0).collect_vec();
        while values.len() > 1 && values.last() == Some(&0) {
            values.pop();
        }
        let vector_entry = VectorEntry { values };
        // Get the relation entry or create it if it doesn't exist
        let relation = self.relations.entry(relation_name.to_string()).or_default();

        // Insert the elements and blame as a tuple
        relation.push((vector_entry, blame.to_string()));
    }

    pub fn add_packed_to_relation(
        &mut self,
        relation_name: &str,
        input: &[PackedM31],
        blame: &str,
    ) {
        for i in 0..N_LANES {
            let mut values = vec![];
            for packed in input {
                values.push(packed.to_array()[i]);
            }
            self.add_to_relation(relation_name, &values, blame);
        }
    }

    // Records yield data as a vector with a blame and yield multiplicity
    pub fn yield_values(
        &mut self,
        relation_name: &str,
        values: &[M31],
        multiplicity: usize,
        blame: &str,
    ) {
        if multiplicity == 0 {
            return;
        }
        let mut values = values.to_vec();
        while values.len() > 1 && values.last() == Some(&M31(0)) {
            values.pop();
        }

        let mut values = values.iter().map(|v| v.0).collect_vec();
        while values.len() > 1 && values.last() == Some(&0) {
            values.pop();
        }
        let vector_entry = VectorEntry { values };
        // Get the yield entry for the relation or create it if it doesn't exist
        let yield_entries = self.yields.entry(relation_name.to_string()).or_default();

        // Insert the yielded vector, blame, and multiplicity as a tuple
        yield_entries.push((vector_entry, blame.to_string(), multiplicity));
    }

    pub fn yield_values_packed(
        &mut self,
        relation_name: &str,
        values: &[PackedM31],
        multiplicity: PackedM31,
        blame: &str,
    ) {
        for i in 0..N_LANES {
            let mut unpacked_values = vec![];
            for packed in values {
                unpacked_values.push(packed.to_array()[i]);
            }
            self.yield_values(
                relation_name,
                &unpacked_values,
                multiplicity.to_array()[i].0 as usize,
                blame,
            );
        }
    }

    pub fn summarize_relations(&self) {
        for (relation_name, entries) in &self.relations {
            println!("Relation: {}", relation_name);

            // Count additions for each vector of values and collect blame info
            let mut add_count: HashMap<&VectorEntry, usize> = HashMap::new();
            let mut blame_info: HashMap<&VectorEntry, Vec<&String>> = HashMap::new();
            for (vector, blame) in entries {
                *add_count.entry(vector).or_insert(0) += 1;
                blame_info.entry(vector).or_default().push(blame);
            }

            // Compute yield sums for each vector of values
            let mut yield_sum: HashMap<&VectorEntry, usize> = HashMap::new();
            if let Some(yield_entries) = self.yields.get(relation_name) {
                for (vector, _blame, multiplicity) in yield_entries {
                    *yield_sum.entry(vector).or_insert(0) += multiplicity;
                }
            }

            // Display addition counts, yield sums, and blame info for each unique vector of values
            for (vector, count) in &add_count {
                let total_yield = yield_sum.get(vector).unwrap_or(&0);

                if count != total_yield {
                    // Highlight mismatch with color, special characters, and blame information
                    println!(
                        "*** Vector {:?}: Added {} times, Total yield sum {} ***",
                        vector.values, count, total_yield
                    );

                    // Display blame information for each addition
                    for blame in &blame_info[vector] {
                        println!("      - Added by: {}", blame);
                    }
                } else {
                    // Regular print for matching counts
                    println!(
                        "  Vector {:?}: Added {} times, Total yield sum {}",
                        vector.values, count, total_yield
                    );
                }
            }

            for (vector, sum) in &yield_sum {
                let total_add = add_count.get(vector).unwrap_or(&0);

                if *total_add == 0 {
                    // Highlight mismatch with color, special characters, and blame information
                    println!(
                        "*** Vector {:?}: Added {} times, Total yield sum {} ***",
                        vector.values, 0, sum
                    );
                }
            }
            println!();
        }
    }
}


pub static GLOBAL_TRACKER: Lazy<Mutex<RelationTracker>> = Lazy::new(|| {
    Mutex::new(RelationTracker::new())
});