pub const BITS_PER_WINDOW: usize = 18;
pub const NUM_WINDOWS: usize = 252usize.div_ceil(BITS_PER_WINDOW);
pub const ROWS_PER_WINDOW: usize = 1 << BITS_PER_WINDOW;

pub const P0_SECTION_START: usize = 0;
pub const P1_SECTION_START: usize = P0_SECTION_START + NUM_WINDOWS * ROWS_PER_WINDOW;
pub const P2_SECTION_START: usize = P1_SECTION_START + 16;
pub const P3_SECTION_START: usize = P2_SECTION_START + NUM_WINDOWS * ROWS_PER_WINDOW;
pub const PEDERSEN_TABLE_N_ROWS: usize = P3_SECTION_START + 16;
