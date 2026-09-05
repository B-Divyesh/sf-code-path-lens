import { defineConfig } from 'vite';
import { fileURLToPath } from 'node:url';
import { resolve, dirname } from 'node:path';
import { readFileSync } from 'node:fs';

const root = dirname(fileURLToPath(import.meta.url));
export default defineConfig({
  root,
  plugins: [{
    name: 'inline-local-css',
    transformIndexHtml: {
      order: 'pre',
      handler(html) {
        const css = readFileSync(resolve(root, 'styles.css'), 'utf8');
        return html.replace('<link rel="stylesheet" href="/styles.css">', `<style>${css}</style>`);
      }
    }
  }],
  build: {
    outDir: resolve(root, '../dist/site'),
    emptyOutDir: true,
    target: 'es2022',
    cssCodeSplit: false,
    rollupOptions: {
      input: {
        index: resolve(root, 'index.html'),
        demo: resolve(root, 'demo/index.html'),
        privacy: resolve(root, 'privacy/index.html'),
        terms: resolve(root, 'terms/index.html'),
        '404': resolve(root, '404.html')
      }
    }
  }
});
