# Verify bounded code-path review — FAIL

**Work order:** `code-path-lens-verify-4`

**Verified:** 2026-09-05 UTC

**Live URL:** <https://code-path-lens.sociobot.in>

**Implementation candidate:** `378180b12c9745185b1adad6f081d05a4cc31bff`

**Documentation baseline:** `3d5a00ba0190bd9efb9d8bcdc19e1265cfebb5c3`

## Verdict

**FAIL — 1 finding and 1 untested public claim.**

The installed CLI performs the real bounded-analysis job, all 16 declared
claim commands pass, and the live site is the candidate build. The release
cannot receive a PASS under the claims contract because the primary demo shows
a false, untested quantitative result: it says the sample has six edge types,
while the real bundled sample has five.

| Severity | Count |
| --- | ---: |
| Blocker | 0 |
| High | 0 |
| Medium | 1 |
| Low | 0 |
| **All findings** | **1** |
| **Untested public claims** | **1** |

## Job, audience, and first action before scrolling

Fresh Chromium contexts at 1366×900 and 390×844 showed the same clear first
screen before any scroll:

- **Job:** Trace a bounded path around a code symbol.
- **Audience:** Developers reviewing unfamiliar code.
- **First action:** **Try it with sample data.** The adjacent text says it
  opens a populated Rust order path and saves nothing.

The action was visible in both viewports. The screen also showed three facts:
local source analysis, an offline sample page, and free core outputs.
Screenshots are in `/work/.evidence/cpl-verify-4/desktop-first-screen.png` and
`phone-first-screen.png`.

## Finding

### CPL-V4-01 — Medium — The demo reports the wrong number of edge types

The live `/demo/` header says **“7 nodes / 6 edge types”** at both desktop and
phone widths. The same page's edge list names only five relationship types:
caller/call, callee/call, uses type, crosses boundary, and unresolved call.

The real installed artifact proves the mismatch. From the packaged consumer,
`code-path-lens demo --json` returned 7 nodes, 12 edges, and these five unique
edge kinds:

```text
uses_type
calls
called_by
crosses_boundary
unresolved_call
```

The value `6 edge types` is a quantitative public claim. It has no matching
claim entry or numerical assertion in `.factory/claims.json`; the
`bounded-evidence` check asserts evidence categories but not this displayed
count. This is both false and untested. It matters because the product is sold
as a deterministic evidence view and the incorrect number appears in its main
sample output.

**Required result:** Show the real count, or replace the count with an accurate
non-quantitative label. If a number remains public, add a tagged claim test
that derives and asserts it from the same sample data.

## Declared claim commands

A new clone was detached at documentation baseline `3d5a00b`. `npm ci`
installed the documented prerequisites and reported 0 vulnerabilities. Every
command in `.factory/claims.json` was then run exactly as declared.

| Claim | Result | Evidence checked by its command |
| --- | --- | --- |
| `deterministic-slice` | PASS | Two sample JSON runs were byte-identical. |
| `bounded-evidence` | PASS | Caller, callee, type, boundary, and unresolved nodes were present. |
| `source-evidence` | PASS | Path, line 7, excerpt, and deterministic link were present. |
| `language-adapters` | PASS | Rust, TypeScript, JavaScript, Python, and Go fixtures traced. |
| `exclusions` | PASS | `.gitignore`, vendor, generated default, and generated opt-in paths behaved as stated. |
| `formats` | PASS | HTML opened directly; JSON parsed in the command's Rust integration run; DOT structure was asserted. |
| `free-bounds` | PASS | Depth 0 and one-node output reported truncation. |
| `static-approximation` | PASS | The sample JSON included the runtime-incompleteness notice and warning. |
| `local-cli` | PASS | The sample trace completed with unreachable HTTP proxy settings. |
| `offline-cli` | PASS | The CLI demo wrote HTML with unreachable HTTP proxy settings. |
| `no-third-party-site` | PASS | The isolated demo load requested only its own origin. |
| `offline-site` | PASS | A dedicated context reloaded `/demo/` offline after service-worker control. |
| `exits-no-prompts` | PASS | Success, missing symbol, invalid depth, and paid-bound exits matched. |
| `cli-demo` | PASS | An empty-directory demo wrote populated HTML and reported both paths. |
| `site-demo-isolation` | PASS | Reset restored the sample and preserved a seeded real-data key. |
| `keyboard-viewer` | PASS | Slash, Escape, and arrow-key behavior passed. |

Declared claims: **16 passed, 0 failed, 0 unrun**. The separate untested count
is the undeclared public `6 edge types` statement in CPL-V4-01.

## Clean build and package consumer

The following passed in the detached checkout:

```text
npm test
npm run build
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty
npm audit --audit-level=high
```

- `npm test`: 7 Rust unit tests, 2 CLI integration tests, static checks, and
  all 16 claim checks passed.
- `npm run build`: produced `dist/bin/code-path-lens` and `dist/site/`.
- Cargo package verification: 26 files, 143.1 KiB unpacked and 40.2 KiB
  compressed.
- The packaged source was installed into a new consumer prefix with `--locked`.
  Only that installed binary was exercised.
- `--version`, `--help`, and `languages` worked. The language list included
  the stated fallbacks.
- `demo --output` wrote a self-contained populated HTML page. `demo --json`
  returned 7 nodes and 12 edges across all required node kinds.
- Boundary `--depth 0 --max-nodes 1` returned one node, no edges, and explicit
  truncation/static warnings.
