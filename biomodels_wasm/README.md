# BioModels WASM

A WebAssembly-based SBML BioModel simulator written in Rust. This library enables running systems biology simulations directly in web browsers and Node.js environments.

## Features

- Parse SBML (Systems Biology Markup Language) XML files
- Run ODE simulations using Euler or Runge-Kutta 4th order methods
- Support for multiple compartments, species, reactions, and parameters
- Parameter scanning capabilities
- Works in both browser and Node.js environments

## Quick Start

### Installation

```bash
# Install Rust and wasm-pack (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack

# Build the project
npm run build  # For web target
npm run build:node  # For Node.js target
```

### Usage in Browser

```javascript
import init, { BioModel } from './pkg/biomodels_wasm.js';

async function runSimulation() {
    // Initialize WASM module
    await init();
    
    // Load SBML model
    const sbmlContent = await fetch('model.xml').then(r => r.text());
    
    // Create model and run simulation
    const model = new BioModel(sbmlContent);
    const results = model.simulate({
        timeEnd: 100.0,
        timeStep: 0.1,
        method: 'rk4'  // or 'euler'
    });
    
    // Access results
    console.log('Species:', model.getSpeciesNames());
    console.log('Time points:', results.time);
    console.log('Concentrations:', results.values);
    
    // Clean up
    model.free();
}
```

### Usage in Node.js

```javascript
const fs = require('fs');
const { BioModel } = require('./pkg-node/biomodels_wasm.js');

// Load SBML model
const sbmlContent = fs.readFileSync('model.xml', 'utf8');

// Create and simulate
const model = new BioModel(sbmlContent);
const results = model.simulate({
    timeEnd: 100.0,
    timeStep: 0.1,
    method: 'rk4'
});

// Access results
console.log('Results:', results);

// Clean up
model.free();
```

## Examples

### Web Example
To run the web example:
```bash
npm run example:web
# Open browser at http://localhost:8000
```

### Node.js Example
To run the Node.js example:
```bash
npm run example:node
```

## API Reference

### `BioModel`

#### Constructor
```javascript
new BioModel(sbmlContent: string)
```
Creates a new BioModel instance from SBML XML content.

#### Methods

##### `simulate(config)`
Runs a simulation with the specified configuration.
- `config.timeEnd`: End time for simulation
- `config.timeStep`: Time step size
- `config.method`: Integration method ('euler' or 'rk4')

Returns an object with:
- `time`: Array of time points
- `values`: Flattened array of species concentrations
- `species_names`: Array of species names
- `num_species`: Number of species

##### `getSpeciesNames()`
Returns an array of species names.

##### `getSpeciesIds()`
Returns an array of species IDs.

##### `getParameters()`
Returns an object with parameter IDs as keys and values as values.

##### `setParameter(paramId, value)`
Sets a parameter value.

##### `parameterScan(paramId, values)`
Performs a parameter scan, running simulations for each parameter value.

##### `free()`
Frees the WASM memory. Should be called when done with the model.

## Development

### Building from Source

```bash
# Clone the repository
git clone [repository-url]
cd biomodels_wasm

# Install dependencies
npm install

# Build WASM modules
npm run build        # Web target
npm run build:node   # Node.js target
npm run build:bundler # Bundler target

# Run tests
npm test
```

### Project Structure

```
biomodels_wasm/
├── src/              # Rust source code
│   ├── lib.rs       # Main library entry
│   ├── parser/      # SBML parsing
│   ├── simulator/   # ODE simulation engine
│   ├── models/      # Data structures
│   └── utils/       # Utilities
├── examples/        # Example usage
│   ├── web/        # Browser example
│   └── node/       # Node.js example
├── pkg/            # Generated WASM package (web)
├── pkg-node/       # Generated WASM package (Node.js)
└── Cargo.toml      # Rust dependencies
```

## Performance

The WASM implementation provides near-native performance for numerical simulations. Key optimizations include:
- Efficient matrix operations using nalgebra
- Optimized Runge-Kutta 4th order solver
- Zero-copy data transfer where possible

## Current Limitations

- Supports basic SBML Level 3 models with species, parameters, and reactions
- Kinetic laws are parsed but only simple expressions are evaluated (mass action kinetics)
- No support for SBML rules (assignment rules, rate rules, algebraic rules)
- No support for SBML events or constraints
- No support for time-series input data
- Limited to deterministic ODE simulations (no stochastic support)
- No support for complex mathematical functions in kinetic laws

