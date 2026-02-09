use cairo_air::air::CairoComponents;
use itertools::chain;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::ComponentProver;

pub fn cairo_provers(components: &CairoComponents) -> Vec<&dyn ComponentProver<SimdBackend>> {
    let components = components.components();
    components
        .into_iter()
        .map(|component| component as &dyn ComponentProver<SimdBackend>)
        .collect()
}
