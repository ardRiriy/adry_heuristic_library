use std::time::Instant;

use rand::{Rng, rng};

pub type TempSchedule = Box<dyn Fn(f64) -> f64>;
pub struct Schedule;
impl Schedule {
    pub fn exponential(start: f64, end: f64) -> TempSchedule {
        Box::new(move |progress| start * (end / start).powf(progress))
    }
    pub fn linear(start: f64, end: f64) -> TempSchedule {
        Box::new(move |progress| start + (end - start) * progress)
    }
}

pub struct SaParams {
    tl: f64,
    schedule: TempSchedule,
}

impl SaParams {
    pub fn new(tl: f64, schedule: TempSchedule) -> Self {
        Self { tl, schedule }
    }
}

struct SaResult {
    pub iterations: u64,
    pub best_improvement: f64,
    pub accept_count: u64,
    pub worse_accept_count: u64,
}

impl SaResult {
    fn print_stats(&self) {
        eprintln!("SA iterations: {}", self.iterations);
        eprintln!(
            "accept rate: {:.1}%",
            self.accept_count as f64 / self.iterations as f64 * 100.0
        );
        eprintln!(
            "worse accept rate: {:.1}%",
            self.worse_accept_count as f64 / self.iterations as f64 * 100.0
        );
    }
}

pub trait SaState {
    type Neighbor;
    type Undo;

    fn gen_neighbor(&self, rng: &mut impl Rng) -> Self::Neighbor;
    fn apply(&mut self, neighbor: &Self::Neighbor) -> (f64, Self::Undo);
    fn undo(&mut self, undo_info: Self::Undo);
}

pub fn sa_solve<S: SaState>(state: &mut S, params: &SaParams) -> SaResult {
    let timer = Instant::now();
    let mut rng = rng();

    let mut current_score = 0.0_f64;
    let mut best_improvement = 0.0_f64;

    let mut iter_count: u64 = 0;
    let mut accept_count: u64 = 0;
    let mut worse_accept_count: u64 = 0;

    // let mut iter_count: u64 = 0;
    loop {
        if iter_count % 256 == 0 {
            if timer.elapsed().as_secs_f64() >= params.tl {
                break;
            }
        }
        iter_count += 1;

        let progress = timer.elapsed().as_secs_f64() / params.tl;
        let temp = (params.schedule)(progress);

        let neighbor = state.gen_neighbor(&mut rng);
        let (diff, undo_info) = state.apply(&neighbor);

        if diff >= 0.0 || rng.random::<f64>() < (diff / temp).exp() {
            current_score += diff;
            accept_count += 1;
            if diff < 0.0 {
                worse_accept_count += 1;
            }
            if current_score > best_improvement {
                best_improvement = current_score;
            }
        } else {
            state.undo(undo_info);
        }
    }

    eprintln!("SA iterations: {}", iter_count);
    eprintln!(
        "accept rate: {:.3}%",
        accept_count as f64 / iter_count as f64 * 100.0
    );
    eprintln!(
        "worse accept rate: {:.3}%",
        worse_accept_count as f64 / iter_count as f64 * 100.0
    );

    SaResult {
        iterations: iter_count,
        best_improvement,
        accept_count,
        worse_accept_count,
    }
}
