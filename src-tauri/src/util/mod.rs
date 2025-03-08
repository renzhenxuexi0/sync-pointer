use rand::RngCore as _;

pub fn generate_id() -> u32 {
    rand::rng().next_u32()
}
