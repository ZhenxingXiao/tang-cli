use std::collections::HashMap;

use rand::{
    distributions::{Distribution, Uniform, uniform::SampleUniform},
    rngs::ThreadRng,
};
use sysinfo::{CpuExt, System, SystemExt, DiskExt};

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
    // len: usize,
    time_step: f64,
    pub time_range: [f64; 2],
    pub cpu_current_usage: HashMap<String, f64>,
    pub cpu_total_usage: Vec<(f64, f64)>
}

impl CpuSignal {
    pub fn new(cpu_count: usize, len: usize, time_step: f64) -> Self {
        let mut cpu_current_usage: HashMap<String, f64> = HashMap::new();
        for i in 1..=cpu_count {
            cpu_current_usage.insert(format!("CPU_{}", i), 0_f64);
        }
        let cpu_total_usage = init_constant_vec::<f64>(len, 0_f64, time_step);
        let time_range = [0_f64, len as f64 * time_step];
        Self { 
            cpu_count, 
            // len, 
            time_step, 
            time_range,
            cpu_current_usage,
            cpu_total_usage
        }
    }

    pub fn on_tick(&mut self, system: &System) -> () {
        let mut total_usage = 0_f64;
        for i in 1..self.cpu_count {
            let cpu_usage = system.cpus().get(i - 1).unwrap().cpu_usage() as f64;
            total_usage += cpu_usage;
            self.cpu_current_usage.insert(format!("CPU_{}", i), cpu_usage);
        }
        total_usage = total_usage / self.cpu_count as f64;
        self.cpu_total_usage.remove(0);
        self.cpu_total_usage.push((self.cpu_total_usage.last().unwrap().0 + self.time_step, total_usage));
        self.time_range = [self.cpu_total_usage.first().unwrap().0, self.cpu_total_usage.last().unwrap().0];
    }
}

pub struct MemorySignal{
    time_step: f64,
    pub used_memory: u64,
    pub total_memory: u64,
    pub free_memory: u64,
    pub time_range: [f64; 2],
    pub mem_total_usage: Vec<(f64, f64)>
}

impl MemorySignal {
    pub fn new(len: usize, time_step: f64) -> Self {
        let mem_total_usage = init_constant_vec::<f64>(len, 0_f64, time_step);
        let time_range = [0_f64, len as f64 * time_step];
        let used_memory = 0_u64;
        let total_memory = 1_u64;
        let free_memory = 0_u64;
        Self {
            time_step, 
            used_memory,
            total_memory,
            free_memory,
            time_range,
            mem_total_usage
        }
    }

    pub fn on_tick(&mut self, system: &System) -> () {
        self.mem_total_usage.remove(0);
        self.free_memory = system.free_memory();
        self.used_memory = system.used_memory();
        self.total_memory = system.total_memory();
        self.mem_total_usage.push((self.mem_total_usage.last().unwrap().0 + self.time_step, self.used_memory as f64 / self.total_memory as f64 * 100_f64));
        self.time_range = [self.mem_total_usage.first().unwrap().0, self.mem_total_usage.last().unwrap().0];
    }
}

pub struct DiskSignal {
    disk_count: usize,
    time_step: f64,
}

impl DiskSignal {
    pub fn new(disk_count: usize, len: usize, time_step: f64) -> Self {
        Self {
            disk_count,
            time_step
        }
    }

    pub fn on_tick(&mut self, system: &System) -> () {
        for disk in system.disks() {
            disk.name();
        }
    }
}