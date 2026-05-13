import { defineConfig } from 'tsup';

export default defineConfig({
  entry: {
    server: 'src/server.ts'
  },
  format: ['esm'],
  dts: true,
  clean: true,
  minify: true,
  splitting: false,
  sourcemap: true,
  target: 'node18',
  outDir: 'dist',
  external: [
    '@neondatabase/serverless',
    'hono',
    '@hono/node-server'
  ]
});
