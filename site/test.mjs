import { readFile, stat } from 'node:fs/promises';

const productionBillingBase = 'https://api.sociobot.in/api/v1';

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
if (!entry.includes(`${productionBillingBase}/products/code-path-lens/checkout`)) throw new Error('production checkout link missing');
const script = entry.match(/<script[^>]+src="([^"]+)"/i)?.[1];
if (!script) throw new Error('application script missing');
const application = await readFile(`dist/site${script}`, 'utf8');
if (!application.includes(productionBillingBase) || !application.includes('/products/code-path-lens/verify')) throw new Error('production license verification endpoint missing');
const releaseFiles = [entry, application];
if (releaseFiles.some((content) => content.includes('pilot-api.sociobot.in'))) throw new Error('pilot billing endpoint shipped in release output');
if (releaseFiles.some((content) => content.includes('__CODE_PATH_LENS_BILLING_BASE__'))) throw new Error('billing endpoint placeholder shipped in release output');
const deploymentConfig = JSON.parse(await readFile('dist/site/staticwebapp.config.json', 'utf8'));
if (deploymentConfig.globalHeaders?.['Content-Security-Policy'] !== "default-src 'self'; base-uri 'self'; connect-src 'self' https://api.sociobot.in; form-action 'self'; frame-ancestors 'none'; img-src 'self'; object-src 'none'; script-src 'self'; style-src 'self' 'unsafe-inline'; worker-src 'self'") throw new Error('deployment CSP is missing or unexpectedly broad');
if (deploymentConfig.globalHeaders?.['X-Frame-Options'] !== 'DENY') throw new Error('deployment frame policy missing');
const assetRoute = deploymentConfig.routes?.find((route) => route.route === '/assets/*');
if (assetRoute?.headers?.['Cache-Control'] !== 'public, max-age=31536000, immutable') throw new Error('hashed asset immutable cache policy missing');
const hero = await stat('dist/site/assets/hero-field-notebook.webp');
if (hero.size > 300_000) throw new Error(`hero is ${hero.size} bytes; budget is 300000`);
console.log(`site checks passed; hero ${(hero.size / 1024).toFixed(1)} KB`);
