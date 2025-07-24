#[executable]
fn main() {
    // Force canonical
    core::internal::require_implicit::<core::pedersen::Pedersen>();
    let a = 2;
    let b = 2;
    let c = a * b;
    assert(c == 4, '2x2=4');
}
