const fs = require('node:fs');
const core = require('@actions/core');

const packageJson = JSON.parse(fs.readFileSync('./package.json'));
core.setOutput('value', packageJson.version);
