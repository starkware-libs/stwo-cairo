use stwo_cairo_verifier::channel::ChannelTrait;
use stwo_cairo_verifier::fields::qm31::{QM31, qm31};
use stwo_cairo_verifier::fields::m31::{m31};
use stwo_cairo_verifier::vcs::verifier::MerkleDecommitment;
use stwo_cairo_verifier::channel::Channel;

#[derive(Drop)]
pub struct FriLayerProof {
    pub evals_subset: Array<QM31>,
    pub decommitment: MerkleDecommitment,
    pub decomposition_coeff: QM31,
    pub commitment: felt252,
}

#[derive(Drop)]
pub struct FriProof {
    pub inner_layers: Array<FriLayerProof>,
    pub last_layer_poly: Array<QM31>,
    pub last_layer_poly_log_size: usize
}

#[derive(Copy, Default, Drop)]
pub struct FriVerifier {
    log_blowup_factor: u32,
    log_degree_bound: usize
}

#[generate_trait]
impl FriVerifierImpl of FriVerifierTrait {
    fn verify(self: FriVerifier, channel: Channel, proof: FriProof) -> bool {
        self.commit_phase(channel, array![2]);
        //self.query_phase();
        true
    }

    fn commit_phase(self: @FriVerifier, mut channel: Channel, column_bounds: Array<u32>) {
        let expected_query_log_domain_size = *column_bounds[0] + *self.log_blowup_factor;
        let circle_poly_alpha = channel.draw_felt();
    }

    fn query_phase(self: FriVerifier) {

    }
}


use core::poseidon::poseidon_hash_span;
#[test]
fn test_fri_verifier() {
    let proof = FriProof {
        inner_layers: array![FriLayerProof {
            evals_subset: array![qm31(1654551922, 1975507039, 724492960, 302041406)],
            decommitment: MerkleDecommitment {
                hash_witness: array![
                    0x02894fb64f5b5ad74ad6868ded445416d52840c2c4a36499f0eb37a03841bfc8,
                    0x05d3f79e2cfd15b605e1e8eb759aa79e775e89df7c4ae5966efe3b96d3554003
                ],
                column_witness: array![]
            },
            decomposition_coeff: qm31(0, 0, 0, 0),
            commitment: 0x03e5bad5822d062c05ff947d282dc2d56a6a420d14f2f74972bb5b01287731a7
        }],
        last_layer_poly: array![
            qm31(1700411592, 1718151617, 1386964511, 2008082344), 
            qm31(1700411592, 1718151617, 1386964511, 2008082344)
        ],
        last_layer_poly_log_size: 1
    };

    let channel = ChannelTrait::new(0x00);
    let verifier = FriVerifier {log_blowup_factor: 2, log_degree_bound: 3};
    assert!(verifier.verify(channel, proof));
}
