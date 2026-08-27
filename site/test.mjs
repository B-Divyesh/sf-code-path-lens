import { readFile, stat } from 'node:fs/promises';

const pages = ['dist/site/index.html', 'dist/site/privacy/index.html', 'dist/site/terms/index.html'];
for (const page of pages) {
  const html = await readFile(page, 'utf8');
  if (!html.includes('<html lang="en"')) throw new Error(`${page}: missing lang`);
  if (!html.includes('<title>')) throw new Error(`${page}: missing title`);
  if (!html.includes('<main')) throw new Error(`${page}: missing main`);
  if ((html.match(/<h1[ >]/g) || []).length !== 1) throw new Error(`${page}: expected one h1`);
  for (const image of html.matchAll(/<img\b[^>]*>/g)) {
    if (!/\balt=/.test(image[0])) throw new Error(`${page}: image without alt`);
  }
}
const entry = await readFile('dist/site/index.html', 'utf8');
if (!entry.includes('/privacy/') || !entry.includes('/terms/')) throw new Error('legal links missing');
if (!entry.includes('pilot-api.sociobot.in')) throw new Error('staging checkout link missing');
const hero = await stat('dist/site/assets/hero-field-notebook.webp');
if (hero.size > 300_000) throw new Error(`hero is ${hero.size} bytes; budget is 300000`);
console.log(`site checks passed; hero ${(hero.size / 1024).toFixed(1)} KB`);
