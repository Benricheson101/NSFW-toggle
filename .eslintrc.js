////////////////////////////////////////////
//          BEN'S ESLINT CONFIG          //
//////////////////////////////////////////

/* eslint spaced-comment: 0 */
const pkg = require('./package.json');

/////////////////////////////
//          BASE          //
///////////////////////////
const config = {
  env: {
    es2021: true,
    node: true,
  },
  extends: ['google'],
  plugins: [],
  parserOptions: {
    ecmaVersion: 12,
    sourceType: 'module',
  },
  overrides: [],
  rules: {
    'require-jsdoc': 0,
    curly: 1,
  },
};

///////////////////////////////
//          SVELTE          //
/////////////////////////////
if (
  (pkg.dependencies?.svelte || pkg.devDependencies?.svelte) &&
  (pkg.dependencies?.['eslint-plugin-svelte3'] ||
    pkg.devDependencies?.['eslint-plugin-svelte3'])
) {
  config.env.browser = true;
  config.plugins.push('svelte3');
  config.overrides.push({files: ['**/*.svelte'], processor: 'svelte3/svelte3'});
  config.rules['no-invalid-this'] = 0;
}

/////////////////////////////////
//          PRETTIER          //
///////////////////////////////
if (
  (pkg.dependencies?.prettier || pkg.devDependencies?.prettier) &&
  (pkg.dependencies?.['eslint-config-prettier'] ||
    pkg.devDependencies?.['eslint-config-prettier'])
) {
  config.extends.push('prettier');
}

module.exports = config;
