
            use stwo_cairo_verifier::verifier::StarkProof;

            pub fn proof() -> StarkProof { 
                Serde::deserialize(ref proof_data).unwrap()
            }