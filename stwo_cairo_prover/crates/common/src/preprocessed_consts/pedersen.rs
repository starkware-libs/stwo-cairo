pub const BITS_PER_WINDOW: usize = 18;
pub const NUM_WINDOWS: usize = 252usize.div_ceil(BITS_PER_WINDOW);
pub const PEDERSEN_TABLE_N_ROWS: usize =
    16 + NUM_WINDOWS * (1 << BITS_PER_WINDOW) + 16 + NUM_WINDOWS * (1 << BITS_PER_WINDOW);
