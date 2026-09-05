import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import { mkdir, mkdtemp, readFile, stat, writeFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { dirname, extname, join, normalize, resolve } from 'node:path';
import { tmpdir } from 'node:os';
import { spawnSync } from 'node:child_process';
import { chromium } from 'playwright';

const root = resolve('.');
const binary = join(root, 'target', 'debug', process.platform === 'win32' ? 'code-path-lens.exe' : 'code-path-lens');
const sampleRoot = join(root, 'examples', 'checkout-sample');
const claimTag = process.argv.includes('--grep') ? process.argv[process.argv.indexOf('--grep') + 1] : null;

function run(command, args, options = {}) {
  return spawnSync(command, args, { cwd: root, encoding: 'utf8', input: '', ...options });
}

function runLens(args, options = {}) {
  const result = run(binary, args, options);
  return result;
}

function assertSuccess(result, description) {
  assert.equal(result.status, 0, `${description}\nstdout: ${result.stdout}\nstderr: ${result.stderr}`);
}

function sampleGraph(extra = []) {
  const result = runLens(['trace', 'handle_order', '--root', sampleRoot, '--json', '--link-template', 'sample://{path}:{line}', ...extra]);
  assertSuccess(result, 'sample trace failed');
  return { output: result.stdout, graph: JSON.parse(result.stdout) };
}

async function makeTempRepository(files) {
  const directory = await mkdtemp(join(tmpdir(), 'code-path-lens-claim-'));
  for (const [path, content] of Object.entries(files)) {
    const destination = join(directory, path);
    await mkdir(dirname(destination), { recursive: true });
    await writeFile(destination, content, 'utf8');
  }
  return directory;
}

async function startSite() {
  const site = join(root, 'dist', 'site');
  const types = { '.html': 'text/html; charset=utf-8', '.js': 'text/javascript; charset=utf-8', '.svg': 'image/svg+xml', '.webp': 'image/webp', '.png': 'image/png', '.xml': 'application/xml', '.txt': 'text/plain' };
  const server = createServer(async (request, response) => {
    const url = new URL(request.url, 'http://localhost');
    let pathname = decodeURIComponent(url.pathname);
    if (pathname === '/') pathname = '/index.html';
    if (pathname === '/demo') pathname = '/demo/index.html';
    if (pathname.endsWith('/')) pathname += 'index.html';
    const candidate = normalize(join(site, pathname));
    const allowed = candidate.startsWith(site);
    let file = allowed ? candidate : join(site, '404.html');
    let statusCode = allowed && existsSync(file) ? 200 : 404;
    if (statusCode === 404) file = join(site, '404.html');
    try {
      const content = await readFile(file);
      response.writeHead(statusCode, { 'content-type': types[extname(file)] || 'application/octet-stream' });
      response.end(content);
    } catch {
      response.writeHead(404, { 'content-type': 'text/plain; charset=utf-8' });
      response.end('Not found');
    }
  });
  await new Promise((resolveListen) => server.listen(0, '127.0.0.1', resolveListen));
  const address = server.address();
  return { server, origin: `http://127.0.0.1:${address.port}` };
}

async function withBrowser(test) {
  const { server, origin } = await startSite();
  const browser = await chromium.launch();
  try {
    await test(browser, origin);
  } finally {
    await browser.close();
    await new Promise((close) => server.close(close));
  }
}

const tests = [
  {
    tag: '@claim:deterministic-slice',
    run: async () => {
      const first = sampleGraph().output;
      const second = sampleGraph().output;
      assert.equal(second, first, 'the same sample trace must produce byte-identical JSON');
    }
  },
  {
    tag: '@claim:bounded-evidence',
    run: async () => {
      const { graph } = sampleGraph();
      const node = (label, kind) => graph.nodes.some((item) => item.label === label && item.kind === kind);
      assert(node('post_order', 'function'), 'known caller is missing');
      assert(node('validate', 'function'), 'known callee is missing');
      assert(node('Order', 'type'), 'referenced type is missing');
      assert(node('database I/O', 'data_boundary'), 'data boundary is missing');
      assert(node('emit_receipt', 'unresolved'), 'unresolved call is missing');
    }
  },
  {
    tag: '@claim:source-evidence',
    run: async () => {
      const { graph } = sampleGraph();
      const entry = graph.nodes.find((node) => node.label === 'handle_order');
      assert.equal(entry.location.path, 'src/orders.rs');
      assert.equal(entry.location.line, 7);
      assert.match(entry.location.link, /^sample:\/\/src\/orders\.rs:7$/);
      assert.match(entry.excerpt, /database_insert/);
    }
  },
  {
    tag: '@claim:language-adapters',
    run: async () => {
      const directory = await makeTempRepository({
        'rust.rs': 'fn rust_entry() {}\n',
        'types.ts': 'function ts_entry() {}\n',
        'script.js': 'function js_entry() {}\n',
        'script.py': 'def py_entry():\n    pass\n',
        'main.go': 'package sample\nfunc go_entry() {}\n'
      });
      for (const symbol of ['rust_entry', 'ts_entry', 'js_entry', 'py_entry', 'go_entry']) {
        const result = runLens(['trace', symbol, '--root', directory, '--json']);
        assertSuccess(result, `${symbol} adapter trace failed`);
        assert.equal(JSON.parse(result.stdout).symbol, symbol);
      }
    }
  },
  {
    tag: '@claim:exclusions',
    run: async () => {
      const directory = await makeTempRepository({
        '.gitignore': 'ignored/\n',
        'src/main.rs': 'fn entry() {}\n',
        'src/generated.generated.rs': '// @generated\nfn generated_entry() {}\n',
        'ignored/hidden.rs': 'fn ignored_entry() {}\n',
        'vendor/hidden.rs': 'fn vendor_entry() {}\n'
      });
      const init = run('git', ['init', '--quiet', directory]);
      assertSuccess(init, 'could not initialize ignored-file fixture');
      const hidden = runLens(['trace', 'ignored_entry', '--root', directory, '--json']);
      assert.equal(hidden.status, 3, 'ignored source should not be traced');
      const vendor = runLens(['trace', 'vendor_entry', '--root', directory, '--json']);
      assert.equal(vendor.status, 3, 'vendor source should not be traced');
      const generated = runLens(['trace', 'generated_entry', '--root', directory, '--json']);
      assert.equal(generated.status, 3, 'generated source should be skipped by default');
      const included = runLens(['trace', 'generated_entry', '--root', directory, '--json', '--include-generated']);
      assertSuccess(included, 'generated source should be traceable when explicitly included');
    }
  },
  {
    tag: '@claim:formats',
    run: async () => {
      const directory = await mkdtemp(join(tmpdir(), 'code-path-lens-format-'));
      const htmlPath = join(directory, 'lens.html');
      const dotPath = join(directory, 'lens.dot');
      assertSuccess(runLens(['trace', 'handle_order', '--root', sampleRoot, '--output', htmlPath]), 'HTML output failed');
      assertSuccess(runLens(['trace', 'handle_order', '--root', sampleRoot, '--format', 'dot', '--output', dotPath]), 'DOT output failed');
      assert.match(await readFile(dotPath, 'utf8'), /digraph code_path_lens/);
      await withBrowser(async (browser) => {
        const context = await browser.newContext();
        const page = await context.newPage();
        await page.goto(`file://${htmlPath}`);
        await assert.doesNotReject(() => page.locator('h1').textContent());
        assert.match(await page.locator('body').innerText(), /handle_order/);
        await context.close();
      });
    }
  },
  {
    tag: '@claim:free-bounds',
    run: async () => {
      const { graph } = sampleGraph(['--depth', '0', '--max-nodes', '1']);
      assert.equal(graph.limits.depth, 0);
      assert.equal(graph.limits.max_nodes, 1);
      assert.equal(graph.nodes.length, 1);
      assert.equal(graph.limits.truncated, true);
    }
  },
  {
    tag: '@claim:static-approximation',
    run: async () => {
      const { graph } = sampleGraph();
      assert.match(graph.approximation_notice, /not a runtime-complete call trace/i);
      assert(graph.warnings.some((warning) => /Static approximation/.test(warning)), 'output lacks the static-analysis warning');
    }
  },
  {
    tag: '@claim:local-cli',
    run: async () => {
      const result = runLens(['trace', 'handle_order', '--root', sampleRoot, '--json'], {
        env: { ...process.env, HTTP_PROXY: 'http://127.0.0.1:9', HTTPS_PROXY: 'http://127.0.0.1:9', ALL_PROXY: 'http://127.0.0.1:9' }
      });
      assertSuccess(result, 'local analysis should not require a reachable network service');
    }
  },
  {
    tag: '@claim:offline-cli',
    run: async () => {
      const output = join(await mkdtemp(join(tmpdir(), 'code-path-lens-offline-')), 'sample.html');
      const result = runLens(['demo', '--output', output], {
        env: { ...process.env, HTTP_PROXY: 'http://127.0.0.1:9', HTTPS_PROXY: 'http://127.0.0.1:9', ALL_PROXY: 'http://127.0.0.1:9' }
      });
      assertSuccess(result, 'bundled CLI demo should run with unreachable network proxies');
      assert((await stat(output)).size > 0, 'offline CLI demo did not write its output');
    }
  },
  {
    tag: '@claim:no-third-party-site',
    run: async () => {
      await withBrowser(async (browser, origin) => {
        const context = await browser.newContext();
        const page = await context.newPage();
        const requests = [];
        page.on('request', (request) => requests.push(new URL(request.url()).origin));
        await page.goto(`${origin}/demo/`, { waitUntil: 'networkidle' });
        assert(requests.every((requestOrigin) => requestOrigin === origin), `unexpected site request: ${requests.join(', ')}`);
        await context.close();
      });
    }
  },
  {
    tag: '@claim:offline-site',
    run: async () => {
      await withBrowser(async (browser, origin) => {
        const context = await browser.newContext();
        const page = await context.newPage();
        await page.goto(`${origin}/demo/`, { waitUntil: 'networkidle' });
        await page.evaluate(() => navigator.serviceWorker.ready);
        await page.reload({ waitUntil: 'networkidle' });
        await context.setOffline(true);
        await page.reload({ waitUntil: 'domcontentloaded' });
        await assert.doesNotReject(() => page.locator('h1').textContent());
        assert.equal(await page.locator('h1').textContent(), 'Inspect a sample code path');
        await context.close();
      });
    }
  },
  {
    tag: '@claim:exits-no-prompts',
    run: async () => {
      assert.equal(runLens(['trace', 'handle_order', '--root', sampleRoot, '--json']).status, 0);
      assert.equal(runLens(['trace', 'missing_symbol', '--root', sampleRoot, '--json']).status, 3);
      assert.equal(runLens(['trace', 'handle_order', '--root', sampleRoot, '--depth', '9', '--json']).status, 2);
      assert.equal(runLens(['trace', 'handle_order', '--root', sampleRoot, '--depth', '3', '--json']).status, 4);
    }
  },
  {
    tag: '@claim:cli-demo',
    run: async () => {
      const output = join(await mkdtemp(join(tmpdir(), 'code-path-lens-demo-')), 'review.html');
      const result = runLens(['demo', '--output', output]);
      assertSuccess(result, 'bundled demo failed');
      assert.match(result.stderr, /Bundled sample repository:/);
      assert.match(result.stderr, /Wrote \d+ nodes and \d+ edges to/);
      assert.match(await readFile(output, 'utf8'), /emit_receipt/);
    }
  },
  {
    tag: '@claim:site-demo-isolation',
    run: async () => {
      await withBrowser(async (browser, origin) => {
        const context = await browser.newContext();
        await context.addInitScript(() => localStorage.setItem('real:data', 'untouched'));
        const page = await context.newPage();
        await page.goto(`${origin}/demo/`, { waitUntil: 'networkidle' });
        assert.equal(await page.locator('.sample-banner strong').textContent(), 'Demo — sample data, nothing is saved');
        await page.getByRole('button', { name: 'validate' }).click();
        assert.equal(await page.locator('#demo-evidence-title').textContent(), 'validate');
        assert.equal(await page.evaluate(() => localStorage.getItem('real:data')), 'untouched');
        assert.equal(await page.evaluate(() => localStorage.getItem('demo:code-path-lens:session')), 'active');
        await page.getByRole('button', { name: 'Reset demo' }).click();
        assert.equal(await page.locator('#demo-evidence-title').textContent(), 'handle_order');
        assert.equal(await page.evaluate(() => localStorage.getItem('real:data')), 'untouched');
        await context.close();
      });
    }
  },
  {
    tag: '@claim:keyboard-viewer',
    run: async () => {
      await withBrowser(async (browser, origin) => {
        const context = await browser.newContext();
        const page = await context.newPage();
        await page.goto(`${origin}/demo/`, { waitUntil: 'networkidle' });
        await page.keyboard.press('/');
        assert.equal(await page.evaluate(() => document.activeElement.id), 'demo-filter');
        await page.keyboard.type('nothing matches');
        assert.equal(await page.locator('#demo-empty').isVisible(), true);
        await page.keyboard.press('Escape');
        assert.equal(await page.locator('#demo-empty').isVisible(), false);
        await page.getByRole('button', { name: 'handle_order' }).focus();
        await page.keyboard.press('ArrowRight');
        assert.notEqual(await page.evaluate(() => document.activeElement.textContent), 'handle_orderentry');
        await context.close();
      });
    }
  }
];

if (!existsSync(binary)) {
  throw new Error(`CLI binary is missing at ${binary}; run cargo build before claim tests`);
}

const selected = claimTag ? tests.filter((test) => test.tag === claimTag) : tests;
if (!selected.length) throw new Error(`no claim test matches ${claimTag}`);
for (const test of selected) {
  await test.run();
  console.log(`passed ${test.tag}`);
}
