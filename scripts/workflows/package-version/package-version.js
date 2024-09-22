const fs = require('node:fs');
// const { version } = require('package.json');

(async () => {
  console.log(`cwd:${process.cwd()}`);
  // process.cwd();
  const fileList = fs.readdirSync('.');

  console.log('Files and folders in the directory:', fileList);

  // core.setOutput('value', version);
})();
