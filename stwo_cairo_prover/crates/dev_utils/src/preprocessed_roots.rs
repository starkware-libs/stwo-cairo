use cairo_air::PreProcessedTraceVariant;
use itertools::Itertools;
use stwo::core::channel::MerkleChannel;
use stwo::core::vcs::blake2_merkle::Blake2sMerkleChannel;
use stwo::core::vcs::poseidon252_merkle::Poseidon252MerkleChannel;
use stwo::core::vcs::MerkleHasher;
use stwo::prover::backend::BackendForChannel;
use stwo_cairo_prover::witness::preprocessed_trace::generate_preprocessed_commitment_root;

#[derive(Clone, Copy, Debug)]
pub enum HashSelection {
    Blake,
    Poseidon,
    Both,
}

fn get_preprocessed_roots<MC: MerkleChannel>(
    max_log_blowup_factor: u32,
    preprocessed_trace: PreProcessedTraceVariant,
) -> Vec<<MC::H as MerkleHasher>::Hash>
where
    stwo::prover::backend::simd::SimdBackend: BackendForChannel<MC>,
{
    (1..=max_log_blowup_factor)
        .map(|i| generate_preprocessed_commitment_root::<MC>(i, preprocessed_trace))
        .collect_vec()
}

pub fn export_preprocessed_roots(max_log_blowup_factor: u32, selection: HashSelection) {
    match selection {
        HashSelection::Blake => {
            let roots = get_preprocessed_roots::<Blake2sMerkleChannel>(
                max_log_blowup_factor,
                PreProcessedTraceVariant::Canonical,
            );
            roots.iter().enumerate().for_each(|(i, root)| {
                let root_bytes = root.0;
                let u32s_hex = root_bytes
                    .as_slice()
                    .chunks_exact(4)
                    .map(|bytes| {
                        let arr: [u8; 4] = bytes.try_into().expect("chunk size");
                        format!("{:#010x}", u32::from_le_bytes(arr))
                    })
                    .collect_vec()
                    .join(", ");
                println!("log_blowup_factor: {}, blake root: [{}]", i + 1, u32s_hex);
            });
        }
        HashSelection::Poseidon => {
            println!("Starting Poseidon roots");
            get_preprocessed_roots::<Poseidon252MerkleChannel>(
                max_log_blowup_factor,
                PreProcessedTraceVariant::CanonicalWithoutPedersen,
            )
            .into_iter()
            .enumerate()
            .for_each(|(i, root)| {
                println!(
                    "log_blowup_factor: {}, poseidon root: [{:#010x}]",
                    i + 1,
                    root
                );
            });
        }
        HashSelection::Both => {
            export_preprocessed_roots(max_log_blowup_factor, HashSelection::Blake);
            export_preprocessed_roots(max_log_blowup_factor, HashSelection::Poseidon);
        }
    }
}


