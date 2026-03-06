use std::time::Instant;

use rand::{Rng, rng};

pub struct SaParams {
    tl: f64,
    temp_start: f64,
    temp_end: f64,
}

impl SaParams {
    pub fn new(tl: f64, temp_start: f64, temp_end: f64) -> Self {
        Self {
            tl,
            temp_start,
            temp_end,
        }
    }
}

pub trait SaState {
    type Neighbor;
    type Undo;

    fn gen_neighbor(&self, rng: &mut impl Rng) -> Self::Neighbor;
    fn apply(&mut self, neighbor: &Self::Neighbor) -> (f64, Self::Undo);
    fn undo(&mut self, undo_info: Self::Undo);
}

pub fn sa_solve<S: SaState>(state: &mut S, params: &SaParams) -> f64 {
    let timer = Instant::now();
    let mut rng = rng();

    let mut current_score = 0.0_f64;
    let mut best_diff = 0.0_f64; // 初期解からの累積改善量

    let mut iter_count: u64 = 0;

    loop {
        if iter_count % 256 == 0 {
            if timer.elapsed().as_secs_f64() >= params.tl {
                break;
            }
        }
        iter_count += 1;

        let progress = timer.elapsed().as_secs_f64() / params.tl;
        let temp = params.temp_start + (params.temp_end - params.temp_start) * progress;

        let neighbor = state.gen_neighbor(&mut rng);
        let (diff, undo_info) = state.apply(&neighbor);

        if diff >= 0.0 || rng.random::<f64>() < (diff / temp).exp() {
            current_score += diff;
            if current_score > best_diff {
                best_diff = current_score;
            }
        } else {
            state.undo(undo_info);
        }
    }
    eprintln!("SA iterations: {}", iter_count);
    best_diff
}
