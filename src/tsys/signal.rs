use rand::{
    distributions::{Distribution, Uniform, uniform::SampleUniform},
    rngs::ThreadRng,
};
use sysinfo::{CpuExt, System, SystemExt, Cpu};
use crate::utils::constants::app_constants::CPU_SIGNAL_LEN;

#[allow(dead_code)]
pub struct Signal<S: Iterator> 
{
    source: S,
    pub points: Vec<S::Item>,
    tick_rate: usize,
}

impl<S> Signal<S> where S: Iterator,
{
    #[allow(dead_code)]
    fn on_tick(&mut self) {
        for _ in 0..self.tick_rate {
            self.points.remove(0);
        }
        self.points
           .extend(self.source.by_ref().take(self.tick_rate));
    }
}

pub struct RandomSignal<T> where T: SampleUniform{
    distribution: Uniform<T>,
    rng: ThreadRng,
}

impl<T> RandomSignal<T> where T: SampleUniform{
    #[allow(dead_code)]
    pub fn new(lower: T, upper: T) -> Self{
        Self {
            distribution: Uniform::new(lower, upper),
            rng: rand::thread_rng(),
        }
    }
}

impl<T> Iterator for RandomSignal<T> where T: SampleUniform{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        Some(self.distribution.sample(&mut self.rng))
    }
}

#[derive(Clone, Copy)]
pub struct ConstantSignal<T> where T: Copy {
    constant: T,
}

impl<T> ConstantSignal<T> where T: Copy {
    pub fn new(_constant: T) -> Self{
        Self { 
            constant: _constant
         }
    }
}

impl<T> Iterator for ConstantSignal<T> where T: Copy {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.constant)
    }
}
