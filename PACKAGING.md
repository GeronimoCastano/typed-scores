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

Use the PR title format `typed-scores:<VERSION>`. For the initial submission:

```text
typed-scores:0.1.0
```

For the initial submission, keep the full new-package checklist from the
`typst/packages` pull request template. Check the new-package box, leave the
update box unchecked, replace the description placeholder, explain the package
name, check every applicable checklist item, and remove the template-only item
because `typed-scores` is not a template package. Use this body:

```markdown
I am submitting
- [x] a new package
- [ ] an update for a package

Description: `typed-scores` engraves western music notation from compact note
text. It gives Typst users a package-native way to create scores with exact
rhythmic layout, multi-staff notation, and bundled Bravura music glyphs.

I have read and followed the submission guidelines and, in particular, I
- [x] selected [a name](https://github.com/typst/packages/blob/main/docs/manifest.md#naming-rules) that isn't the most obvious or canonical name for what the package does
  - Explanation: `typed-scores` uses the non-descriptive `typed-` prefix to identify it as part of the Typed package family; `scores` communicates its music-notation domain.
- [x] added a [`typst.toml`](https://github.com/typst/packages/blob/main/docs/manifest.md#package-metadata) file with all required keys
- [x] added a [`README.md`](https://github.com/typst/packages/blob/main/docs/documentation.md) with documentation for my package
- [x] have chosen [a license](https://github.com/typst/packages/blob/main/docs/licensing.md) and added a `LICENSE` file or linked one in my `README.md`
- [x] tested my package locally on my system and it worked
- [x] [`exclude`d](https://github.com/typst/packages/blob/main/docs/tips.md#what-to-commit-what-to-exclude) PDFs or README images, if any, but not the LICENSE
```

For later releases, use only the compact update body. Do not include the full
new-package checklist, validation sections, or generated-artifact notes:

```markdown
I am submitting
- [ ] a new package
- [x] an update for a package

Changes:

- <meaningful user-visible change>
- <meaningful user-visible change>
```
