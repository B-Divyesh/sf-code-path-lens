# Independent verification 3 — PASS

**Work order:** `code-path-lens-verify-3`  
**Candidate:** `28d7f03615cee3f2602e76ab81c87413df362a3b`  
**Public URL:** <https://code-path-lens.sociobot.in/>  
**Verified:** 2026-08-28 UTC

## Verdict

**PASS — release-ready.** The candidate builds and packages from a clean checkout, the installed consumer CLI performs the bounded local analysis described in the brief, the landing page and generated viewer work at desktop and 390 px widths, and the live site is a byte-identical deployment of the candidate's production static output.

| Severity | Defects |
| --- | --- |
| Blocker | None |
| High | None |
| Medium | None |
| Low | None |

## Clean checkout and production build

A new detached clone at exactly the candidate SHA (`/tmp/code-path-lens-qa-Zrhpeh`) began clean. `npm ci` installed 20 packages and reported 0 vulnerabilities. The following all passed:

```sh
cargo test
cargo clippy --all-targets --all-features -- -D warnings
npm test
npm run build
cargo package --allow-dirty
npm audit --audit-level=high
```

- `cargo test`: 7 unit tests plus 1 CLI integration test passed.
- Strict Clippy emitted no warnings; the dependency audit found 0 vulnerabilities.
- `npm test` rebuilt the site and passed endpoint, legal, semantic, cache-policy, and asset-budget assertions.
- `npm run build` produced `dist/bin/code-path-lens` and `dist/site/`.
- `cargo package --allow-dirty` verified a package of 20 files, 124.5 KiB unpacked / 35.3 KiB compressed.
- The initial site JavaScript is 6,346 bytes (2,680 bytes gzip), no webfont ships, and the WebP hero is 75,322 bytes: within all stated static budgets.

## Installed consumer CLI

The package was installed into a clean consumer prefix, not run from the checkout:

```sh
cargo install --path target/package/code-path-lens-0.1.0 --root /tmp/cpl-consumer-AYunYX
```

Only the installed binary was exercised. `--version` reported 0.1.0; `--help` documented `trace` and `languages`; and `languages` listed Rust, TypeScript/JavaScript, Python, and Go with explicit fallback behavior.

An independent Git fixture contained `post_order -> handle_order -> validate`, `Order`, `database_insert`, and `unknown_runtime_target`, plus a generated source file and a `.gitignore`d source file. A normal JSON trace emitted all required node kinds (`entry`, `function`, `type`, `data_boundary`, `unresolved`) and caller/callee/type/boundary/unresolved edges. It emitted the static-approximation notice, skipped the generated file by default, and did not scan the ignored file. `--include-generated` changed parsed files from 1 to 2 and skipped-generated from 1 to 0.

Boundary and recovery behavior passed:

- `--depth 0 --max-nodes 1` gave one node, zero edges, `truncated: true`, and explicit node-budget plus static-approximation warnings.
- A missing symbol exited **3**; invalid `--depth 9` exited **2**; a paid bound without a license exited **4**, each with useful recovery copy.
- The default HTML output is a source-linked, filterable evidence viewer.

Source review shows the free analysis path has no HTTP client invocation; the only Rust HTTP client is in the paid-license verification branch. No source, file name, graph, command argument, analytics, tracking, remote-index, CDN font, or third-party script request was observed.

## Browser, accessibility, privacy, and PWA

The exact local `dist/site/`, live URL, and generated package viewer were exercised in Playwright Chromium 1.58.2 at 1366×900 and 390×844.

- `npm run test:a11y` against local and live URLs reported **0 total axe findings** (therefore 0 serious/critical) at both widths. The generated viewer also had 0 axe findings at both widths.
- The landing demo and generated viewer selected nodes, reached their empty filter state, recovered with Escape, focused filtering with `/`, and moved nodes with arrows. First Tab reached the skip link; it and sampled nodes had solid 3 px focus outlines. There was no horizontal page overflow at 390 px, each document had one `h1`, and there were no console/page errors.
- Reduced-motion CSS computed `scroll-behavior: auto` and transition/animation duration `0.00001s` for the live and generated views.
- An isolated `?license=qa-invalid-token` test removed the token from the URL, stored it only in isolated browser local storage, showed invalid-license recovery copy, and requested exactly `https://api.sociobot.in/api/v1/products/code-path-lens/verify?...` with no errors. Normal first loads contacted only the product origin.
- The deployed worker controlled a reload and an offline reload retained the title without errors. An isolated same-scope registration at `/sw.js?qa-update=20260828` became active immediately; the worker's `skipWaiting`/`clients.claim` update path and `no-cache` response policy were verified.

Lighthouse 13.4.1 on the live URL reported **Performance 100** and **Accessibility 100**: FCP **1.3 s**, LCP **1.3 s**, TBT **0 ms**, CLS **0**.

## Live deployment, headers, and cache policy

Normal TLS validation succeeded: the certificate subject is `CN = code-path-lens.sociobot.in` and hostname verification returned `0 (ok)`. Live `index.html`, application JS, and `sw.js` are byte-identical to the fresh candidate build:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `c9b131d3b3a4520d603085e81b264287c5754acfb0ac535840693047f7de1ab9` |
| `assets/index-DNl_AHsP.js` | `167d03bbf27426dd81a6dc7ca815caeb86e359356b71d8f441e393ccf41ec63f` |
| `sw.js` | `b0572e8bfcdf220e8ae9ad2bcd627dac3c16f56c7bea00da057229d507d6b7e4` |

`28d7f03` differs from the repair source commit only in factory handoff text, so byte-identical production artifacts establish that the deployed product matches the tested candidate.

`/`, `/privacy/`, and `/terms/` have short revalidation caching; JS and WebP assets have correct MIME types and `public, max-age=31536000, immutable`; and `/sw.js` has JavaScript MIME and `no-cache`. Live responses provide HSTS, nosniff, strict-origin referrer policy, a self-only CSP permitting only `https://api.sociobot.in` in `connect-src`, `X-Frame-Options: DENY`, COOP, CORP, and a restrictive Permissions-Policy.

## Remaining work

None for this candidate. The factory owns deployment and registry credentials; the ready-to-publish verification remains `cargo package --allow-dirty`.
