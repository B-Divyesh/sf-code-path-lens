# Independent verification 2 — FAIL

**Work order:** `code-path-lens-verify-2`  
**Candidate:** `85f6581b5525ea8a8f53796a2cac2e58fcd661be`  
**Public URL:** `https://code-path-lens.sociobot.in/`  
**Verified:** 2026-08-28 UTC

## Verdict

**FAIL — do not release the paid production product yet.** The candidate is
buildable, its CLI and viewer work end to end, and the current public site is
now a byte-for-byte deployment of the candidate. The prior TLS/deployment
blocker is therefore resolved. But the public production site uses the pilot
billing API for checkout and license verification, contrary to the required
production Sociobot billing integration.

## Defects

### HIGH — deployed production paid flow uses the pilot billing service

The exact deployed candidate contains both of these production-page URLs:

- Buy link: `https://pilot-api.sociobot.in/api/v1/products/code-path-lens/checkout`
- License verification: `https://pilot-api.sociobot.in/api/v1/products/code-path-lens/verify?license=…`

Fresh browser evidence used a harmless `qa-invalid-token` in an isolated
browser context. The page stripped `?license=` from its URL, saved the token
and invalid verdict locally, showed the expected recovery message, and made
the actual request to `pilot-api.sociobot.in`; no console error occurred. The
source has the same hard-coded pilot base in `site/app.js` and the CLI default
in `src/license.rs`. The CLI does offer a `CODE_PATH_LENS_BILLING_BASE`
override, but the deployable site has no equivalent release configuration.

The product contract requires `https://api.sociobot.in/api/v1` for a released
paid product; pilot is only valid while staging. Buyers at the public URL
would be sent to the non-production flow. Configure/build the site with the
production base, deploy it, and repeat a token-only browser verification.

### MEDIUM — hashed static assets are not immutable-cached

The live HTML, JavaScript, service worker, and 73.6 KB WebP all return:

```text
Cache-Control: public, must-revalidate, max-age=30
```

including the content-addressed `assets/index-BK_Ypz2y.js`. This misses the
required long-lived immutable caching policy for hashed static assets. HTML
and `sw.js` may remain short-lived; versioned JS/image assets should instead
receive a long immutable lifetime.

### LOW — incomplete browser security policy hardening

The live response supplies HSTS, `Referrer-Policy: strict-origin-when-cross-origin`,
and `X-Content-Type-Options: nosniff`, but no Content-Security-Policy,
Permissions-Policy, X-Frame-Options, COOP, or CORP header. This was not an
observed exploit and does not alter the release verdict, but a static paid
site should define a tight CSP and explicit framing/permissions policy at its
deployment boundary.

## Fresh evidence

### Clean-checkout quality gates

A detached clean worktree at the candidate SHA was used. `npm ci` completed
with 0 audit vulnerabilities. The following passed:

```text
cargo test
  6 unit tests + 1 documented CLI integration test passed
cargo clippy --all-targets --all-features -- -D warnings
npm test
npm run build
cargo package --allow-dirty
  package verification passed; 20 files, 123.5 KiB unpacked / 35.1 KiB compressed
```

The exact build writes `dist/bin/code-path-lens` (9.2 MB) and `dist/site/`.
The first-load site bundle is 6,352 bytes JavaScript (2,680 gzip), the HTML
contains its CSS, no webfonts ship, and the hero WebP is 75,322 bytes. These
are within the JS/font/hero budgets.

Lighthouse was attempted with the installed matching Chromium. Its normal
launcher crashed the tab; a second single-process run produced accessibility
100 and FCP 0.9 s/LCP 1.5 s/TBT 0 ms/CLS 0, but Lighthouse reported no
screenshots and a non-valid performance score of 0. No Lighthouse performance
score is claimed from this container.

### CLI and package consumer

The verified package source from `cargo package` was installed into an empty
consumer prefix with:

```sh
cargo install --path target/package/code-path-lens-0.1.0 --root /tmp/cpl-consumer-prefix
```

Only the installed binary was exercised. `--version`, `--help`, and
`languages` gave the documented public surface. A separate Rust repository
with a caller, `handle_order`, `validate`, `Order`, `db_insert`, an unknown
call, a `.gitignore`, and a generated file emitted all required node kinds
(`entry`, `function`, `type`, `data_boundary`, `unresolved`) and edge kinds.
It retained the unresolved call and static-approximation warning; the
generated file was skipped by default and found with `--include-generated`;
the ignored file was not found when the fixture was a Git repository.

Boundary and recovery checks succeeded: `--depth 0 --max-nodes 1` emitted one
node, zero edges, and a visible truncation warning; missing symbol exited 3;
invalid `--depth 9` exited 2; a paid bound without a token exited 4. The
installed binary emitted a self-contained HTML lens with 7 nodes/12 edges.

### Browser, accessibility, PWA, and privacy

The exact local `dist/site/` and the live URL were tested in Chromium at
1366×900 and 390×844.

- `npm run test:a11y` reported 0 total (therefore 0 serious/critical) axe
  findings for both local and live viewports.
- The landing demo selected evidence, reached its filter empty state, recovered
  with Escape, accepted `/` to focus filtering, and moved nodes with arrow
  keys. Both widths had no horizontal overflow, one h1, visible 3px focus
  outlines, reduced-motion transitions of 0.01 ms, and no console/page errors.
- A CLI-generated HTML viewer separately passed axe with 0 findings at both
  widths and had the same keyboard, filter-recovery, focus, reduced-motion,
  overflow, and error results.
- The local service worker controlled a reload, accepted a same-scope updated
  script URL without waiting, and an offline reload retained the application
  title with no page errors. The page shell worked offline; the container's
  browser offline emulation did not flip `navigator.onLine`, so it could not
  independently assert the offline-banner visual state.
- A normal initial page load made no cross-origin network requests. Static
  inspection and the exercised invalid-license path show that the only
  application network request is the license token request; no repository
  contents, filenames, graph data, analytics, tracking, CDN font, or remote
  index request is sent.

### Live deployment and response evidence

The previously broken hostname is now valid: DNS resolved to
`40.67.153.174`, the SNI certificate subject is
`CN = code-path-lens.sociobot.in`, OpenSSL hostname verification returned 0,
and normal HTTPS returned HTTP/2 200. The live `/` HTML SHA-256 was
`1bcf821070d0e1f08e8445d08ac5005a49cc501afdea929203fbc8b92e8b5112`,
identical to the candidate build. The live hashed JS asset name and SHA-256
also matched exactly:

```text
/assets/index-BK_Ypz2y.js
141c04790b6dc293bf312385d8fd89cacdd2720e7e9ee510f7b4a79bf3580988
```

The live browser exercises produced no console or page errors at either
viewport, establishing that the deployment is the tested candidate. Headers
were checked for `/`, the hashed JS, `/sw.js`, `/privacy/`, and the hero image;
all had correct MIME types and the policy/cache results recorded above.

## Required release actions

1. Build/deploy the public site with `https://api.sociobot.in/api/v1` for both
   checkout and verification; keep pilot only for a staging deployment.
2. Add immutable cache headers for hashed static assets while preserving a
   short update interval for HTML and the service worker.
3. Add an explicit CSP and framing/permissions policy at deployment, then
   rerun the public live, license, header/cache, and Lighthouse checks.
