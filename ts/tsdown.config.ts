import { defineConfig } from 'tsdown';

export default defineConfig([
  {
    entry: ['./src/index.ts'],
    format: 'cjs',
    outDir: './dist',
    clean: true,
    dts: false,
    minify: true,
  },
  {
    entry: ['./src/index.ts'],
    format: {
      esm: {
        outExtensions: () => ({ dts: '.d.ts', js: '.js' }),
      },
    },
    outDir: './dist',
    clean: true,
    dts: true,
    minify: true,
  },
]);
