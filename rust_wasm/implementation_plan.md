Here's a complete project structure for a Rust+WASM BioModel simulator:

## Project Directory Structure

```
biomodel-wasm/
├── Cargo.toml                 # Rust project manifest
├── Cargo.lock
├── package.json               # NPM package configuration
├── README.md
├── LICENSE
│
├── src/                       # Rust source code
│   ├── lib.rs                # Main library entry point
│   ├── parser/               # SBML parsing module
│   │   ├── mod.rs
│   │   ├── sbml.rs          # SBML XML parsing
│   │   └── mathml.rs        # MathML equation parsing
│   ├── simulator/            # Simulation engine
│   │   ├── mod.rs
│   │   ├── ode.rs           # ODE solvers
│   │   ├── stochastic.rs    # Gillespie algorithm
│   │   └── events.rs        # SBML events handling
│   ├── models/               # Model structures
│   │   ├── mod.rs
│   │   ├── species.rs
│   │   ├── reactions.rs
│   │   └── compartments.rs
│   └── utils/               # Utilities
│       ├── mod.rs
│       └── math.rs
│
├── tests/                    # Test files
│   ├── integration/
│   │   └── sbml_test_suite/  # SBML test suite cases
│   └── web.rs               # WASM-specific tests
│
├── examples/                 # Example usage
│   ├── node/
│   │   ├── package.json
│   │   ├── index.js        # Node.js example
│   │   └── models/
│   │       └── glycolysis.xml
│   └── web/
│       ├── index.html      # Browser example
│       ├── app.js
│       └── style.css
│
├── pkg/                     # Generated WASM package (git-ignored)
│   ├── biomodel_wasm_bg.wasm
│   ├── biomodel_wasm.js
│   ├── biomodel_wasm.d.ts
│   └── package.json
│
├── scripts/                 # Build scripts
│   ├── build.sh
│   └── test.sh
│
└── .github/
    └── workflows/
        └── ci.yml          # GitHub Actions CI/CD
```

## Core Configuration Files

### `Cargo.toml`
```toml
[package]
name = "biomodel-wasm"
version = "0.1.0"
authors = ["Your Name"]
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.6"
nalgebra = "0.32"
quick-xml = "0.31"
thiserror = "1.0"
getrandom = { version = "0.2", features = ["js"] }

[dependencies.web-sys]
version = "0.3"
features = [
  "console",
  "Performance",
]

[dev-dependencies]
wasm-bindgen-test = "0.3"

[profile.release]
opt-level = 3
lto = true
```

### `package.json` (root)
```json
{
  "name": "biomodel-wasm",
  "version": "0.1.0",
  "description": "SBML BioModel simulator in WebAssembly",
  "main": "pkg/biomodel_wasm.js",
  "types": "pkg/biomodel_wasm.d.ts",
  "scripts": {
    "build": "wasm-pack build --target web --out-dir pkg",
    "build:node": "wasm-pack build --target nodejs --out-dir pkg-node",
    "build:bundler": "wasm-pack build --target bundler --out-dir pkg-bundler",
    "test": "wasm-pack test --headless --firefox",
    "clean": "rm -rf pkg pkg-node pkg-bundler target",
    "example:web": "cd examples/web && python3 -m http.server 8000",
    "example:node": "cd examples/node && npm start"
  },
  "devDependencies": {
    "wasm-pack": "^0.12.0"
  },
  "files": [
    "pkg/"
  ],
  "repository": {
    "type": "git",
    "url": "https://github.com/yourusername/biomodel-wasm"
  },
  "keywords": [
    "sbml",
    "biomodel",
    "simulation",
    "wasm",
    "systems-biology"
  ],
  "license": "MIT"
}
```

## Installation & Usage

### For Developers (Building from Source)

