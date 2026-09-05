# Review bounded code paths around one symbol — FAIL

**Work order:** `code-path-lens-review-1`

**Reviewed:** 2026-09-05 UTC

**Live URL:** <https://code-path-lens.sociobot.in>

**Implementation candidate:** `bfb3e1d258fadf56478cd980b575a82708a81a63`

**Documentation baseline:** `f801a5aa4703d3f20f0eae4e19bc0ac7bfe1cc71`

## Verdict

**FAIL — 7 findings and 28 untested public claim families.**

The free installed CLI performs the main analysis job and produces a usable
viewer. The live site is fast, accessible in automated checks, and matches the
last implementation build. The product does not meet the current release
contract because its paid checkout is broken, its required CLI and site sample
mode is missing, and none of its public claims has the required claim entry and
sandbox test.

| Severity | Count |
| --- | ---: |
| Blocker | 0 |
| High | 3 |
| Medium | 3 |
| Low | 1 |
| **All findings** | **7** |
| **Untested claim families** | **28** |

## Job, audience, and first action before scrolling

- **Job:** Build a small, bounded, source-linked view around one code symbol.
- **Audience:** Developers reviewing an unfamiliar Rust, TypeScript/JavaScript,
  Python, or Go code path.
- **First action shown:** “Install the CLI” on desktop and phone. “Inspect the
  live slice” is the second action. The required “Try it with sample data”
  action is not present.

The first screen was opened in fresh Chromium contexts at 1366×900 and
390×844 before scrolling. Screenshots are in
`/work/.evidence/live-desktop-first-screen.png` and
`/work/.evidence/live-phone-first-screen.png`.

## Findings

### CPL-R1-01 — High — The advertised Pro checkout is unavailable

The public “Buy Code Path Lens Pro” link points to the correct production API
host, but a fresh request returned HTTP 404 with a JSON error saying the
factory product was not enabled. This is an unexpected 404 in a purchase path,
not the expected response from a deliberately missing site page. A visitor
cannot buy the advertised $29 license.

The invalid-license verification endpoint does work: it returned HTTP 200 with
an invalid verdict, and the live page removed the query token, stored the local
verdict, used the production API, and showed recovery text. A successful paid
unlock could not be tested because checkout cannot start and no valid test
license is present.

**Required result:** Enable the product in the Sociobot billing service, then
test checkout, return, restore, valid unlock, invalid/revoked recovery, and the
paid CLI limits without putting a credential in test output.

### CPL-R1-02 — High — The required sample mode is missing

The page has a useful pre-populated six-node example. Selecting nodes,
filtering to an empty result, pressing Escape, and moving between nodes work on
desktop and phone. The sample interaction made no local-storage change and no
cross-origin request.

It does not meet the sample contract:

- The first screen has no “Try it with sample data” action.
- `/demo` serves the ordinary landing page with the landing title.
- There is no persistent “Demo — sample data, nothing is saved” label.
- There are no “Reset demo” and “Start for real” controls.
- The sample's “Open source location” link goes to the repository root rather
  than the displayed sample file and line.
- `.factory/demo.md` is absent.
- `examples/` is absent.
- The landing page has no terminal recording of the installed binary.
- The installed CLI has no `demo` or `--demo` entry point. `code-path-lens
  demo` exits 2 as an unknown command.

**Required result:** Ship one bundled sample repository, run it through the
real binary from a one-command demo, document it, and expose a real `/demo`
state with the persistent sample label and reset/exit controls.

### CPL-R1-03 — High — Public claims have no claim manifest or tagged tests

`.factory/claims.json` is absent. There are no `@claim:<id>` tests and therefore
no declared claim commands to run. The normal test suite covers parts of the
behavior, but it cannot substitute for the required clean-sandbox claim map.
Twenty-eight distinct claim families were deduplicated across the live site,
README, privacy page, terms, and CLI help:

| ID | Public claim family | Required claim test |
| --- | --- | --- |
| C01 | Output is deterministic | Repeat the same sample and compare output |
| C02 | Finds callers | Assert a known caller in the bundled sample |
| C03 | Finds callees | Assert a known callee in the bundled sample |
| C04 | Finds referenced types | Assert the sample type node and edge |
| C05 | Finds data boundaries | Assert the sample boundary node and edge |
| C06 | Preserves source excerpts and links | Assert file, line, excerpt, and link |
| C07 | Keeps unresolved calls visible | Assert the known unresolved sample call |
| C08 | Supports Rust | Run the Rust sample fixture |
| C09 | Supports TypeScript/JavaScript | Run the TypeScript and JavaScript fixtures |
| C10 | Supports Python | Run the Python fixture |
| C11 | Supports Go | Run the Go fixture |
| C12 | Respects `.gitignore` | Put a matching symbol only in an ignored file |
| C13 | Excludes generated/vendor code by default | Compare default and include-generated runs |
| C14 | Emits self-contained HTML | Open generated HTML without repository assets |
| C15 | Emits stable JSON | Assert schema and sample graph |
| C16 | Emits Graphviz DOT | Parse or assert the sample DOT graph |
| C17 | Applies free bounds and reports truncation | Assert depth 2, 40 nodes, and truncation |
| C18 | Labels static approximation limits | Assert the notice in every output format |
| C19 | Source and repository data stay local | Record all sample-run network requests |
| C20 | Needs no daemon, account, cloud index, or LSP | Run in an isolated clean consumer |
| C21 | Has no telemetry, crash reporter, or remote index | Record process and browser network traffic |
| C22 | Loads no third-party fonts, scripts, ads, or analytics | Record a fresh page load |
| C23 | The free CLI works offline | Install first, block network, then run the sample |
| C24 | The site shell works offline | Load, go offline, and reload a fresh context |
| C25 | Uses documented exit codes and never prompts in CI | Assert all normal and error exits with closed stdin |
| C26 | Stores and checks only a license token at most daily | Assert requests and browser storage with fixtures |
| C27 | Pro is a $29 one-time license with depth 8 and 250 nodes | Test a recorded checkout and valid-license fixture |
| C28 | Formats, evidence, accessibility, export, and privacy stay free | Run each feature without a license |

