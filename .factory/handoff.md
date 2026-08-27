# Handoff — Code Path Lens 0.1.0

## What shipped

- A Rust single-binary CLI with a small `trace`/`languages` command surface.
- Tree-sitter adapters for Rust, TypeScript/TSX, JavaScript/JSX, Python, and Go.
- Deterministic caller/callee traversal with explicit depth and node limits,
  source locations and excerpts, referenced types, heuristic named data
  boundaries, generated-code exclusions, `.gitignore` support, and unresolved
  calls that remain visible.
- Self-contained, keyboard-operable HTML graph output with typed link lines,
  evidence panel, source links, filter, paper/night themes, warnings, and a
  text ledger. Stable JSON and Graphviz DOT outputs are also included.
- Exit codes: 0 success, 1 analysis/I/O, 2 CLI usage, 3 missing/ambiguous
  symbol, and 4 paid-limit/license failure. The CLI never prompts or sends
  source over the network.
- A responsive Vite landing/docs site in `dist/site/`, including an interactive
  example, install instructions, offline shell, empty/error/offline states,
  privacy and terms pages, and the Sociobot one-time purchase/restore flow.
- Free caps are depth 2 / 40 nodes. The $29 Pro license raises them to depth 8
  / 250 nodes; all formats, source evidence, accessibility, and export remain
  free. The page saves `sb_license:code-path-lens`, strips returned tokens from
  the URL, caches verification for one day, and reconciles without blocking the
  free first paint.

## Build and verification

Run from a clean clone:

```sh
npm install
npm test
npm run build
```

The exact factory build command is `npm run build`. It writes the release binary
to `dist/bin/code-path-lens` and the deployable static root to `dist/site/`
(`dist/site/index.html` exists). The site-only command required by the work
order is `npm run build:site`.

Verification completed on 27 August 2026:

- `npm test`: 6 unit tests + 1 end-to-end CLI integration test passed; Vite
  production build and static page assertions passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `npm audit --audit-level=high`: 0 vulnerabilities.
- `cargo package --allow-dirty`: packaged 20 files, 123.4 KiB uncompressed /
  35.0 KiB compressed, and the package verification build passed. Do not
  publish here; registry credentials remain with the factory.
- Real self-analysis: `trace main --root . --json` emitted 35 nodes / 35 edges
  covering entry, function, type, data-boundary, and unresolved node kinds.
  Its generated HTML loaded with zero console errors and passed Axe at 1366 px
  and 390 px.
- Factory URL verifier against the production preview: HTTP 200, 525 ms load,
  title and `lang`, one h1, main landmark, complete image alt, labeled buttons,
  and zero console errors.
- Playwright + axe: 0 total violations at 1366×900 and 390×844 for both the
  product site and generated viewer.
- Lighthouse mobile production preview: Performance 100, Accessibility 100,
  Best Practices 100, SEO 100; FCP 0.8 s, LCP 1.4 s, CLS 0, TBT 0 ms.
- Initial site payload: 6.35 KB JS (2.68 KB gzip), 9.6 KB inline CSS (about
  3 KB gzip), no webfonts, and a 73.6 KB WebP hero. All are below budget.

## Original asset provenance

`site/public/assets/hero-field-notebook.webp` was generated with
`/opt/fleet/lib/gen-image.sh`, deployment `factory-image`, at 1536×1024/high,
then converted locally to WebP quality 72. The original generated PNG was
discarded after visual inspection; the shipped WebP is 73.6 KB. The exact
prompt and generator settings are retained beside it in
`hero-field-notebook.webp.json`. The prompt asks for an impossible midnight
surveyor's paper landscape, a coral path and brass pins across indigo source
strata, a moss specimen label, and a cobalt unresolved doorway, with no text,
logos, people, computers, code, UI, or watermark. The output is original to
this product and used under the generator service terms. Icons and graph marks
are hand-authored SVG/CSS.

## Known gaps and release steps

- This is honest static approximation, not control-flow or runtime-complete
  analysis. Name resolution is intentionally conservative: overloads and
  duplicate method names become ambiguous/unresolved, and dynamic dispatch,
  reflection, macro expansion, build tags, imports, and dependency bodies are
  not resolved. JavaScript uses the TypeScript/TSX grammar fallback.
- Data boundaries are labeled by visible call-name rules and carry the exact
  call-site evidence. They are signals for review, not a taint analysis.
- The factory must register the paid product and replace the staging
  `https://pilot-api.sociobot.in` base with production at release. No product
  ID or payment-provider integration is embedded.
- Release binaries still need the factory's target matrix/signing workflow.
  The local Linux release binary is 9.2 MB.
