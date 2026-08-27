const demoNodes = [
  { id: 'handle', label: 'handle_order', kind: 'entry', meta: 'Rust · src/orders.rs:18 · entry', reason: 'Requested entry symbol.', source: '18 │ fn handle_order(order: Order) {\n19 │     validate(&order)?;\n20 │     repository.insert(order)?;\n21 │     emit_receipt(order.id);\n22 │ }' },
  { id: 'validate', label: 'validate', kind: 'function', meta: 'Rust · src/orders.rs:31 · function', reason: 'Call expression at src/orders.rs:19.', source: '31 │ fn validate(order: &Order) -> Result<()> {\n32 │     ensure!(!order.items.is_empty());\n33 │     Ok(())\n34 │ }' },
  { id: 'caller', label: 'post_order', kind: 'function', meta: 'Rust · src/http.rs:44 · caller', reason: 'post_order calls handle_order at src/http.rs:48.', source: '44 │ async fn post_order(body: Json<Order>) {\n45 │     handle_order(body.0)\n46 │ }' },
  { id: 'order', label: 'Order', kind: 'type', meta: 'Rust · src/model.rs:7 · type', reason: 'Order appears in handle_order’s declaration.', source: ' 7 │ struct Order {\n 8 │     id: OrderId,\n 9 │     items: Vec<LineItem>,\n10 │ }' },
  { id: 'database', label: 'database I/O', kind: 'boundary', meta: 'src/orders.rs:20 · data boundary', reason: 'Call “insert” matched the database I/O boundary rule.', source: '' },
  { id: 'receipt', label: 'emit_receipt', kind: 'unresolved', meta: 'src/orders.rs:21 · unresolved call', reason: 'No matching declaration in scanned files. Kept visible rather than guessed.', source: '' }
];

const map = document.querySelector('#demo-map');
if (map) {
  const buttons = demoNodes.map((node) => {
    const button = document.createElement('button');
    button.type = 'button';
    button.className = 'demo-node';
    button.dataset.kind = node.kind;
    button.setAttribute('aria-pressed', 'false');
    const title = document.createElement('strong');
    const detail = document.createElement('small');
    title.textContent = node.label;
    detail.textContent = node.kind.replace('_', ' ');
    button.append(title, detail);
    const select = () => {
      buttons.forEach((item) => item.setAttribute('aria-pressed', String(item === button)));
      document.querySelector('#demo-evidence-title').textContent = node.label;
      document.querySelector('#demo-evidence-meta').textContent = node.meta;
      document.querySelector('#demo-evidence-reason').textContent = node.reason;
      const source = document.querySelector('#demo-evidence-source');
      source.hidden = !node.source;
      source.textContent = node.source;
    };
    button.addEventListener('click', select);
    button.addEventListener('keydown', (event) => {
      if (!['ArrowRight', 'ArrowDown', 'ArrowLeft', 'ArrowUp'].includes(event.key)) return;
      event.preventDefault();
      const visible = buttons.filter((item) => !item.hidden);
      const step = ['ArrowRight', 'ArrowDown'].includes(event.key) ? 1 : -1;
      visible[(visible.indexOf(button) + step + visible.length) % visible.length]?.focus();
    });
    map.append(button);
    return button;
  });
  buttons[0].setAttribute('aria-pressed', 'true');

  const filter = document.querySelector('#demo-filter');
  const applyFilter = () => {
    const query = filter.value.trim().toLowerCase();
    let count = 0;
    demoNodes.forEach((node, index) => {
      const show = !query || `${node.label} ${node.kind} ${node.reason}`.toLowerCase().includes(query);
      buttons[index].hidden = !show;
      if (show) count += 1;
    });
    document.querySelector('#demo-empty').hidden = count !== 0;
  };
  filter.addEventListener('input', applyFilter);
  document.addEventListener('keydown', (event) => {
    if (event.key === '/' && document.activeElement !== filter) {
      event.preventDefault();
      filter.focus();
    }
    if (event.key === 'Escape' && document.activeElement === filter) {
      filter.value = '';
      applyFilter();
      filter.blur();
    }
  });
}

