const { version } = require('./package');

(async () => {
  core.setOutput('value', version);
})();
