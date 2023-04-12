pub trait Heatable {
    fn heat_to_temp(&mut self, temp: f64);
    fn stop_heating(&mut self);
    fn is_heating(&self) -> bool;
    fn set_heating_rate(&mut self, rate: f64);
    fn get_current_temp(&self) -> f64;
    fn get_target_temp(&self) -> f64;
    fn update_temp(&mut self, time_delta: f64);
}

pub struct Vaporizer {
    current_temp: f64,
    target_temp: f64,
    heating_rate: f64,
    is_heating: bool,
}

impl Heatable for Vaporizer {
    // Function to start heating the vaporizer to a specific temperature
    fn heat_to_temp(&mut self, temp: f64) {
        self.target_temp = temp;
        self.is_heating = true;
    }

    // Function to stop the vaporizer from heating
    fn stop_heating(&mut self) {
        self.is_heating = false;
    }

    // Function to check if the vaporizer is currently heating
    fn is_heating(&self) -> bool {
        self.is_heating
    }

    // Function to set the desired heating rate for the vaporizer
    fn set_heating_rate(&mut self, rate: f64) {
        self.heating_rate = rate;
    }

    // Function to get the current temperature of the vaporizer
    fn get_current_temp(&self) -> f64 {
        self.current_temp
    }

    // Function to get the target temperature that the vaporizer is currently heating towards
    fn get_target_temp(&self) -> f64 {
        self.target_temp
    }

    // Function to update the current temperature of the vaporizer based on the heating rate
    fn update_temp(&mut self, time_delta: f64) {
        if self.is_heating {
            if self.current_temp < self.target_temp {
                self.current_temp += self.heating_rate * time_delta;
            } else {
                self.is_heating = false;
            }
        }
    }
}
