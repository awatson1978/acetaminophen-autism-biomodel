const fs = require('fs');
const path = require('path');

async function main() {
    try {
        const { BioModel } = require('../../pkg-node/biomodels_wasm.js');
        
        console.log('BioModels WASM - Node.js Example');
        console.log('=================================\n');
        
        const sampleModel = `<?xml version="1.0" encoding="UTF-8"?>
<sbml xmlns="http://www.sbml.org/sbml/level3/version2/core" level="3" version="2">
  <model id="lotka_volterra" name="Lotka-Volterra Predator-Prey Model">
    <listOfCompartments>
      <compartment id="ecosystem" name="Ecosystem" constant="true"/>
    </listOfCompartments>
    <listOfSpecies>
      <species id="prey" name="Prey Population" compartment="ecosystem" initialConcentration="10.0"/>
      <species id="predator" name="Predator Population" compartment="ecosystem" initialConcentration="5.0"/>
    </listOfSpecies>
    <listOfParameters>
      <parameter id="prey_growth" value="1.0" constant="true"/>
      <parameter id="predation_rate" value="0.1" constant="true"/>
      <parameter id="predator_efficiency" value="0.075" constant="true"/>
      <parameter id="predator_death" value="1.5" constant="true"/>
    </listOfParameters>
    <listOfReactions>
      <reaction id="prey_birth" name="Prey Birth">
        <listOfReactants>
          <speciesReference species="prey"/>
        </listOfReactants>
        <listOfProducts>
          <speciesReference species="prey"/>
          <speciesReference species="prey"/>
        </listOfProducts>
      </reaction>
      <reaction id="predation" name="Predation">
        <listOfReactants>
          <speciesReference species="prey"/>
          <speciesReference species="predator"/>
        </listOfReactants>
        <listOfProducts>
          <speciesReference species="predator"/>
          <speciesReference species="predator"/>
        </listOfProducts>
      </reaction>
      <reaction id="predator_death" name="Predator Death">
        <listOfReactants>
          <speciesReference species="predator"/>
        </listOfReactants>
        <listOfProducts/>
      </reaction>
    </listOfReactions>
  </model>
</sbml>`;
        
        console.log('Loading Lotka-Volterra predator-prey model...');
        const model = new BioModel(sampleModel);
        
        const speciesNames = model.getSpeciesNames();
        const speciesIds = model.getSpeciesIds();
        console.log(`\nModel contains ${speciesNames.length} species:`);
        speciesNames.forEach((name, i) => {
            console.log(`  - ${name} (${speciesIds[i]})`);
        });
        
        const parameters = model.getParameters();
        console.log(`\n${Object.keys(parameters).length} parameters defined`);
        
        console.log('\nRunning simulation...');
        const results = model.simulate({
            timeEnd: 50.0,
            timeStep: 0.1,
            method: 'rk4'
        });
        
        console.log(`Simulation complete! Generated ${results.time.length} time points`);
        
        console.log('\nSample results (first 10 time points):');
        console.log('Time\t\tPrey\t\tPredator');
        console.log('----\t\t----\t\t--------');
        for (let i = 0; i < Math.min(10, results.time.length); i++) {
            const prey = results.values[i * results.num_species];
            const predator = results.values[i * results.num_species + 1];
            console.log(`${results.time[i].toFixed(1)}\t\t${prey.toFixed(3)}\t\t${predator.toFixed(3)}`);
        }
        
        console.log('\n=== Parameter Scan Example ===');
        console.log('Scanning predation rate from 0.05 to 0.2...\n');
        
        const scanResults = model.parameterScan('predation_rate', [0.05, 0.1, 0.15, 0.2]);
        
        console.log('Average final populations for each parameter value:');
        scanResults.forEach(scan => {
            const lastIndex = scan.results.time.length - 1;
            const finalPrey = scan.results.values[lastIndex * scan.results.num_species];
            const finalPredator = scan.results.values[lastIndex * scan.results.num_species + 1];
            console.log(`Predation rate = ${scan.parameter_value}: Prey = ${finalPrey.toFixed(2)}, Predator = ${finalPredator.toFixed(2)}`);
        });
        
        const outputDir = path.join(__dirname, 'output');
        if (!fs.existsSync(outputDir)) {
            fs.mkdirSync(outputDir);
        }
        
        const outputFile = path.join(outputDir, 'simulation_results.json');
        fs.writeFileSync(outputFile, JSON.stringify(results, null, 2));
        console.log(`\nFull results saved to: ${outputFile}`);
        
        model.free();
        console.log('\nSimulation complete and resources freed!');
        
    } catch (error) {
        console.error('Error:', error);
        process.exit(1);
    }
}

main();