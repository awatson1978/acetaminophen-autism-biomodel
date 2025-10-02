use wasm_bindgen::prelude::*;
use std::collections::HashMap;

pub mod parser;
pub mod simulator;
pub mod models;
pub mod utils;

use models::BioModelData;
use simulator::Simulator;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct BioModel {
    model_data: BioModelData,
    simulator: Simulator,
}

#[wasm_bindgen]
impl BioModel {
    #[wasm_bindgen(constructor)]
    pub fn new(sbml_content: &str) -> Result<BioModel, JsValue> {
        console_log!("Parsing SBML model...");
        
        match parser::parse_sbml(sbml_content) {
            Ok(model_data) => {
                console_log!("Model loaded: {} species, {} reactions", 
                    model_data.species.len(), 
                    model_data.reactions.len()
                );
                
                let simulator = Simulator::new(&model_data);
                
                Ok(BioModel {
                    model_data,
                    simulator,
                })
            }
            Err(e) => Err(JsValue::from_str(&format!("Failed to parse SBML: {}", e)))
        }
    }
    
    #[wasm_bindgen(js_name = simulate)]
    pub fn simulate(&mut self, config: JsValue) -> Result<JsValue, JsValue> {
        let config: SimulationConfig = serde_wasm_bindgen::from_value(config)
            .map_err(|e| JsValue::from_str(&format!("Invalid config: {}", e)))?;
        
        console_log!("Running simulation from t=0 to t={} with step {}", 
            config.time_end, config.time_step);
        
        let num_steps = (config.time_end / config.time_step) as usize;
        console_log!("This will generate {} time points", num_steps + 1);
        
        // Warn if too many points
        if num_steps > 10000 {
            console_log!("Warning: Large simulation with {} steps may be slow", num_steps);
        }
        
        let results = self.simulator.simulate(
            config.time_end,
            config.time_step,
            &config.method
        )?;
        
        serde_wasm_bindgen::to_value(&results)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize results: {}", e)))
    }
    
    #[wasm_bindgen(js_name = getSpeciesNames)]
    pub fn get_species_names(&self) -> Vec<String> {
        self.model_data.species.iter()
            .map(|s| s.name.clone())
            .collect()
    }
    
    #[wasm_bindgen(js_name = getSpeciesIds)]
    pub fn get_species_ids(&self) -> Vec<String> {
        self.model_data.species.iter()
            .map(|s| s.id.clone())
            .collect()
    }
    
    #[wasm_bindgen(js_name = getParameters)]
    pub fn get_parameters(&self) -> Result<JsValue, JsValue> {
        let params: HashMap<String, f64> = self.model_data.parameters.iter()
            .map(|p| (p.id.clone(), p.value))
            .collect();
        
        serde_wasm_bindgen::to_value(&params)
            .map_err(|e| JsValue::from_str(&format!("Failed to get parameters: {}", e)))
    }
    
    #[wasm_bindgen(js_name = getInitialConcentrations)]
    pub fn get_initial_concentrations(&self) -> Result<JsValue, JsValue> {
        let concentrations: HashMap<String, f64> = self.model_data.species.iter()
            .map(|s| (s.id.clone(), s.initial_concentration))
            .collect();
        
        serde_wasm_bindgen::to_value(&concentrations)
            .map_err(|e| JsValue::from_str(&format!("Failed to get concentrations: {}", e)))
    }
    
    #[wasm_bindgen(js_name = setInitialConcentration)]
    pub fn set_initial_concentration(&mut self, species_id: &str, value: f64) -> Result<(), JsValue> {
        for species in &mut self.model_data.species {
            if species.id == species_id {
                species.initial_concentration = value;
                self.simulator.update_parameters(&self.model_data);
                return Ok(());
            }
        }
        Err(JsValue::from_str(&format!("Species '{}' not found", species_id)))
    }
    
    #[wasm_bindgen(js_name = setParameter)]
    pub fn set_parameter(&mut self, param_id: &str, value: f64) -> Result<(), JsValue> {
        for param in &mut self.model_data.parameters {
            if param.id == param_id {
                param.value = value;
                self.simulator.update_parameters(&self.model_data);
                return Ok(());
            }
        }
        Err(JsValue::from_str(&format!("Parameter '{}' not found", param_id)))
    }
    
    #[wasm_bindgen(js_name = parameterScan)]
    pub fn parameter_scan(&mut self, param_id: &str, values: Vec<f64>) -> Result<JsValue, JsValue> {
        let original_value = self.model_data.parameters.iter()
            .find(|p| p.id == param_id)
            .ok_or_else(|| JsValue::from_str(&format!("Parameter '{}' not found", param_id)))?
            .value;
        
        let mut scan_results = Vec::new();
        
        for value in values {
            self.set_parameter(param_id, value)?;
            
            let results = self.simulator.simulate(100.0, 0.1, "rk4")?;
            scan_results.push(ScanResult {
                parameter_value: value,
                results,
            });
        }
        
        self.set_parameter(param_id, original_value)?;
        
        serde_wasm_bindgen::to_value(&scan_results)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize scan results: {}", e)))
    }
}

#[derive(serde::Deserialize)]
struct SimulationConfig {
    #[serde(rename = "timeEnd")]
    time_end: f64,
    #[serde(rename = "timeStep")]
    time_step: f64,
    #[serde(default = "default_method")]
    method: String,
}

fn default_method() -> String {
    "rk4".to_string()
}

#[derive(serde::Serialize)]
struct ScanResult {
    parameter_value: f64,
    results: simulator::SimulationResults,
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log!("BioModels WASM module loaded");
}