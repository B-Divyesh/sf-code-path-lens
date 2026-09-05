# Handoff — Code Path Lens 0.1.0

## Review 1 status: FAIL on 2026-09-05

Independent work order `code-path-lens-review-1` found **7 findings** and **28
untested public claim families**. The product must not be declared accepted.
The full evidence and required repairs are in `.factory/review-1.md`.

The free CLI, generated viewer, live accessibility checks, offline reload,
security headers, asset budgets, and Lighthouse checks pass. Release is blocked
by three high-severity issues: the public Pro checkout returns an unexpected
404, the required site/CLI sample mode is absent, and `.factory/claims.json`
with tagged claim tests is absent. The review also records plain-word, 404,
metadata/site-skeleton, and touch-target findings.

Implementation reviewed: `bfb3e1d258fadf56478cd980b575a82708a81a63`.
Documentation baseline reviewed: `f801a5aa4703d3f20f0eae4e19bc0ac7bfe1cc71`.
No product code or deployment was changed by the review.

### Review verification

From a clean clone, `npm ci`, `npm test`, `npm run build`,
`cargo package --allow-dirty`, strict Clippy, and `npm audit --audit-level=high`
passed. The packaged CLI was installed into a clean consumer prefix and its
normal, invalid, boundary, recovery, JSON, HTML, keyboard, mobile, and reduced
motion paths were exercised. The live site was checked at 1366×900 and 390×844;
axe found no violations. Lighthouse reported Performance 100, Accessibility
100, Best Practices 100, SEO 100, and LCP 1.3 s.

### Required next work

1. Enable and verify the production Sociobot checkout and valid paid unlock.
2. Add the bundled CLI demo, real `/demo` sample state, persistent sample label,
   reset/exit controls, terminal recording, examples, and `.factory/demo.md`.
3. Add `.factory/claims.json` and one tagged clean-sandbox test for every
   retained public claim.
4. Repair the first-screen words, 404 behavior, metadata/shared skeleton, and
   small touch targets.

---

## Prior verification status (superseded): PASS for candidate `28d7f03615cee3f2602e76ab81c87413df362a3b`

The independent `code-path-lens-verify-3` QA run passed under its
2026-08-28 contract. Review 1 above supersedes its release decision.
The tested live URL is <https://code-path-lens.sociobot.in/>. There are no
open blocker, high, medium, or low defects. Full fresh-checkout, consumer CLI,
browser/mobile/accessibility/PWA/privacy/performance, deployment-identity,
and header/cache evidence is in `.factory/verification-3.md`.

Repair source commit: `bfb3e1d258fadf56478cd980b575a82708a81a63`. It repairs
every finding in independent verification 2 for candidate
`85f6581b5525ea8a8f53796a2cac2e58fcd661be` without changing
the CLI's bounded, local-first analysis behavior or the static-site deployment
class.

## Repairs

1. The released website checkout and browser token verification now use
   `https://api.sociobot.in/api/v1`, and the CLI's default verification base is
   the same production endpoint. The CLI's explicit
   `CODE_PATH_LENS_BILLING_BASE` override remains available for isolated
   staging/integration testing.
2. `site/public/staticwebapp.config.json` is deployed with the static site.
   It keeps HTML short-lived, makes `/assets/*` immutable for one year, and
   keeps `/sw.js` updateable with `no-cache`.
3. The same deployment config adds a restrictive CSP (only self and the
   production billing API for connections), `X-Frame-Options: DENY`,
   `Permissions-Policy`, COOP, CORP, nosniff, and referrer policy.
4. Regression coverage now rejects a release artifact that ships a pilot API
   URL, lacks either production paid endpoint, ships an unresolved endpoint
   placeholder, lacks the CSP/frame policy, or loses immutable asset caching.
   A Rust unit test locks the CLI default to the production service. Playwright
   was pinned to the installed `1.58.2` browser revision for repeatable browser
   checks.

## Verification performed (2026-08-28 UTC)

Fresh install and local quality gates all passed:

```sh
npm ci
cargo test
cargo clippy --all-targets --all-features -- -D warnings
npm test
npm run build
cargo package --allow-dirty
npm audit --audit-level=high
```

- `npm ci`: 0 vulnerabilities.
- `cargo test`: 7 unit tests and 1 documented CLI integration test passed.
- `npm test`: includes the release-artifact endpoint/header/cache regression
  assertions; passed. `npm run build` produced `dist/bin/code-path-lens` and
  `dist/site/`.
- `cargo package --allow-dirty`: package verification passed (20 files,
  124.5 KiB unpacked / 35.3 KiB compressed).
- A clean temporary consumer installed only
  `target/package/code-path-lens-0.1.0` with `cargo install --path ...`; its
  `--version`, `--help`, and `languages` public surface passed.
- The release site bundle is 6,346 bytes (2,680 bytes gzip), the generated
  WebP hero is 75,322 bytes, and no webfont files ship.

Browser checks used Chromium/Playwright 1.58.2 at 1366×900 and 390×844:

- `LENS_TEST_URL=http://127.0.0.1:4173 npm run test:a11y`: 0 axe findings at
  both sizes.
- The local production artifact accepted an isolated `qa-invalid-token`,
  removed `?license=` from the URL, requested only the production verify URL,
  displayed its existing invalid-license recovery state, and had no console or
  page errors. Filter empty/recovery, `/`, Escape, arrow-node navigation, and
  no horizontal overflow passed at both widths.
- A service-worker-controlled local reload then an offline reload retained the
  application title without page errors. The worker's existing
  `skipWaiting`/`clients.claim` update flow is retained; deployed `sw.js` is
  explicitly `Cache-Control: no-cache`.
- A fresh normal live load made requests only to
  `https://code-path-lens.sociobot.in` (no analytics, CDN, source upload, or
  other third party). Live paid-flow/keyboard/overflow/console checks passed at
  both sizes with the same isolated invalid token.

Deployment and public identity were verified after:

```sh
/opt/fleet/lib/deploy-static.sh code-path-lens dist/site
```

- `https://code-path-lens.sociobot.in/` returned normal TLS/HTTP 200. The live
  HTML SHA-256 is
  `c9b131d3b3a4520d603085e81b264287c5754acfb0ac535840693047f7de1ab9`,
  exactly matching `dist/site/index.html`.
- The live hashed asset `/assets/index-DNl_AHsP.js` SHA-256 is
  `167d03bbf27426dd81a6dc7ca815caeb86e359356b71d8f441e393ccf41ec63f`,
  exactly matching the build. Its response is
  `Cache-Control: public, max-age=31536000, immutable`; HTML is
  `public, max-age=0, must-revalidate`; `/sw.js` is `no-cache`.
- Live responses include the configured CSP, `X-Frame-Options: DENY`,
  `Permissions-Policy`, COOP, CORP, `Referrer-Policy`, and nosniff.
- `LENS_TEST_URL=https://code-path-lens.sociobot.in npm run test:a11y`: 0 axe
  findings at both target sizes.
- Lighthouse 12.8.2 against the live site (mobile default) reported
  Performance **100**, Accessibility **100**, FCP **0.9 s**, LCP **1.2 s**,
  and CLS **0**.

## Build, run, deploy, publish

```sh
npm ci
npm test
npm run build
cargo package --allow-dirty
```

The deployable static output is `dist/site/`; the release CLI is
`dist/bin/code-path-lens`. The factory owns registry credentials: do not
publish from this worker. The ready-to-publish check is
`cargo package --allow-dirty`.

## Known gaps

None. The production site is deployed and the repair's live identity, paid
flow routing, cache/security policy, browser, accessibility, privacy, offline,
package, and performance checks passed.