- Missing symbol exited 3, invalid depth exited 2, a future paid bound exited
  4, and a missing root exited 1. Each returned useful text without prompting.
- The generated viewer worked at desktop and phone widths. Filter empty and
  recovery states, arrow and Enter selection, both paper themes, source
  evidence, and reduced motion worked. Axe found 0 violations in both themes.

## Live demo, accessibility, privacy, and recovery

The first-screen action was clicked in both fresh browser contexts rather than
opening the demo only by URL.

- The demo opened with its persistent **“Demo — sample data, nothing is
  saved”** label and realistic `handle_order` source evidence.
- Seven nodes covered the entry, caller, callees, `Order`, database boundary,
  and unresolved `emit_receipt` call. Every source link resolved to a checked-in
  file and line.
- Selecting `validate` changed the evidence. **Reset demo** restored
  `handle_order`, cleared the filter, and focused it.
- A seeded `real:data` value remained unchanged throughout. Demo mode used only
  `demo:code-path-lens:session`. **Start for real** removed that marker, kept
  the real-data value, and returned home.
- Slash focused the filter, an unmatched filter displayed the empty state,
  Escape restored the nodes, arrows moved focus, and Enter and Space selected
  nodes. Focus used a visible 3 px ochre outline.
- Every visible link, input, and button measured at least 44 px in both tested
  contexts. The phone pages had no document overflow. The viewport metadata
  does not disable zoom.
- With reduced motion enabled, scrolling was `auto`, transitions were 0.01 ms,
  and no infinite animation remained.
- A service-worker-controlled `/demo/` reloaded offline with the demo title,
  sample, and visible offline status. The worker is served with `no-cache` and
  uses immediate activation; no separate update promise is made.
- Normal landing and demo use made only same-origin requests. No console error
  or page error occurred, and the seeded real-data key was untouched.
- `/opt/fleet/lib/verify-url.sh` passed. Axe reported 0 violations on `/`,
  `/demo/`, `/privacy/`, `/terms/`, and `/404.html` at 1366 px and 390 px.
- Each real route has its own title, one `h1`, `lang="en"`, header/nav/main/footer
  landmarks, canonical and share metadata, and no phone-width page overflow.
  First Tab reached the visible skip link and showed the designed focus ring.
- Internal links and the public repository/sample source links returned 200.
  An unknown URL deliberately returned HTTP 404 with the styled “Page not
  found” page and working home/sample actions. Chromium's failed-resource log
  for that deliberate navigation is expected, not a defect.
- The privacy-request route is the public repository. This product has no
  product account database.

This is a CLI plus static site. Backend tenant isolation, SQLite restart
persistence, service health, and 429/`Retry-After` checks are not applicable.

## Performance and deployment identity

Live Lighthouse 13.4.1 scored **100 Performance, 100 Accessibility, 100 Best
Practices, and 100 SEO**. FCP was 1.028 s, LCP 1.230 s, TBT 0 ms, and CLS 0.
The report is `/work/.evidence/cpl-verify-4/lighthouse.json`.

The initial JavaScript is 6,068 bytes (2,032 gzip), the hero WebP is 75,322
bytes, and no webfonts ship. Hashed JavaScript is immutable-cached; HTML is
revalidated; `sw.js` is `no-cache`. Live responses include the expected CSP,
HSTS, nosniff, referrer, framing, opener/resource, and permissions policies.

The valid TLS certificate names `code-path-lens.sociobot.in`. Fresh candidate
output and live bytes match exactly:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `be03305eefe18566cff3d536f6294267d0b8debaf854628f8798f68c4e9ea413` |
| `assets/app-bH3Yy194.js` | `54ef2a834122058879d123deafd3e922389187afbe6038395c957603c1723cdc` |
| `sw.js` | `e7ba57672075099438c5646233d29bc6d77e4e40f0bc01ff5f5db18f79c7ef87` |

Only report files and the changelog differ between `378180b` and the reviewed
documentation baseline, so this proves the live runtime is the implementation
candidate rather than a later report-only build.

## Earlier finding disposition

| Earlier finding | Current disposition |
| --- | --- |
| Invalid TLS and missing live product | Resolved. HTTPS is valid and serves the matching candidate. |
| Pilot billing base | Resolved in the CLI's future verification default; the unavailable paid UI was removed. |
| Weak cache and security headers | Resolved and verified live. |
| Broken paid checkout | Removed from public claims and actions. A fresh unauthenticated endpoint check now returned a 303 hosted-checkout redirect, so external registration appears to have changed; no purchase or license flow was attempted. |
| Missing CLI and browser sample | Resolved by `demo`, `/demo/`, shipped source, reset/exit controls, and source links. |
| Missing claims manifest/tests | Structurally resolved for 16 declared claims, but the new false count in CPL-V4-01 remains outside the manifest. |
| Metaphorical first-screen copy | Resolved with the direct job, audience, action, facts, and copy audit. |
| Missing deliberate 404 | Resolved. Unknown paths return the designed page with HTTP 404. |
| Missing metadata/shared structure | Resolved on all real routes. |
| Small touch targets | Resolved; tested visible targets are at least 44 px. |

## Evidence

Browser screenshots, URL verification output, and Lighthouse JSON are under
`/work/.evidence/cpl-verify-4/`. The required report copy and machine verdict
are `/work/.evidence/qa-report.md` and `/work/.evidence/qa-result.json`.
