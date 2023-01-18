use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
};


pub struct Signal<S: Iterator> 
{
    source: S,
    pub points: Vec<S::Item>,
    tick_rate: usize,
}

impl<S> Signal<S> where S: Iterator,
{
    fn on_tick(&mut self) {
        for _ in 0..self.tick_rate {
            self.points.remove(0);
        }
        self.points
           .extend(self.source.by_ref().take(self.tick_rate));
    }
}

#[derive(Clone)]
pub struct RandomSignal {
    distribution: Uniform<u64>,
    rng: ThreadRng,
}

impl RandomSignal {
    pub fn new(lower: u64, upper: u64) -> RandomSignal {
        RandomSignal {
            distribution: Uniform::new(lower, upper),
            rng: rand::thread_rng(),
        }
    }
}

impl Iterator for RandomSignal {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        Some(self.distribution.sample(&mut self.rng))
    }
}

#[derive(Clone)]
pub struct ConstantSignal {
    constant: u64,
}

impl ConstantSignal {
    pub fn new(_constant: u64) -> ConstantSignal{
        ConstantSignal { 
            constant: _constant
         }
    }
}

impl Iterator for ConstantSignal {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.constant)
    }
}