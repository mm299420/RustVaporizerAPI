use crate::vaporizer::{Heatable, Vaporizer};

pub trait Sequence {

    fn add_step(&mut self, temp: f64, duration: f64);

    fn insert_step(&mut self, index: usize, temp: f64, duration: f64);

    fn remove_step(&mut self, index: usize);

    fn set_step_temp(&mut self, index: usize, temp: f64);

    fn set_step_duration(&mut self, index: usize, duration: f64);

    fn start(&mut self);

    fn update(&mut self, time_delta: f64);

    fn is_finished(&self) -> bool;

    fn get_num_steps(&self) -> usize;

    fn get_step_temp(&self, index: usize) -> f64;

    fn get_step_duration(&self, index: usize) -> f64;

}

pub struct SequencePlan {

    steps: Vec<(f64, f64)>, // Tuple of (temperature, duration) for each step in the sequence

    current_step: usize,

    vaporizer: Vaporizer,

}

impl Sequence for SequencePlan {

    fn add_step(&mut self, temp: f64, duration: f64) {

        self.steps.push((temp, duration));

    }

    fn insert_step(&mut self, index: usize, temp: f64, duration: f64) {

        self.steps.insert(index, (temp, duration));

    }

    fn remove_step(&mut self, index: usize) {

        self.steps.remove(index);

    }

    fn set_step_temp(&mut self, index: usize, temp: f64) {

        self.steps[index].0 = temp;

    }

    fn set_step_duration(&mut self, index: usize, duration: f64) {

        self.steps[index].1 = duration;

    }

    fn start(&mut self) {

        self.current_step = 0;

        self.vaporizer.heat_to_temp(self.steps[self.current_step].0);

    }

    fn update(&mut self, time_delta: f64) {

        self.vaporizer.update_temp(time_delta);

        if !self.vaporizer.is_heating() && self.current_step < self.steps.len() - 1 {

            self.current_step += 1;

            let (temp, duration) = self.steps[self.current_step];

            self.vaporizer.heat_to_temp(temp);

        }

    }

    fn is_finished(&self) -> bool {

        !self.vaporizer.is_heating() && self.current_step == self.steps.len() - 1

    }

    fn get_num_steps(&self) -> usize {

        self.steps.len()

    }

    fn get_step_temp(&self, index: usize) -> f64 {

        self.steps[index].0

    }

    fn get_step_duration(&self, index: usize) -> f64 {

        self.steps[index].1

    }

}

impl SequencePlan {

    pub fn new(vaporizer: Vaporizer) -> Self {

        SequencePlan {

            steps: Vec::new(),

            current_step: 0,

            vaporizer,

        }

    }

}

 
