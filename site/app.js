const demoNodes = [
  {
    id: 'handle', label: 'handle_order', kind: 'entry',
    meta: 'Rust · examples/checkout-sample/src/orders.rs:7 · entry',
    reason: 'Requested entry symbol.',
    link: 'https://github.com/B-Divyesh/sf-code-path-lens/blob/main/examples/checkout-sample/src/orders.rs#L7-L11',
    source: '7 │ pub fn handle_order(order: Order) {\n8 │     validate(&order);\n9 │     database_insert(order);\n10 │     emit_receipt();\n11 │ }'
  },
  {
    id: 'validate', label: 'validate', kind: 'function',
    meta: 'Rust · examples/checkout-sample/src/orders.rs:5 · function',
    reason: 'Call expression at examples/checkout-sample/src/orders.rs:8.',
    link: 'https://github.com/B-Divyesh/sf-code-path-lens/blob/main/examples/checkout-sample/src/orders.rs#L5',
    source: '5 │ pub fn validate(_order: &Order) {}'
  },
  {
    id: 'caller', label: 'post_order', kind: 'function',
    meta: 'Rust · examples/checkout-sample/src/http.rs:4 · caller',
    reason: 'post_order calls handle_order at examples/checkout-sample/src/http.rs:5.',
    link: 'https://github.com/B-Divyesh/sf-code-path-lens/blob/main/examples/checkout-sample/src/http.rs#L4-L6',
    source: '4 │ pub fn post_order(order: Order) {\n5 │     handle_order(order)\n6 │ }'
  },
  {
    id: 'order', label: 'Order', kind: 'type',
    meta: 'Rust · examples/checkout-sample/src/model.rs:1 · type',
    reason: 'Order appears in handle_order’s declaration.',
    link: 'https://github.com/B-Divyesh/sf-code-path-lens/blob/main/examples/checkout-sample/src/model.rs#L1-L4',
    source: '1 │ pub struct Order {\n2 │     pub id: String,\n3 │     pub items: Vec<String>,\n4 │ }'
  },
  {
    id: 'insert', label: 'database_insert', kind: 'function',
    meta: 'Rust · examples/checkout-sample/src/orders.rs:3 · function',
    reason: 'Call expression at examples/checkout-sample/src/orders.rs:9.',
    link: 'https://github.com/B-Divyesh/sf-code-path-lens/blob/main/examples/checkout-sample/src/orders.rs#L3',
    source: '3 │ pub fn database_insert(_order: Order) {}'
  },
  {
    id: 'database', label: 'database I/O', kind: 'boundary',
    meta: 'examples/checkout-sample/src/orders.rs:9 · data boundary',
    reason: 'Call “database_insert” matched the database I/O boundary rule.',
    link: 'https://github.com/B-Divyesh/sf-code-path-lens/blob/main/examples/checkout-sample/src/orders.rs#L9',
    source: ''
  },
  {
    id: 'receipt', label: 'emit_receipt', kind: 'unresolved',
    meta: 'examples/checkout-sample/src/orders.rs:10 · unresolved call',
    reason: 'No matching declaration in scanned files. Kept visible rather than guessed.',
    link: 'https://github.com/B-Divyesh/sf-code-path-lens/blob/main/examples/checkout-sample/src/orders.rs#L10',
    source: ''
  }
];

const map = document.querySelector('#demo-map');
if (map) {
  const title = document.querySelector('#demo-evidence-title');
  const meta = document.querySelector('#demo-evidence-meta');
  const reason = document.querySelector('#demo-evidence-reason');
  const source = document.querySelector('#demo-evidence-source');
  const sourceLink = document.querySelector('#demo-evidence-link');
  const buttons = demoNodes.map((node) => {
    const button = document.createElement('button');
    button.type = 'button';
    button.className = 'demo-node';
    button.dataset.kind = node.kind;
    button.setAttribute('aria-pressed', 'false');
    const label = document.createElement('strong');
    const detail = document.createElement('small');
    label.textContent = node.label;
    detail.textContent = node.kind.replace('_', ' ');
    button.append(label, detail);
    const select = () => {
      buttons.forEach((item) => item.setAttribute('aria-pressed', String(item === button)));
      title.textContent = node.label;
      meta.textContent = node.meta;
      reason.textContent = node.reason;
      source.hidden = !node.source;
      source.textContent = node.source;
      sourceLink.href = node.link;
    };
    button.addEventListener('click', select);
    button.addEventListener('keydown', (event) => {
      if (!['ArrowRight', 'ArrowDown', 'ArrowLeft', 'ArrowUp'].includes(event.key)) return;
      event.preventDefault();
      const visible = buttons.filter((item) => !item.hidden);
      if (!visible.length) return;
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

  document.querySelector('#reset-demo')?.addEventListener('click', () => {
    localStorage.removeItem('demo:code-path-lens:session');
    localStorage.setItem('demo:code-path-lens:session', 'active');
    filter.value = '';
    applyFilter();
    buttons[0].click();
    filter.focus();
  });
}

if (document.body.dataset.demo === 'true') {
  localStorage.setItem('demo:code-path-lens:session', 'active');
  document.querySelector('.banner-link')?.addEventListener('click', () => {
    localStorage.removeItem('demo:code-path-lens:session');
  });
}

const copyButton = document.querySelector('#copy-command');
copyButton?.addEventListener('click', async () => {
  const command = document.querySelector('#install-command').textContent;
  try {
    await navigator.clipboard.writeText(command);
    copyButton.textContent = 'Copied';
  } catch {
    copyButton.textContent = 'Select command';
    const selection = window.getSelection();
    const range = document.createRange();
    range.selectNodeContents(document.querySelector('#install-command'));
    selection.removeAllRanges();
    selection.addRange(range);
  }
  setTimeout(() => { copyButton.textContent = 'Copy command'; }, 1800);
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
