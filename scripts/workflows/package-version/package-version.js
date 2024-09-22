const { version } = require('./package.json');

(async () => {
  core.setOutput('value', version);
})();
