# Handoff — Code Path Lens 0.1.0

## Independent verification status: FAIL

Fresh verification on 2026-08-28 tested candidate
`85f6581b5525ea8a8f53796a2cac2e58fcd661be` against
`https://code-path-lens.sociobot.in/`.

The earlier deployment-only failure is resolved: normal HTTPS validates and
the live HTML and hashed JavaScript are byte-for-byte identical to the exact
candidate production build. Local tests, Clippy, package verification, clean
consumer install, CLI flows, desktop/mobile browser checks, axe, focus,
reduced motion, generated viewer, and offline reload all passed.

**Do not release yet.** The public site sends checkout and license verification
to `https://pilot-api.sociobot.in/api/v1`, not the required production
`https://api.sociobot.in/api/v1`. This is a HIGH release blocker for a paid
production product. The live deployment also caches hashed assets for only 30
seconds (MEDIUM) and lacks CSP/framing/permissions policy hardening (LOW).

Complete exact evidence, commands, test outcomes, deployment hashes, and
required fixes are in `.factory/verification-2.md`. The earlier report remains
in `.factory/verification.md` for historical context.

## How to build and verify locally

```sh
npm ci
cargo test
cargo clippy --all-targets --all-features -- -D warnings
npm test
npm run build
cargo package --allow-dirty
```

`npm run build` writes the release binary to `dist/bin/code-path-lens` and the
deployable static site to `dist/site/`. The ready-to-publish Rust package was
verified with `cargo package --allow-dirty`; do not publish it from this worker.

## Required release steps

1. Build/deploy the public site with the production Sociobot API base and
   repeat the token-only live license check.
2. Apply immutable cache policy to hashed JS/image assets while keeping HTML
   and `sw.js` updateable.
3. Apply CSP/framing/permissions response policy and rerun live header/cache
   and Lighthouse verification.
