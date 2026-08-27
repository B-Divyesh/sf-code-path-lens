# Independent verification — FAIL

**Work order:** `code-path-lens-verify-1`  
**Candidate:** `85f6581b5525ea8a8f53796a2cac2e58fcd661be` (`origin/main` resolved to the same SHA)  
**Public URL checked:** `https://code-path-lens.sociobot.in/`  
**Date:** 2026-08-27 UTC

## Verdict

**FAIL — do not release.** The local candidate is buildable and the core CLI
works, but the required public deployment does not serve Code Path Lens. A
normal TLS client rejects the hostname certificate; bypassing certificate
verification returns Azure's `404 Site Not Found`, not the candidate. It is
therefore impossible to confirm the live deployment, its browser security
headers, cache policy, or deployed build identity.

## Release-blocking defects

### BLOCKER — public URL has no valid Code Path Lens deployment

Fresh evidence on 2026-08-27:

- DNS resolved `code-path-lens.sociobot.in` to `68.220.237.27`
  (`waws-prod-bn1-395a8892.sip.p.azurewebsites.windows.net`).
- `curl --proto '=https' --tlsv1.2 https://code-path-lens.sociobot.in/`
  failed before HTTP with: `SSL: no alternative certificate subject name
  matches target host name`.
- SNI certificate subject was
  `*.msha-slice-7-eus2-0-ase.p.azurewebsites.net`; its SAN list contains only
  Azure names and does not contain `code-path-lens.sociobot.in`.
- Diagnostic-only `curl -k` returned `HTTP/1.1 404 Site Not Found`,
  `Content-Type: text/html`, 2667 bytes. This was not treated as a valid
  successful request.

This blocks the acceptance requirement to confirm the live candidate and
means real users cannot safely load the product.

### HIGH — production artifact is hard-coded to the pilot billing service

The candidate's site and CLI use
`https://pilot-api.sociobot.in/api/v1` (`site/app.js`, `src/license.rs`) for
checkout/license verification. The product contract requires the production
Sociobot billing API for a released paid feature. There is no build-time
configuration in this repository that changes this value. This must be changed
or supplied by a verified release build before a production launch.

## Local candidate evidence

The checkout began clean at the candidate SHA. `npm ci` completed with 0 audit
vulnerabilities. The following all passed:

```text
cargo test
  6 unit tests + 1 CLI integration test passed
cargo clippy --all-targets --all-features -- -D warnings
  passed
npm test
  Rust tests passed; Vite production build passed; static page tests passed
npm run build
  passed; produced dist/bin/code-path-lens and dist/site/
cargo package --allow-dirty
  passed package verification; 20 files, 123.5 KiB / 35.1 KiB compressed
npm audit --audit-level=high
  0 vulnerabilities
```

`dist/site/` budget evidence: initial JavaScript is 6,352 bytes (2,680 bytes
gzip), no shipped webfont files, and the hero WebP is 75,322 bytes. All are
within the stated 200 KB JS / 120 KB fonts / 300 KB hero limits. The build
inlines CSS into HTML, so there is no separate CSS payload.

## End-to-end CLI/package evidence

I copied Cargo's packaged source to a fresh temporary consumer directory,
installed it with `cargo install --path <package-copy> --root <clean-prefix>`,
then invoked only `<clean-prefix>/bin/code-path-lens`.

- `--version` reported `code-path-lens 0.1.0`; `--help` and `languages` gave
  the documented command surface and adapter fallbacks.
- A Rust fixture with `handle -> validate`, `Order`, `db_insert`, and an
  unknown call produced entry/function/type/data-boundary/unresolved nodes and
  calls, type, boundary, unresolved, and caller edges.
- The fixture's `.gitignore` excluded an `ignored/` Rust file. A generated
  Rust file was skipped by default (`parsed_files: 2`,
  `skipped_generated: 1`) and included only with `--include-generated`
  (`parsed_files: 3`, `skipped_generated: 0`).
- A representative trace of this repository hit the deliberate free bound at
  40 nodes / 47 edges and emitted the truncation and static-approximation
  warnings; all five node kinds were present.
- Boundary case `--depth 0 --max-nodes 1` emitted one node, zero edges, and a
  visible truncation warning. Missing symbol exited 3; invalid `--depth 9`
  exited 2; paid bounds without a token exited 4.

The code path has no remote-index/telemetry call. Static review finds the
only Rust HTTP client in paid-license verification; an initial browser load
made no outbound requests. The website has expected external GitHub links;
license submission is designed to contact the billing endpoint and sends the
token only. No source is sent by the tested free CLI flow.

## Browser, accessibility, offline, and privacy evidence

The exact `dist/site/` artifact was served locally and exercised in Chromium
at 1366×900 and 390×844.

- `npm run test:a11y` against the production build: Axe found **0 total / 0
  serious-or-critical** violations at both viewports.
- The landing demo selected evidence, filtered to its empty state, recovered
  with Escape, and moved nodes with arrow keys. Desktop and mobile had no
  document horizontal overflow and no console/page errors.
- Keyboard Tab traversal reached the skip link, navigation, controls, filter,
  and nodes; each sampled focus target had the designed solid 3px ochre focus
  outline. Reduced motion changed scroll behavior to `auto` and control
  transitions to 0.01 ms.
- A generated CLI HTML viewer was separately tested at both viewports. It had
  0 Axe findings, node selection/filter-empty/recovery/arrow navigation
  worked, focus was visible, reduced motion was 0.01 ms, no console/page
  errors occurred, and document width stayed within 390px.
- The landing service worker registered and controlled a reloaded page.
  Reloading while offline retained the page title with no page errors. Its
  `skipWaiting`/`clients.claim` update path was inspected; no separate live
  release version was available to test an over-the-air update.
- Local preview headers are development-server headers only (`Cache-Control:
  no-cache`); they are not deployment evidence. The broken public URL prevents
  a valid CSP, HSTS, MIME, cache, or immutable-asset-header assessment.

Lighthouse was attempted against the local production build with the supplied
Chromium. The container's Lighthouse launcher either could not connect to
Chrome or the tab crashed, so no score is claimed.

## Required next steps

1. Bind a certificate containing `code-path-lens.sociobot.in` and deploy
   `dist/site/` so normal HTTPS returns the candidate, then rerun live URL,
   headers/caching, and deployment-identity verification.
2. Configure the released CLI/site to use `https://api.sociobot.in/api/v1`,
   not the pilot API; verify checkout and a token-only license check on the
   released URL.
3. Publish cache/security response policy with the deploy (including
   immutable hashed assets) and rerun Lighthouse in a stable Chrome runtime.

