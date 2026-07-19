# Packaging And Release Checklist

Use `main` as the source of truth. The package submission branch and the
`typst/packages` fork are release artifacts, not development branches.

## Repositories

Source repository:

```text
/Users/gerocastano8/Documents/Coding/Projects/typed-scores
https://github.com/GeronimoCastano/typed-scores
```

The pull request target is always `typst/packages:main`. Typst package versions
are immutable in practice: publish a new version instead of modifying an
accepted version directory.

## Local Release Steps

1. Update `typst.toml` to the new SemVer version.
2. Update README imports and examples to use the new version.
3. Run the complete release verification:

```sh
scripts/verify-release.sh
```

This builds the WASM plugin, runs Rust and expected-error tests, regenerates the
visual suite, guide, and five-piece showcase, creates an isolated package
bundle, and compiles a smoke document against that bundle.

The committed `tests/test.pdf` is also the approved rendering baseline. CI
compiles the same fixture suite and compares Poppler rasterizations with a
small anti-aliasing tolerance. After an intentional visual change, inspect the
new PDF before committing it; an unreviewed engraving change will fail CI.

4. Regenerate README images whose source or rendering changed. The release-gate
fixture uses:

```sh
typst compile --root . --ppi 220 assets/readme/chopin-opening.typ assets/readme/chopin-opening.png
```

5. Inspect the regenerated PDFs and PNGs, then commit and push the release.

## Typst Packages PR Steps

1. Sync the fork of `typst/packages` with upstream `main`.
2. Create a release branch from upstream `main`.
3. From this repository, create the guarded package bundle:

```sh
scripts/package-preview.sh 0.1.0 /path/to/typst/packages
```

The script verifies the manifest version and required runtime files, refuses
to overwrite an existing version, and copies only the package whitelist.

4. Validate from the packages checkout:

```sh
cd /path/to/typst/packages/packages
typst-package-check check @preview/typed-scores:0.1.0
```

5. Commit the new directory, push the release branch, and open a PR against
`typst/packages:main`.

The submission directory is:

```text
packages/preview/typed-scores/0.1.0/
```

It contains `typst.toml`, `LICENSE`, `README.md`, the complete runtime `src/`
tree (including `plugin.wasm`, Bravura SVGs, and the OFL license), and README
PNG assets. It excludes Rust sources, tests, documentation sources, build
scripts, and repository guidance.

For the initial submission, use the PR title:

```text
[preview/typed-scores:0.1.0] Add typed-scores package
```
