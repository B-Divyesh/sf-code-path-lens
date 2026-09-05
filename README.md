# Code Path Lens

Code Path Lens traces a bounded path around one code symbol. It is for
developers reviewing unfamiliar Rust, TypeScript/JavaScript, Python, or Go
code who need callers, callees, types, data boundaries, source excerpts, and
unresolved calls in one small review slice.

The CLI uses local tree-sitter parsing. It does not upload source code or claim
to reproduce runtime behavior. Dynamic dispatch, reflection, generated code,
and runtime configuration can change the actual path.

## Try the bundled sample

Run this after installing the CLI. It creates a temporary Rust order sample,
opens the same evidence model used by the site demo, and prints the HTML output
path.

```sh
code-path-lens demo
```

The browser equivalent is <https://code-path-lens.sociobot.in/demo/>. It uses
only the `demo:code-path-lens:` local-storage namespace. **Reset demo** clears
that sample state. **Start for real** clears it and returns home.

## Install

Build from a source checkout with Rust 1.85 or newer:

```sh
cargo install --path .
```

## Use on a local repository

Create a self-contained HTML review slice. The free limits are depth 2 and 40
nodes.

```sh
code-path-lens trace handle_order --root ./shop --output order-lens.html
```

Export stable JSON for scripts or Graphviz DOT:

```sh
code-path-lens trace handle_order --root ./shop --json > order-lens.json
code-path-lens trace handle_order --root ./shop --format dot --output order-lens.dot
```

List supported adapters and their fallback behavior:

```sh
code-path-lens languages
```

Useful options:

```text
--depth <0..8>          Call steps in each direction (default 2)
--max-nodes <1..250>   Hard graph budget (default 40)
--exclude <GLOB>       Repeatable extra exclusion
--include-generated    Include files marked as generated
--link-template <TEXT> Source URL with {path}, {abs}, and {line} tokens
```

The CLI exits with `0` on success, `1` for analysis or I/O errors, `2` for
invalid arguments, `3` when a symbol is missing or ambiguous, and `4` when a
future paid bound needs a valid license. It never prompts, so it is safe in
CI.

## Viewer keyboard map

- `Tab`: reach filters, nodes, source links, and theme controls.
- Arrow keys: move between visible graph nodes.
- `Enter`: select a node and reveal its evidence excerpt.
- `/`: focus the node filter; `Escape`: clear it.

## Privacy

The free CLI has no account, cloud index, telemetry, or source upload. It
respects `.gitignore` and skips common generated and vendor directories by
default. The public site has no analytics, advertising, third-party fonts, or
third-party scripts. Read the full [privacy policy](https://code-path-lens.sociobot.in/privacy/)
and [terms](https://code-path-lens.sociobot.in/terms/).

## Develop and verify

From a clean checkout:

```sh
npm ci
npm test
npm run build            # release CLI + site -> dist/
npm run build:site       # landing/docs site -> dist/site/
cargo package --allow-dirty
```

`npm test -- --grep @claim:<id>` runs one documented public-claim check. The
complete list and commands are in `.factory/claims.json`. With `npm run dev`
running, `npm run test:a11y` checks desktop and 390 px viewports with
Playwright and axe. Set `LENS_TEST_URL` to check another server.

## Deploy and publish

The factory deploys `dist/site/` to
<https://code-path-lens.sociobot.in>. Do not deploy infrastructure from this
repository. The ready-to-publish package check is:

```sh
cargo package --allow-dirty
```

The factory owns registry and future billing configuration. This source tree
does not include payment credentials or a checkout substitute.

## License

MIT © 2026 Sociobot (Param Factory). See [LICENSE](LICENSE).
