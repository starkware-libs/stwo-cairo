use stwo::core::air::Component;
use stwo_constraint_framework::TraceLocationAllocator;

use crate::components::{
    blake_g, blake_round, blake_round_sigma, triple_xor_32, verify_bitwise_xor_12,
};
use crate::relations::CommonLookupElements;

pub struct BlakeContextComponents {
    pub components: Option<Components>,
}
impl BlakeContextComponents {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        blake_round_claim: &Option<blake_round::Claim>,
        blake_g_claim: &Option<blake_g::Claim>,
        blake_sigma_claim: &Option<blake_round_sigma::Claim>,
        triple_xor_32_claim: &Option<triple_xor_32::Claim>,
        verify_bitwise_xor_12_claim: &Option<verify_bitwise_xor_12::Claim>,
        common_lookup_elements: &CommonLookupElements,
        blake_round_interaction_claim: &Option<blake_round::InteractionClaim>,
        blake_g_interaction_claim: &Option<blake_g::InteractionClaim>,
        blake_sigma_interaction_claim: &Option<blake_round_sigma::InteractionClaim>,
        triple_xor_32_interaction_claim: &Option<triple_xor_32::InteractionClaim>,
        verify_bitwise_xor_12_interaction_claim: &Option<verify_bitwise_xor_12::InteractionClaim>,
    ) -> Self {
        let components = (*blake_round_interaction_claim).map(|_| {
            Components::new(
                tree_span_provider,
                blake_round_claim,
                blake_g_claim,
                blake_sigma_claim,
                triple_xor_32_claim,
                verify_bitwise_xor_12_claim,
                common_lookup_elements,
                blake_round_interaction_claim,
                blake_g_interaction_claim,
                blake_sigma_interaction_claim,
                triple_xor_32_interaction_claim,
                verify_bitwise_xor_12_interaction_claim,
            )
        });
        Self { components }
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        self.components
            .as_ref()
            .map(|c| c.components())
            .unwrap_or_default()
    }
}

impl std::fmt::Display for BlakeContextComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.components {
            Some(components) => write!(f, "{components}"),
            None => write!(f, "No Blake Context Components"),
        }
    }
}

pub struct Components {
    pub blake_round: blake_round::Component,
    pub blake_g: blake_g::Component,
    pub blake_sigma: blake_round_sigma::Component,
    pub triple_xor_32: triple_xor_32::Component,
    pub verify_bitwise_xor_12: verify_bitwise_xor_12::Component,
}
impl Components {
    #[allow(clippy::too_many_arguments)]
    fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        blake_round_claim: &Option<blake_round::Claim>,
        blake_g_claim: &Option<blake_g::Claim>,
        blake_sigma_claim: &Option<blake_round_sigma::Claim>,
        triple_xor_32_claim: &Option<triple_xor_32::Claim>,
        verify_bitwise_xor_12_claim: &Option<verify_bitwise_xor_12::Claim>,
        common_lookup_elements: &CommonLookupElements,
        blake_round_interaction_claim: &Option<blake_round::InteractionClaim>,
        blake_g_interaction_claim: &Option<blake_g::InteractionClaim>,
        blake_sigma_interaction_claim: &Option<blake_round_sigma::InteractionClaim>,
        triple_xor_32_interaction_claim: &Option<triple_xor_32::InteractionClaim>,
        verify_bitwise_xor_12_interaction_claim: &Option<verify_bitwise_xor_12::InteractionClaim>,
    ) -> Self {
        let blake_round_component = blake_round::Component::new(
            tree_span_provider,
            blake_round::Eval {
                claim: (*blake_round_claim).unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            blake_round_interaction_claim.unwrap().claimed_sum,
        );

        let blake_g_component = blake_g::Component::new(
            tree_span_provider,
            blake_g::Eval {
                claim: (*blake_g_claim).unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            blake_g_interaction_claim.unwrap().claimed_sum,
        );

        let blake_sigma_component = blake_round_sigma::Component::new(
            tree_span_provider,
            blake_round_sigma::Eval {
                claim: (*blake_sigma_claim).unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            blake_sigma_interaction_claim.unwrap().claimed_sum,
        );

        let triple_xor_32_component = triple_xor_32::Component::new(
            tree_span_provider,
            triple_xor_32::Eval {
                claim: (*triple_xor_32_claim).unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            triple_xor_32_interaction_claim.unwrap().claimed_sum,
        );
        let verify_bitwise_xor_12_component = verify_bitwise_xor_12::Component::new(
            tree_span_provider,
            verify_bitwise_xor_12::Eval {
                claim: (*verify_bitwise_xor_12_claim).unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            verify_bitwise_xor_12_interaction_claim.unwrap().claimed_sum,
        );
        Self {
            blake_round: blake_round_component,
            blake_g: blake_g_component,
            blake_sigma: blake_sigma_component,
            triple_xor_32: triple_xor_32_component,
            verify_bitwise_xor_12: verify_bitwise_xor_12_component,
        }
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        vec![
            &self.blake_round,
            &self.blake_g,
            &self.blake_sigma,
            &self.triple_xor_32,
            &self.verify_bitwise_xor_12,
        ]
    }
}

impl std::fmt::Display for Components {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "BlakeRound: {}", self.blake_round)?;
        writeln!(f, "BlakeG: {}", self.blake_g)?;
        writeln!(f, "BlakeSigma: {}", self.blake_sigma)?;
        writeln!(f, "TripleXor32: {}", self.triple_xor_32)?;
        writeln!(f, "VerifyBitwiseXor12: {}", self.verify_bitwise_xor_12)?;
        Ok(())
    }
}
