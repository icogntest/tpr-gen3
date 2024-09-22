const fs = require('node:fs');

const packageJson = JSON.parse(fs.readFileSync('./package.json'));
core.setOutput('value', packageJson.version);