## Next Steps

### Priority 1: SBML Rules Support
**Goal:** Enable time-dependent hormone curves and dynamic parameter calculations

#### Assignment Rules
- Parse `<assignmentRule>` elements from SBML
- Build dependency graph to determine evaluation order
- Evaluate rules at each time step before calculating derivatives
- Support MathML expressions (exp, sin, cos, power, etc.)

**Design Ideas:**
```rust
// In models.rs
pub struct AssignmentRule {
    variable: String,
    expression: MathExpression,
}

// In simulator.rs
fn apply_assignment_rules(&mut self) {
    // Sort rules by dependency order
    // Evaluate each rule and update species/parameter values
}
```

#### Rate Rules
- Parse `<rateRule>` elements for time-dependent changes
- Add rate rule contributions to ODE system
- Handle both species and parameter rate rules

**Design Ideas:**
```rust
fn compute_derivatives(&self, state: &DVector<f64>) -> DVector<f64> {
    let mut derivs = self.compute_reaction_rates(state);
    derivs += self.compute_rate_rules(state);
    derivs
}
```

### Priority 2: Time Series Input Support
**Goal:** Allow feeding external data (e.g., hormone curves) into simulations

- Extend configuration format to accept time series data
- Interpolate values between time points
- Override species/parameter values at each time step

**Design Ideas:**
```javascript
// Config with time series
{
    timeEnd: 6720,
    timeStep: 1.0,
    timeSeries: {
        time: [0, 168, 336, ...],
        Testosterone: [30, 35, 40, ...],
        LH: [10, 12, 15, ...]
    }
}
```

### Priority 3: Enhanced Mathematical Functions
**Goal:** Support complex kinetic laws and mathematical expressions

- Implement MathML parser for complex expressions
- Support standard math functions: exp, log, sin, cos, pow, sqrt, abs
- Handle piecewise functions and conditionals
- Support min/max functions

**Design Ideas:**
```rust
pub enum MathOp {
    Add, Subtract, Multiply, Divide,
    Exp, Log, Sin, Cos, Power,
    GreaterThan, LessThan, Equal,
    Piecewise { conditions: Vec<(Expr, Expr)>, default: Expr }
}
```

### Priority 4: Events Support
**Goal:** Handle discrete state changes at specific times or conditions

- Parse `<event>` elements with triggers and assignments
- Evaluate trigger conditions at each time step
- Apply event assignments when triggered
- Handle event priorities and delays

### Priority 5: Visualization Enhancements
**Goal:** Better data visualization and analysis tools

- Implement data downsampling for large simulations
- Add phase plots and bifurcation diagrams
- Export to standard formats (CSV, HDF5)
- Real-time plotting during simulation

### Priority 6: Performance Optimizations
**Goal:** Handle larger models efficiently

- Implement sparse matrix support for large models
- Add adaptive time stepping for stiff systems
- Parallelize parameter scans using Web Workers
- Implement JIT compilation for kinetic laws

### Priority 7: Model Validation
**Goal:** Ensure model correctness

- Implement SBML validation against schema
- Check for conservation laws
- Detect algebraic loops in rules
- Provide meaningful error messages

## Architecture Improvements

### Parser Module
- Refactor to use a proper XML/MathML parsing library
- Create abstract syntax tree (AST) for mathematical expressions
- Implement visitor pattern for expression evaluation

### Simulator Module
- Separate ODE solver interface from implementation
- Add plugin architecture for custom solvers
- Implement state-space representation for better performance

### API Design
- Add streaming API for long simulations
- Implement observable pattern for simulation progress
- Add cancellation support for long-running simulations

## Testing Strategy

- Unit tests for each mathematical function
- Integration tests with reference SBML models
- Benchmark suite comparing with other simulators
- Fuzzing tests for parser robustness

## Documentation Needs

- Tutorial for modeling hormone dynamics
- Guide for implementing custom kinetic laws
- Performance tuning guide
- SBML feature compatibility matrix

## License

MIT

## Contributing

Contributions are welcome! Priority areas:
1. SBML rules implementation (assignment and rate rules)
2. Mathematical expression parser
3. Time series input support
4. Test coverage improvements

Please open an issue to discuss major changes or submit a pull request for bug fixes and small improvements.