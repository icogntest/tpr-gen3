const fs = require('node:fs');
// const { version } = require('package.json');

(async () => {
  console.log(`cwd:${process.cwd()}`);
  // process.cwd();
  const json = fs.readFileSync('.').toJSON();
  console.log(json);

  // core.setOutput('value', version);
})();