All 28 are **untested under the claims contract** because none is listed and
none has exactly one tagged test. This count is separate from the single
umbrella finding count.

**Required result:** Add `.factory/claims.json`, one tagged clean-sandbox test
for each retained claim, and remove or narrow any claim that cannot be proved.

### CPL-R1-04 — Medium — The first screen and section headings fail the plain-words contract

The headline “See the path. Not the whole repository.” does not name the job.
The sentence below it does not name developers or the unfamiliar-code
situation. The primary action installs the CLI instead of starting the sample.
The three fact lines omit the required price fact.

The page also uses metaphor or decorative labels where the contract requires
plain headings, including “A field guide for unfamiliar code,” “Recorded
evidence / 01,” “Evidence before explanation,” “One binary / 03,” “Point it at
a symbol,” “Pro field kit / 04,” and “Wider bounds, once.” The HTML title repeats
the metaphor instead of naming the analysis job. `.factory/copy-audit.md` is
absent, so the required sentence and terminology audit was not performed.

**Required result:** Name the bounded symbol-analysis job, audience, and sample
action in the first screen. Replace decorative headings with section names and
complete the copy audit.

### CPL-R1-05 — Medium — Missing URLs do not get a deliberate 404 page

`/404.html` and a fresh nonexistent path both returned HTTP 200 and the full
landing page. There is no product-styled 404 document, no missing-page heading,
and no response override. This is a missing required route, not a defect merely
because a deliberate 404 status was observed.

**Required result:** Add a styled 404 page with a way home and configure the
host to return HTTP 404 for missing pages.

### CPL-R1-06 — Medium — Required metadata and shared site structure are incomplete

No checked route supplies a canonical link, Open Graph metadata, Twitter card,
1200×630 product image, or 180 px Apple touch icon. The sitemap omits the
required `/demo` route. Header and footer content differs between landing,
privacy, and terms pages. No footer has the required “Built by Param Factory”
text and build id; legal-page footers also omit the product version.

Route titles, `lang`, one `h1`, main landmarks, favicon, theme color,
`robots.txt`, legal pages, and the landing description are present.

**Required result:** Add the missing metadata/assets, list `/demo`, and use the
same required header/footer skeleton on every real route.

### CPL-R1-07 — Low — Several touch targets are smaller than 44 px

At phone width, the repository wordmark, sample source link, privacy/terms
links, and footer links measure about 19–26 px high. Desktop header text links
are also about 19 px high. Keyboard focus is visible, but these targets do not
meet the attached 44×44 px touch-target requirement.

**Required result:** Increase the clickable area without changing the visible
text size or link meaning.

## Clean checkout and declared commands

A fresh clone at documentation SHA `f801a5a` was used. `npm ci` installed the
documented prerequisites and reported 0 vulnerabilities.

| Command | Result |
| --- | --- |
| `npm test` | PASS: 7 Rust unit tests, 1 CLI integration test, site build and static checks |
| `npm run build` | PASS: `dist/bin/code-path-lens` and `dist/site/` produced |
| `npm run build:site` | PASS as part of both commands |
| `cargo package --allow-dirty` | PASS: 20 files, 124.5 KiB unpacked, 35.3 KiB compressed |
| `cargo clippy --all-targets --all-features -- -D warnings` | PASS |
| `npm audit --audit-level=high` | PASS: 0 vulnerabilities |
| `LENS_TEST_URL=https://code-path-lens.sociobot.in npm run test:a11y` | PASS: 0 axe findings at 1366 px and 390 px |
| Claim commands from `.factory/claims.json` | NONE: required file is missing |

The README's plain `cargo install --path .` command is valid for a source
checkout. Package consumption was tested more strictly from Cargo's packaged
source as described below.

## Installed CLI and viewer results

The packaged source was installed into a new consumer prefix with `--locked`.
Only that installed binary was then exercised.

- `--version`, `--help`, and `languages` passed. Help lists the supported
  languages and explicit unsupported-language fallback.
- A normal trace of the packaged source around `main` returned 35 nodes and 35
  edges with entry, function, type, data-boundary, and unresolved node kinds.
