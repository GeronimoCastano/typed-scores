# Packaging And Release Checklist

Use `main` as the source of truth. Package submissions to `typst/packages` are
release artifacts, not development branches.

## Local Release Steps

1. Update `typst.toml` to the new SemVer version.
2. Update README imports and examples to use the new version.
3. Build the WASM plugin:

```sh
./build.sh
```

4. Run Rust tests:

```sh
cargo test --manifest-path plugin/Cargo.toml
```

5. Compile the visual smoke test:

```sh
typst compile --root . tests/test.typ tests/test.pdf
```

6. Commit and push the release.

## Typst Packages PR Steps

1. Sync the fork of `typst/packages` with upstream `main`.
2. Create the submission directory:

```text
packages/preview/typed-scores/0.1.0/
```

3. Copy only package bundle files: `typst.toml`, `LICENSE`, `README.md`, and
   `src/` including `src/plugin.wasm` and SVG assets.
4. Do not copy `docs/`, `plugin/`, `build.sh`, `assets/`, `tests/`, or
   `PACKAGING.md`.
5. Open a PR against `typst/packages` main titled:

```text
[preview/typed-scores:0.1.0] Add typed-scores package
```
