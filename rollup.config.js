import commonjs from '@rollup/plugin-commonjs';
import json from '@rollup/plugin-json';
import {nodeResolve} from '@rollup/plugin-node-resolve';

export default {
  input: 'src/index',
  output: {
    dir: 'api',
    format: 'cjs',
    exports: 'default',
  },
  plugins: [commonjs(), json(), nodeResolve()],
};
