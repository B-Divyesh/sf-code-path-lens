import { chromium } from 'playwright';
import AxeBuilder from '@axe-core/playwright';

const url = process.env.LENS_TEST_URL || 'http://127.0.0.1:5173';
const browser = await chromium.launch();
let failed = false;
for (const viewport of [{ width: 1366, height: 900 }, { width: 390, height: 844 }]) {
  const context = await browser.newContext({ viewport });
  const page = await context.newPage();
  await page.goto(url, { waitUntil: 'networkidle' });
  const result = await new AxeBuilder({ page }).analyze();
  const serious = result.violations.filter((item) => ['serious', 'critical'].includes(item.impact));
  console.log(`${viewport.width}px: ${result.violations.length} total axe findings; ${serious.length} serious/critical`);
  for (const item of serious) {
    console.error(`${item.id}: ${item.help}`);
    for (const node of item.nodes) console.error(`  ${node.target.join(' ')} — ${node.failureSummary}`);
  }
  failed ||= serious.length > 0;
  await context.close();
}
await browser.close();
if (failed) process.exit(1);
