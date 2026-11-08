use eval::static_evaluation::MATE_VALUE;

pub mod bot;
pub mod random_bot;
pub mod min_max;
pub mod mc_bot;
pub mod alfa_beta;

const INF: i32 = MATE_VALUE + 1;