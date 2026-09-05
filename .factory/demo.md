# Demo sandbox

## Browser demo

Open <https://code-path-lens.sociobot.in/demo/> or `/demo/` on a local build.
The first screen contains a populated Rust order path around `handle_order`.
The persistent banner reads **“Demo — sample data, nothing is saved.”**

The browser demo writes only `demo:code-path-lens:session` to local storage.
It does not read or write real product keys. **Reset demo** returns the filter
and selected evidence to the shipped sample, then replaces that demo marker.
**Start for real** removes the marker and returns to the landing page.

## CLI demo

```sh
code-path-lens demo
```

The binary materializes `examples/checkout-sample/` in a new temporary
directory, runs the same real analyzer around `handle_order`, and prints both
the temporary repository location and generated output path. By default it
writes `handle_order-lens.html` beside that temporary sample. Use `--output`
to choose another location or `--json` for standard output.

## Shipped sample data

`examples/checkout-sample/` contains a small Rust order handler with a caller,
a callee, the `Order` type, an `insert` data boundary, and an unresolved
`emit_receipt` call. The source location links on `/demo/` point to these
checked-in files and lines.