- `--depth 0 --max-nodes 1` returned one node, no edges, `limits.truncated:
  true`, and clear node-budget and static-approximation warnings.
- A missing symbol exited 3; depth 9 exited 2; an unpaid depth 3 exited 4; and a
  missing root exited 1. Each wrote useful error text and did not prompt.
- The generated 35-node HTML viewer worked at 1366×900 and 390×844. Node
  selection, filter empty state, Escape recovery, `/` focus, source evidence,
  arrow navigation, and both paper themes worked. It had no document overflow,
  console error, page error, or axe violation. Reduced motion computed to an
  effectively instant transition.
- The existing unit tests still prove ambiguous-symbol handling, the polyglot
  adapters, boundary and unresolved evidence, and the production billing base.
  Earlier fresh-fixture findings for `.gitignore` and generated-code handling
  have no intervening implementation change, but they still need claim-tagged
  tests under CPL-R1-03.

The installed viewer and screenshots are in `/work/.evidence/`.

## Live browser, accessibility, privacy, and recovery results

Fresh desktop and phone contexts were used; the sample was not accepted based
on source inspection alone.

- Landing, privacy, and terms pages had one `h1`, useful landmarks, correct
  route titles for the legal pages, no horizontal document overflow, and no
  console or page errors.
- Axe reported 0 violations on landing, privacy, terms, and the generated CLI
  viewer at both target widths. The viewer's second theme also had 0 findings.
- Tab reached the skip link first. Sampled focus targets showed the designed
  3 px ochre outline. Arrow keys, Enter/click selection, `/`, and Escape worked.
- Reduced-motion contexts reported `scroll-behavior: auto` and effectively
  instant transitions. Nothing flashed or looped.
- A fresh normal load and the complete sample interaction contacted only the
  product origin and did not change local storage. An isolated invalid-license
  recovery contacted only the production Sociobot API, stored only the test
  token/verdict, stripped the token from the URL, and showed clear recovery
  text. No credential was used or recorded.
- After one online load, the service worker controlled a reload. An offline
  reload retained the page, set `navigator.onLine` false, displayed the offline
  status, and produced no browser error. The worker calls `skipWaiting` and
  `clients.claim`; the deployed worker is served with `Cache-Control: no-cache`.
- The public privacy-request path points to the working GitHub repository or a
  purchase-receipt support contact. The product holds no account database.
- Internal links returned 200. The public GitHub repository returned 200. The
  only broken public action found was the paid checkout in CPL-R1-01.

This is a CLI with a static site, not a product backend. Tenant isolation,
SQLite restart persistence, backend health, and 429/`Retry-After` checks are
not applicable. The external billing gateway was checked only through the
product's documented public requests.

## Performance, security, and deployment identity

Lighthouse 13.4.1 against the live phone profile reported Performance 100,
Accessibility 100, Best Practices 100, and SEO 100. FCP was 1.0 s, LCP 1.3 s,
TBT 80 ms, and CLS 0. The result is stored at
`/work/.evidence/lighthouse-review-1.json`.

The built JavaScript is 6,346 bytes (2,680 bytes gzip), no font ships, and the
hero WebP is 75,322 bytes. The release stays below the stated asset budgets.
Hashed JS and the hero use one-year immutable caching; the service worker uses
`no-cache`. Live responses include HSTS, CSP, nosniff, strict referrer policy,
frame denial, COOP, CORP, and a restrictive permissions policy.

Current build and live hashes match:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `c9b131d3b3a4520d603085e81b264287c5754acfb0ac535840693047f7de1ab9` |
| `assets/index-DNl_AHsP.js` | `167d03bbf27426dd81a6dc7ca815caeb86e359356b71d8f441e393ccf41ec63f` |
| `sw.js` | `b0572e8bfcdf220e8ae9ad2bcd627dac3c16f56c7bea00da057229d507d6b7e4` |

`bfb3e1d` is the last implementation commit. Commits `28d7f03` and `f801a5a`
change only factory reports, so no newer product image is required for those
documentation changes.

## Earlier finding disposition

| Earlier finding | Current disposition and proof |
| --- | --- |
| Broken TLS and missing live deployment | **Resolved.** Normal HTTPS returns 200 with a valid live product response. |
| Production site and CLI used the pilot billing host | **Resolved.** Built and live assets use `https://api.sociobot.in/api/v1`; invalid verification reached that host. |
| Hashed assets lacked immutable caching | **Resolved.** Live JS and WebP return `public, max-age=31536000, immutable`. |
| Security headers were incomplete | **Resolved.** CSP, frame denial, permissions policy, HSTS, nosniff, COOP, CORP, and referrer policy are live. |
| Lighthouse performance was not established | **Resolved.** Current live run scored 100 performance with 1.3 s LCP. |
| Previous verification 3 reported no gaps | **Superseded.** It did not apply the current claims, sample, plain-words, site-structure, checkout-availability, and 404 checks that produced this review's findings. |

## Release decision

Do not declare the product accepted. Fix all seven findings, add and run every
claim command from a clean sample environment, enable and test checkout, then
repeat this independent review. A successful build or worker exit does not
change this **FAIL** verdict.
