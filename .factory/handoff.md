# Handoff — Code Path Lens 0.1.0 repair 2

## Status

Implementation candidate: `378180b12c9745185b1adad6f081d05a4cc31bff`.

This repair resolves the product-controlled sample, claims, copy, route,
metadata, shared-skeleton, touch-target, and 404 findings from review 1. It is
deployed at <https://code-path-lens.sociobot.in>. The factory billing product
is still not enabled at the external Sociobot billing service. The site no
longer shows the broken checkout or an unavailable paid offer. That external
registration remains the only known dependency before a paid tier can return.

The current report/handoff is committed after the implementation candidate;
use `git log -- .factory/handoff.md` to identify its separate documentation
commit SHA.

## What changed

- Added `code-path-lens demo`, which materializes the checked-in
  `examples/checkout-sample/` repository in a temporary directory, runs the
  real analyzer, writes a populated review page, and reports its location.
- Added `/demo/` with a real populated sample, source-line links, persistent
  **“Demo — sample data, nothing is saved”** banner, **Reset demo**, and
  **Start for real**. It uses only `demo:code-path-lens:session`; real browser
  data is neither read nor written in demo mode.
- Added `.factory/demo.md`, `.factory/claims.json`, 16 isolated claim checks,
  and a test runner that accepts every documented `npm test -- --grep
  @claim:<id>` command.
- Rewrote the first screen in plain words: job, audience, and one-click sample
  action are visible before scrolling. Added the copy and terminology audit.
- Added consistent headers and footers, route titles, canonical/OG/Twitter
  metadata, a product-derived 1200×630 social card, 180 px touch icon, demo
  sitemap entry, terminal recording, and 44 px target areas.
- Added a designed `404.html`; unknown paths now return an HTTP 404 with that
  page. Removed the multipage site's landing-page fallback.
- Kept the existing local-first analyzer and free outputs. The viewer,
  `.gitignore` handling, generated/vendor exclusions, output formats, explicit
  bounds, and static-approximation notice remain intact.

## Verification

From the documented setup, the following passed on 2026-09-05 UTC:

```sh
npm ci
npm test
npm run build
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty
npm audit --audit-level=high
```

- `npm test` ran 7 Rust unit tests, 2 CLI integration tests, static-site
  checks, and all 16 declared claim commands. Each claim starts from the
  shipped sample or a fresh browser/temporary repository. Browser claims use
  a dedicated context for offline reloads.
- `cargo package --allow-dirty` verified 26 package files (142.9 KiB unpacked,
  40.1 KiB compressed). A clean consumer install from
  `target/package/code-path-lens-0.1.0` ran `--version`, `languages`, and
  `demo --output`; it wrote a populated HTML review page.
- `npm run build` produced `dist/bin/code-path-lens` and `dist/site/`. Initial
  JavaScript is 6.07 KB (2.03 KB gzip); the hero is 75.3 KB; no webfonts ship.
- `/opt/fleet/lib/verify-url.sh` passed against the public URL: title, language,
  h1, main landmark, image alt text, and console checks all passed.
- `LENS_TEST_URL=https://code-path-lens.sociobot.in npm run test:a11y` found
  zero axe violations for `/`, `/demo/`, `/privacy/`, `/terms/`, and `/404.html`
  at 1366×900 and 390×844.
- Live Lighthouse (Chromium with `--disable-dev-shm-usage --disable-gpu`) scored
  Performance 100, Accessibility 100, Best Practices 100, and SEO 100. FCP
  was 0.9 s, LCP 1.2 s, and CLS 0.

## Live deployment evidence

`/opt/fleet/lib/deploy-static.sh code-path-lens dist/site` completed on
2026-09-05 UTC. The live root and hashed application asset match this
implementation byte-for-byte:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `be03305eefe18566cff3d536f6294267d0b8debaf854628f8798f68c4e9ea413` |
| `assets/app-bH3Yy194.js` | `54ef2a834122058879d123deafd3e922389187afbe6038395c957603c1723cdc` |

Live HTML is short-lived; hashed assets are `public, max-age=31536000,
immutable`; `sw.js` is `no-cache`. CSP limits connections to the same origin;
frame denial, nosniff, strict referrer policy, COOP, CORP, and permissions
policy are live.

Fresh live desktop and phone browser contexts confirmed before scrolling:

- **Job:** trace a bounded path around a code symbol.
- **Audience:** developers reviewing unfamiliar code.
- **First action:** Try it with sample data.

Both contexts clicked that action, saw the persistent sample banner, selected
`validate`, reset to `handle_order`, retained a seeded real-data key unchanged,
and made no third-party requests or console/page errors. **Start for real**
removed the demo marker while retaining that real-data key. The visible touch
targets measured at least 44 px. A service-worker-controlled live `/demo/`
reload also succeeded while offline, with the demo title and h1 intact.

Unknown `/no-such-page` returns HTTP 404 with title **“Page not found — Code
Path Lens”** and a home link. Chromium records the expected failed navigation
resource for that deliberate 404; it is not a page error or broken route.

Evidence files are under `/work/.evidence/`, including `cpl-live-verify/`,
desktop/phone first-screen and demo screenshots, and
`cpl-live-lighthouse-retry.json`.

## Review-history disposition

| Earlier finding | Current disposition |
| --- | --- |
| Broken TLS/live deployment | Resolved by the existing static app; current deployment returns valid HTTPS 200 and matching artifacts. |
| Pilot billing base and weak cache/security headers | Site paid UI was removed until the billing product exists. Static cache and security headers remain live and verified. The CLI retains its production verification default for a future enabled license. |
| Missing CLI/site sample mode | Resolved by the bundled CLI sample, `/demo/`, sample label, reset/exit actions, documentation, example source, source links, and terminal recording. |
| Missing claims manifest and tagged checks | Resolved by 16 manifest entries and clean-sandbox outcome tests. |
| Plain-language first screen and headings | Resolved by the job headline, named audience, primary sample action, plain section headings, and `.factory/copy-audit.md`. |
| Missing deliberate 404 | Resolved by `404.html`, response override, and live HTTP 404 verification. |
| Metadata/shared skeleton gaps | Resolved on all real routes; sitemap includes `/demo/`. |
| Small touch targets | Resolved; visible interactive areas are at least 44 px. |

## Known gap and next step

The external endpoint
`https://api.sociobot.in/api/v1/products/code-path-lens/checkout` returned its
documented error that the factory product is not enabled. No credentials or
mock payment path were added. A factory operator must register/enable the
product and supply its real return configuration. After that, restore the
transparent $29 one-time paid UI, then verify checkout, a real return token,
restore on a second device, valid unlock, and revoked-token recovery. Do not
claim a paid tier before those external checks pass.