```bash
# Prerequisites
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  # Install Rust
cargo install wasm-pack                                         # Install wasm-pack

# Clone and build
git clone https://github.com/yourusername/biomodel-wasm
cd biomodel-wasm

# Build for web browsers
npm run build

# Build for Node.js
npm run build:node

# Run tests
npm test

# Try examples
npm run example:web  # Opens browser at localhost:8000
npm run example:node # Runs Node.js simulation
```

### For Users (NPM Package)

```bash
# Install from npm
npm install biomodel-wasm

# Or with yarn
yarn add biomodel-wasm
```

### Usage in Node.js
```javascript
// examples/node/index.js
const fs = require('fs');
const { BioModel } = require('biomodel-wasm');

// Load SBML model
const sbmlContent = fs.readFileSync('./models/glycolysis.xml', 'utf8');

// Create and simulate
const model = new BioModel(sbmlContent);
const results = model.simulate({
    timeEnd: 100.0,
    timeStep: 0.1,
    method: "rk4"  // or "euler", "gillespie"
});

// Access results
console.log("Species:", model.getSpeciesNames());
console.log("Time points:", results.time);
console.log("Concentrations:", results.values);

// Parameter scan
const scanResults = model.parameterScan(
    "k1",           // parameter name
    [0.1, 1.0, 10.0] // values to test
);

// Clean up
model.free();
```

### Usage in Browser
```html
<!-- examples/web/index.html -->
<!DOCTYPE html>
<html>
<head>
    <script type="module">
        import init, { BioModel } from '../../pkg/biomodel_wasm.js';
        
        async function run() {
            // Initialize WASM module
            await init();
            
            // Fetch SBML model
            const response = await fetch('models/repressilator.xml');
            const sbmlContent = await response.text();
            
            // Create model
            const model = new BioModel(sbmlContent);
            
            // Run simulation
            const results = model.simulate({
                timeEnd: 1000.0,
                timeStep: 0.1
            });
            
            // Plot with Chart.js or similar
            plotResults(results);
            
            // Clean up
            model.free();
        }
        
        run();
    </script>
</head>
<body>
    <canvas id="chart"></canvas>
</body>
</html>
```

### Usage with Bundlers (Webpack/Vite)
```javascript
// For use with bundlers
import init, { BioModel } from 'biomodel-wasm/pkg-bundler';

// In your React/Vue/Svelte app
await init();
const model = new BioModel(sbmlContent);
```

## Docker Support (Optional)

### `Dockerfile`
```dockerfile
FROM rust:1.75 as builder
RUN cargo install wasm-pack
WORKDIR /usr/src/biomodel-wasm
COPY . .
RUN wasm-pack build --target web

FROM nginx:alpine
COPY --from=builder /usr/src/biomodel-wasm/pkg /usr/share/nginx/html/pkg
COPY examples/web /usr/share/nginx/html
```

## GitHub Actions CI

### `.github/workflows/ci.yml`
```yaml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - uses: jetli/wasm-pack-action@v0.4.0
      - run: wasm-pack test --headless --chrome
      - run: wasm-pack build
      
  publish:
    if: startsWith(github.ref, 'refs/tags/')
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: jetli/wasm-pack-action@v0.4.0
      - run: wasm-pack build
      - run: npm publish ./pkg
        env:
          NODE_AUTH_TOKEN: ${{secrets.NPM_TOKEN}}
```

## Development Workflow

```bash
# 1. Make changes to Rust code
vim src/simulator/ode.rs

# 2. Build and test
cargo build
cargo test
wasm-pack build

# 3. Test in browser
cd examples/web && python3 -m http.server

# 4. Test in Node
cd examples/node && node index.js

# 5. Publish to npm
npm version patch
git push --tags
# CI automatically publishes to npm
```

This structure gives you:
- Clean separation between Rust and JS code
- Examples for both environments  
- Automated testing and publishing
- TypeScript definitions for free
- Multiple build targets (web, Node, bundler)

The beauty is users just `npm install` and start using it - they never need to touch Rust unless they want to contribute!