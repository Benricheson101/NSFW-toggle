const pkg = require('./package.json');

const config = {
  singleQuote: true,
  bracketSpacing: false,
  trailingComma: 'es5',
  arrowParens: 'avoid',
};

///////////////////////////////
//          SVELTE          //
/////////////////////////////
if (
  pkg.dependencies?.['prettier-plugin-svelte'] ||
  pkg.devDependencies?.['prettier-plugin-svelte']
) {
  console.log('svelte plugin');
  config.svelteBracketNewLine = false;
}

module.exports = config;
