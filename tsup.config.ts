import { defineConfig } from 'tsup';

export default defineConfig([
  {
    entry: ['./src/index.ts'],
    format: 'esm',
    outDir: './dist',
    clean: true,
    dts: true,
    minify: true,
  },
  {
    entry: ['./src/index.ts'],
    format: ['esm', 'cjs'],
    outDir: './dist',
    clean: true,
    dts: false,
    minify: true,
  },
]);