const copyButton = document.querySelector('#copy-command');
copyButton?.addEventListener('click', async () => {
  const command = document.querySelector('#install-command').textContent;
  try {
    await navigator.clipboard.writeText(command);
    copyButton.textContent = 'Copied';
  } catch {
    copyButton.textContent = 'Select text';
    const selection = window.getSelection();
    const range = document.createRange();
    range.selectNodeContents(document.querySelector('#install-command'));
    selection.removeAllRanges();
    selection.addRange(range);
  }
  setTimeout(() => { copyButton.textContent = 'Copy'; }, 1800);
});

const LICENSE_KEY = 'sb_license:code-path-lens';
const VERDICT_KEY = `${LICENSE_KEY}:verdict`;
const API = 'https://pilot-api.sociobot.in/api/v1';
const licenseNote = document.querySelector('#license-note');
const tokenInput = document.querySelector('#license-token');

function setLicenseState(message, state = '') {
  if (!licenseNote) return;
  licenseNote.textContent = message;
  licenseNote.dataset.state = state;
}

async function verifyLicense(token, force = false) {
  let cached = null;
  try {
    cached = JSON.parse(localStorage.getItem(VERDICT_KEY) || 'null');
  } catch {
    localStorage.removeItem(VERDICT_KEY);
  }
  const fresh = cached && cached.token === token && Date.now() - cached.checkedAt < 86_400_000;
  if (!force && fresh) {
    setLicenseState(cached.valid ? 'Pro is unlocked on this browser.' : 'This license is no longer active.', cached.valid ? 'success' : 'error');
    return cached.valid;
  }
  if (!navigator.onLine) {
    if (cached?.valid && cached.token === token) setLicenseState('Pro is unlocked from the last verified license. Verification will resume online.', 'success');
    else setLicenseState('You’re offline. The free experience still works; reconnect to verify this license.');
    return Boolean(cached?.valid && cached.token === token);
  }
  setLicenseState('Checking the license…');
  try {
    const response = await fetch(`${API}/products/code-path-lens/verify?license=${encodeURIComponent(token)}`, { headers: { accept: 'application/json' } });
    if (!response.ok) throw new Error(`Verification returned ${response.status}`);
    const result = await response.json();
    const verdict = { token, valid: result.valid === true, checkedAt: Date.now() };
    localStorage.setItem(VERDICT_KEY, JSON.stringify(verdict));
    setLicenseState(verdict.valid ? 'Pro is unlocked on this browser.' : 'This license is no longer active. You can purchase a new license below.', verdict.valid ? 'success' : 'error');
    return verdict.valid;
  } catch {
    setLicenseState('License verification could not connect. The free experience still works; try again when online.', 'error');
    return false;
  }
}

const query = new URLSearchParams(location.search);
const returnedLicense = query.get('license');
if (returnedLicense) {
  localStorage.setItem(LICENSE_KEY, returnedLicense);
  query.delete('license');
  const clean = `${location.pathname}${query.size ? `?${query}` : ''}${location.hash}`;
  history.replaceState({}, '', clean);
}
const storedLicense = returnedLicense || localStorage.getItem(LICENSE_KEY);
if (storedLicense && tokenInput) {
  tokenInput.value = storedLicense;
  verifyLicense(storedLicense);
}

document.querySelector('#license-form')?.addEventListener('submit', async (event) => {
  event.preventDefault();
  const token = tokenInput.value.trim();
  if (!token) {
    setLicenseState('Paste the license token from your receipt.', 'error');
    tokenInput.focus();
    return;
  }
  localStorage.setItem(LICENSE_KEY, token);
  await verifyLicense(token, true);
});

function updateOffline() {
  const banner = document.querySelector('#offline');
  if (banner) banner.hidden = navigator.onLine;
}
window.addEventListener('online', updateOffline);
window.addEventListener('offline', updateOffline);
updateOffline();

if ('serviceWorker' in navigator) {
  window.addEventListener('load', () => navigator.serviceWorker.register('/sw.js').catch(() => {}));
}
