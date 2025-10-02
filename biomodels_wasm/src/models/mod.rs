use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioModelData {
    pub compartments: Vec<Compartment>,
    pub species: Vec<Species>,
    pub reactions: Vec<Reaction>,
    pub parameters: Vec<Parameter>,
}

impl BioModelData {
    pub fn new() -> Self {
        BioModelData {
            compartments: Vec::new(),
            species: Vec::new(),
            reactions: Vec::new(),
            parameters: Vec::new(),
        }
    }
    
    pub fn get_species_index(&self, species_id: &str) -> Option<usize> {
        self.species.iter().position(|s| s.id == species_id)
    }
    
    pub fn get_parameter_value(&self, param_id: &str) -> f64 {
        self.parameters.iter()
            .find(|p| p.id == param_id)
            .map(|p| p.value)
            .unwrap_or(1.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compartment {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Species {
    pub id: String,
    pub name: String,
    pub compartment: String,
    pub initial_concentration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub id: String,
    pub name: String,
    pub reactants: Vec<String>,
    pub products: Vec<String>,
    pub rate_constant: f64,
    pub kinetic_law: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub id: String,
    pub value: f64,
    pub constant: bool,
}