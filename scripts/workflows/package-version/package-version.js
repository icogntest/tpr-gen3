const fs = require('node:fs');
// const { version } = require('package.json');

(async () => {
  console.log(`cwd:${process.cwd()}`);
  // process.cwd();
  const json = JSON.parse(fs.readFileSync('./package.json'));
  console.log(json);

  // core.setOutput('value', version);
})();
