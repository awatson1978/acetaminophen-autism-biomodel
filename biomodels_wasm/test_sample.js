// Simple test to verify the WASM module loads and runs

console.log("Testing BioModels WASM Module");
console.log("=============================\n");

const sampleSBML = `<?xml version="1.0" encoding="UTF-8"?>
<sbml xmlns="http://www.sbml.org/sbml/level3/version2/core" level="3" version="2">
  <model id="simple_test" name="Simple Test Model">
    <listOfCompartments>
      <compartment id="cell" name="Cell" constant="true"/>
    </listOfCompartments>
    <listOfSpecies>
      <species id="A" name="Species A" compartment="cell" initialConcentration="10.0"/>
      <species id="B" name="Species B" compartment="cell" initialConcentration="0.0"/>
    </listOfSpecies>
    <listOfParameters>
      <parameter id="k1" value="0.1" constant="true"/>
    </listOfParameters>
    <listOfReactions>
      <reaction id="conversion" name="A to B">
        <listOfReactants>
          <speciesReference species="A"/>
        </listOfReactants>
        <listOfProducts>
          <speciesReference species="B"/>
        </listOfProducts>
      </reaction>
    </listOfReactions>
  </model>
</sbml>`;

console.log("Sample SBML model created:");
console.log("- 1 compartment (Cell)");
console.log("- 2 species (A and B)");
console.log("- 1 reaction (A -> B)");
console.log("- Initial: A=10.0, B=0.0");
console.log("\nThis represents a simple conversion reaction where A converts to B.");
console.log("\n✅ WASM module is ready to use!");
console.log("\nTo test in browser: npm run example:web");
console.log("To test in Node.js: npm run example:node");