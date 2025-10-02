use quick_xml::events::Event;
use quick_xml::Reader;
use thiserror::Error;
use wasm_bindgen::prelude::*;

use crate::models::{BioModelData, Species, Reaction, Parameter, Compartment};

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("XML parsing error: {0}")]
    XmlError(String),
    #[error("Invalid SBML structure: {0}")]
    InvalidStructure(String),
}

pub fn parse_sbml(content: &str) -> Result<BioModelData, ParserError> {
    let mut reader = Reader::from_str(content);
    
    let mut model_data = BioModelData::new();
    let mut current_section = String::new();
    let mut buf = Vec::new();
    
    // Debug: Log content length
    web_sys::console::log_1(&format!("Parsing SBML content of length: {}", content.len()).into());
    
    // Let's also log the first part of the content to see what we're actually parsing
    let preview = if content.len() > 500 { &content[0..500] } else { content };
    web_sys::console::log_1(&format!("Content preview: {}", preview).into());
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                let name = String::from_utf8(e.name().as_ref().to_vec())
                    .map_err(|e| ParserError::XmlError(e.to_string()))?;
                
                // Debug: log all tags we encounter
                web_sys::console::log_1(&format!("Start/Empty Tag: '{}', Section: '{}'", name, current_section).into());
                
                match name.as_str() {
                    "listOfCompartments" => {
                        current_section = "compartments".to_string();
                        web_sys::console::log_1(&"Setting section to compartments".into());
                    }
                    "listOfSpecies" => {
                        current_section = "species".to_string();
                        web_sys::console::log_1(&"Setting section to species".into());
                    }
                    "listOfParameters" => {
                        current_section = "parameters".to_string();
                        web_sys::console::log_1(&"Setting section to parameters".into());
                    }
                    "listOfReactions" => {
                        current_section = "reactions".to_string();
                        web_sys::console::log_1(&"Setting section to reactions".into());
                    }
                    "compartment" if current_section == "compartments" => {
                        web_sys::console::log_1(&"Parsing compartment".into());
                        if let Some(comp) = parse_compartment(&e) {
                            model_data.compartments.push(comp);
                        }
                    }
                    "species" if current_section == "species" => {
                        web_sys::console::log_1(&"Parsing species".into());
                        if let Some(spec) = parse_species(&e) {
                            web_sys::console::log_1(&format!("Found species: {} ({})", spec.name, spec.id).into());
                            model_data.species.push(spec);
                        }
                    }
                    "parameter" if current_section == "parameters" => {
                        web_sys::console::log_1(&"Parsing parameter".into());
                        if let Some(param) = parse_parameter(&e) {
                            model_data.parameters.push(param);
                        }
                    }
                    "reaction" if current_section == "reactions" => {
                        web_sys::console::log_1(&"Parsing reaction".into());
                        if let Some(reaction) = parse_reaction(&e, &mut reader)? {
                            model_data.reactions.push(reaction);
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(e)) => {
                let name = String::from_utf8(e.name().as_ref().to_vec())
                    .map_err(|e| ParserError::XmlError(e.to_string()))?;
                
                if name.starts_with("listOf") {
                    web_sys::console::log_1(&format!("Clearing section on end of: {}", name).into());
                    current_section.clear();
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(ParserError::XmlError(e.to_string())),
            _ => {}
        }
        buf.clear();
    }
    
    web_sys::console::log_1(&format!("Parsing complete: {} species, {} parameters, {} reactions", 
        model_data.species.len(), model_data.parameters.len(), model_data.reactions.len()).into());
    
    Ok(model_data)
}

fn parse_compartment(e: &quick_xml::events::BytesStart) -> Option<Compartment> {
    let mut id = String::new();
    let mut name = String::new();
    
    for attr in e.attributes() {
        if let Ok(attr) = attr {
            let key = std::str::from_utf8(attr.key.as_ref()).ok()?;
            let value = attr.unescape_value().ok()?;
            
            match key {
                "id" => id = value.to_string(),
                "name" => name = value.to_string(),
                _ => {}
            }
        }
    }
    
    if !id.is_empty() {
        Some(Compartment { id, name })
    } else {
        None
    }
}

fn parse_species(e: &quick_xml::events::BytesStart) -> Option<Species> {
    let mut id = String::new();
    let mut name = String::new();
    let mut compartment = String::new();
    let mut initial_amount = 0.0;
    
    web_sys::console::log_1(&format!("Parsing species with {} attributes", e.attributes().count()).into());
    
    for attr in e.attributes() {
        if let Ok(attr) = attr {
            let key = std::str::from_utf8(attr.key.as_ref()).ok()?;
            let value = attr.unescape_value().ok()?;
            
            web_sys::console::log_1(&format!("  Species attr: {} = {}", key, value).into());
            
            match key {
                "id" => id = value.to_string(),
                "name" => name = value.to_string(),
                "compartment" => compartment = value.to_string(),
                "initialAmount" => initial_amount = value.parse().unwrap_or(0.0),
                "initialConcentration" => initial_amount = value.parse().unwrap_or(0.0),
                _ => {}
            }
        }
    }
    
    if !id.is_empty() {
        let final_name = if name.is_empty() { id.clone() } else { name };
        Some(Species {
            id,
            name: final_name,
            compartment,
            initial_concentration: initial_amount,
        })
    } else {
        None
    }
}

fn parse_parameter(e: &quick_xml::events::BytesStart) -> Option<Parameter> {
    let mut id = String::new();
    let mut value = 0.0;
    let mut constant = true;
    
    for attr in e.attributes() {
        if let Ok(attr) = attr {
            let key = std::str::from_utf8(attr.key.as_ref()).ok()?;
            let attr_value = attr.unescape_value().ok()?;
            
            match key {
                "id" => id = attr_value.to_string(),
                "value" => value = attr_value.parse().unwrap_or(0.0),
                "constant" => constant = attr_value == "true",
                _ => {}
            }
        }
    }
    
    if !id.is_empty() {
        Some(Parameter { id, value, constant })
    } else {
        None
    }
}

fn parse_reaction(
    e: &quick_xml::events::BytesStart,
    reader: &mut Reader<&[u8]>,
) -> Result<Option<Reaction>, ParserError> {
    let mut id = String::new();
    let mut name = String::new();
    
    for attr in e.attributes() {
        if let Ok(attr) = attr {
            let key = std::str::from_utf8(attr.key.as_ref())
                .map_err(|e| ParserError::XmlError(e.to_string()))?;
            let value = attr.unescape_value()
                .map_err(|e| ParserError::XmlError(e.to_string()))?;
            
            match key {
                "id" => id = value.to_string(),
                "name" => name = value.to_string(),
                _ => {}
            }
        }
    }
    
    let mut reactants = Vec::new();
    let mut products = Vec::new();
    let mut kinetic_law = String::new();
    let mut in_reactants = false;
    let mut in_products = false;
    let mut in_kinetic_law = false;
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let tag_name = String::from_utf8(e.name().as_ref().to_vec())
                    .map_err(|e| ParserError::XmlError(e.to_string()))?;
                
                match tag_name.as_str() {
                    "listOfReactants" => in_reactants = true,
                    "listOfProducts" => in_products = true,
                    "kineticLaw" => in_kinetic_law = true,
                    "speciesReference" => {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                let key = std::str::from_utf8(attr.key.as_ref())
                                    .map_err(|e| ParserError::XmlError(e.to_string()))?;
                                
                                if key == "species" {
                                    let species_id = attr.unescape_value()
                                        .map_err(|e| ParserError::XmlError(e.to_string()))?
                                        .to_string();
                                    
                                    if in_reactants {
                                        reactants.push(species_id);
                                    } else if in_products {
                                        products.push(species_id);
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(e)) => {
                let tag_name = String::from_utf8(e.name().as_ref().to_vec())
                    .map_err(|e| ParserError::XmlError(e.to_string()))?;
                
                match tag_name.as_str() {
                    "listOfReactants" => in_reactants = false,
                    "listOfProducts" => in_products = false,
                    "kineticLaw" => in_kinetic_law = false,
                    "reaction" => break,
                    _ => {}
                }
            }
            Ok(Event::Text(e)) if in_kinetic_law => {
                kinetic_law.push_str(&e.unescape()
                    .map_err(|e| ParserError::XmlError(e.to_string()))?);
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(ParserError::XmlError(e.to_string())),
            _ => {}
        }
        buf.clear();
    }
    
    if !id.is_empty() {
        let final_name = if name.is_empty() { id.clone() } else { name };
        Ok(Some(Reaction {
            id,
            name: final_name,
            reactants,
            products,
            rate_constant: 0.1,
            kinetic_law,
        }))
    } else {
        Ok(None)
    }
}