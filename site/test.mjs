import { readFile, stat } from 'node:fs/promises';

const pages = [
  ['dist/site/index.html', 'Code Path Lens — Trace bounded code paths'],
  ['dist/site/demo/index.html', 'Demo — Code Path Lens'],
  ['dist/site/privacy/index.html', 'Privacy — Code Path Lens'],
  ['dist/site/terms/index.html', 'Terms — Code Path Lens'],
  ['dist/site/404.html', 'Page not found — Code Path Lens']
];

for (const [path, title] of pages) {
  const html = await readFile(path, 'utf8');
  if (!html.includes('<html lang="en"')) throw new Error(`${path}: missing language`);
  if (!html.includes(`<title>${title}</title>`)) throw new Error(`${path}: unexpected route title`);
  if (!html.includes('<main')) throw new Error(`${path}: missing main landmark`);
  if ((html.match(/<h1[ >]/g) || []).length !== 1) throw new Error(`${path}: expected one h1`);
  if (!html.includes('rel="canonical"')) throw new Error(`${path}: missing canonical URL`);
  if (!html.includes('property="og:image"') || !html.includes('name="twitter:card"')) throw new Error(`${path}: missing share metadata`);
  if (!html.includes('Built by Param Factory')) throw new Error(`${path}: missing shared footer`);
  for (const image of html.matchAll(/<img\b[^>]*>/g)) {
    if (!/\balt=/.test(image[0])) throw new Error(`${path}: image without alt text`);
  }
}

const entry = await readFile('dist/site/index.html', 'utf8');
if (!entry.includes('Try it with sample data')) throw new Error('landing page has no sample action');
if (!entry.includes('/demo/')) throw new Error('landing page has no demo route');
const demo = await readFile('dist/site/demo/index.html', 'utf8');
for (const value of ['Demo — sample data, nothing is saved', 'Reset demo', 'Start for real']) {
  if (!demo.includes(value)) throw new Error(`demo page is missing ${value}`);
}

const deploymentConfig = JSON.parse(await readFile('dist/site/staticwebapp.config.json', 'utf8'));
if (deploymentConfig.navigationFallback) throw new Error('multipage site must not rewrite missing URLs to the landing page');
if (deploymentConfig.responseOverrides?.['404']?.statusCode !== 404) throw new Error('404 response override is missing');
if (deploymentConfig.responseOverrides?.['404']?.rewrite !== '/404.html') throw new Error('404 document is missing');
if (!deploymentConfig.routes?.some((route) => route.route === '/demo' && route.rewrite === '/demo/index.html')) throw new Error('direct /demo route is missing');
if (!deploymentConfig.globalHeaders?.['Content-Security-Policy']?.includes("connect-src 'self'")) throw new Error('CSP does not limit connections to this site');
const assetRoute = deploymentConfig.routes?.find((route) => route.route === '/assets/*');
if (assetRoute?.headers?.['Cache-Control'] !== 'public, max-age=31536000, immutable') throw new Error('hashed asset cache policy missing');

const hero = await stat('dist/site/assets/hero-field-notebook.webp');
const shareCard = await stat('dist/site/assets/code-path-lens-card.webp');
const touchIcon = await stat('dist/site/apple-touch-icon.png');
if (hero.size > 300_000) throw new Error(`hero is ${hero.size} bytes; budget is 300000`);
if (shareCard.size === 0 || touchIcon.size === 0) throw new Error('share or touch image is empty');
console.log(`site checks passed; hero ${(hero.size / 1024).toFixed(1)} KB`);
