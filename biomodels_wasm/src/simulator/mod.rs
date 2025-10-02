use nalgebra::{DVector, DMatrix};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use crate::models::BioModelData;
use web_sys;

pub struct Simulator {
    state: DVector<f64>,
    stoichiometry_matrix: DMatrix<f64>,
    rate_constants: Vec<f64>,
    model_ref: BioModelData,
}

impl Simulator {
    pub fn new(model: &BioModelData) -> Self {
        let n_species = model.species.len();
        let n_reactions = model.reactions.len();
        
        let mut state = DVector::zeros(n_species);
        for (i, species) in model.species.iter().enumerate() {
            state[i] = species.initial_concentration;
        }
        
        let mut stoichiometry = DMatrix::zeros(n_species, n_reactions);
        
        for (j, reaction) in model.reactions.iter().enumerate() {
            for reactant_id in &reaction.reactants {
                if let Some(i) = model.get_species_index(reactant_id) {
                    stoichiometry[(i, j)] -= 1.0;
                }
            }
            
            for product_id in &reaction.products {
                if let Some(i) = model.get_species_index(product_id) {
                    stoichiometry[(i, j)] += 1.0;
                }
            }
        }
        
        let rate_constants = model.reactions.iter()
            .map(|r| r.rate_constant)
            .collect();
        
        Simulator {
            state,
            stoichiometry_matrix: stoichiometry,
            rate_constants,
            model_ref: model.clone(),
        }
    }
    
    pub fn update_parameters(&mut self, model: &BioModelData) {
        self.model_ref = model.clone();
        self.rate_constants = model.reactions.iter()
            .map(|r| r.rate_constant)
            .collect();
        // Also update initial state from the model
        for (i, species) in model.species.iter().enumerate() {
            self.state[i] = species.initial_concentration;
        }
    }
    
    pub fn simulate(&mut self, time_end: f64, time_step: f64, method: &str) -> Result<SimulationResults, JsValue> {
        let num_steps = (time_end / time_step) as usize;
        let mut time_points = Vec::with_capacity(num_steps + 1);
        let mut values = Vec::with_capacity((num_steps + 1) * self.state.len());
        
        self.reset_state();
        
        time_points.push(0.0);
        values.extend_from_slice(self.state.as_slice());
        
        let mut t = 0.0;
        
        for _ in 0..num_steps {
            match method {
                "euler" => self.euler_step(time_step),
                "rk4" => self.runge_kutta4_step(time_step),
                _ => self.runge_kutta4_step(time_step),
            }
            
            t += time_step;
            time_points.push(t);
            values.extend_from_slice(self.state.as_slice());
            
            for i in 0..self.state.len() {
                if self.state[i] < 0.0 {
                    self.state[i] = 0.0;
                }
            }
        }
        
        web_sys::console::log_1(&format!("Simulation complete: {} time points generated", time_points.len()).into());
        
        Ok(SimulationResults {
            time: time_points,
            values,
            species_names: self.model_ref.species.iter().map(|s| s.name.clone()).collect(),
            num_species: self.state.len(),
        })
    }
    
    fn reset_state(&mut self) {
        for (i, species) in self.model_ref.species.iter().enumerate() {
            self.state[i] = species.initial_concentration;
        }
    }
    
    fn compute_reaction_rates(&self, state: &DVector<f64>) -> DVector<f64> {
        let n_reactions = self.model_ref.reactions.len();
        let mut rates = DVector::zeros(n_reactions);
        
        for (j, reaction) in self.model_ref.reactions.iter().enumerate() {
            let mut rate = self.rate_constants[j];
            
            for reactant_id in &reaction.reactants {
                if let Some(i) = self.model_ref.get_species_index(reactant_id) {
                    rate *= state[i].max(0.0);
                }
            }
            
            rates[j] = rate;
        }
        
        rates
    }
    
    fn compute_derivatives(&self, state: &DVector<f64>) -> DVector<f64> {
        let reaction_rates = self.compute_reaction_rates(state);
        &self.stoichiometry_matrix * reaction_rates
    }
    
    fn euler_step(&mut self, dt: f64) {
        let derivatives = self.compute_derivatives(&self.state);
        self.state += derivatives * dt;
    }
    
    fn runge_kutta4_step(&mut self, dt: f64) {
        let k1 = self.compute_derivatives(&self.state);
        let k2 = self.compute_derivatives(&(&self.state + &k1 * (dt / 2.0)));
        let k3 = self.compute_derivatives(&(&self.state + &k2 * (dt / 2.0)));
        let k4 = self.compute_derivatives(&(&self.state + &k3 * dt));
        
        self.state += (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (dt / 6.0);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResults {
    pub time: Vec<f64>,
    pub values: Vec<f64>,
    pub species_names: Vec<String>,
    pub num_species: usize,
}

impl SimulationResults {
    pub fn get_species_trajectory(&self, species_index: usize) -> Vec<f64> {
        let mut trajectory = Vec::new();
        for i in 0..self.time.len() {
            trajectory.push(self.values[i * self.num_species + species_index]);
        }
        trajectory
    }
}