use std::collections::HashMap;

use rand::{
    distributions::{Distribution, Uniform, uniform::SampleUniform},
    rngs::ThreadRng,
};
use sysinfo::{CpuExt, System, SystemExt};

#[allow(dead_code)]
pub fn init_constant_vec<T>(len: usize, init_value: T, step: f64) -> Vec<(f64, T)> where T: Copy{
    let init_signal = ConstantSignal::<T>::new(init_value).by_ref().take(len).collect::<Vec<T>>();
    let mut index: f64 = 0_f64;
    init_signal.iter().map(|x| {index += 1_f64 * step; (index, *x)}).rev().collect::<Vec<(f64, T)>>()
}

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
    constant: T
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

pub trait SystemSignalIterator {
    type Item;
    fn next(&mut self, system: &System) -> Option<Self::Item>;
}

pub struct CpuSignal {
    cpu_count: usize,
    len: usize,
    step: f64,
    pub data: HashMap<String, Vec<(f64, f64)>>
}

impl SystemSignalIterator for CpuSignal {
    type Item = HashMap<String, f64>;
    fn next(&mut self, system: &System) -> Option<Self::Item> {
        let mut v = HashMap::<String, f64>::new();
        for i in 1..self.cpu_count {
            let cpu_usage = system.cpus().get(i - 1).unwrap().cpu_usage() as f64;
            v.insert(format!("CPU_{}", i), cpu_usage);
        }
        Some(v)
    }
}

impl CpuSignal {
    pub fn new(cpu_count: usize, len: usize, step: f64) -> Self {
        let mut data: HashMap<String, Vec<(f64, f64)>> = HashMap::new();
        for i in 1..=cpu_count {
            let init_data = init_constant_vec::<f64>(len, 0_f64, step);
            data.insert(format!("CPU_{}", i), init_data);
        }
        Self { cpu_count, len, step, data}
    }

    pub fn on_tick(&mut self, system: &System) -> () {
        let usages = self.next(system).unwrap();
        for (k, v) in usages{
            if self.data.contains_key(&k) {
                self.data.entry(k).and_modify(|item| {
                    let next_index = item.last().unwrap().0 + self.step;
                    item.remove(0);
                    item.push((next_index, v));
                });
            }
        }
    }
}