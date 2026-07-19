# LilyPond showcase references

These LilyPond 2.26.0 sources express the same excerpts and supported notation
as the three pages in `../showcase.typ`. They are translations of the
typed-scores fixtures, rather than copies of the larger Mutopia transcriptions,
so differences in the generated SVGs are useful engraving comparisons.

The explicit system breaks mirror the current showcase packing:

| Fixture | Systems |
| --- | --- |
| `chopin-nocturne-op9-no2` | pickup + 1–2, 3–4, 5–6, 7–8 |
| `mozart-eine-kleine-nachtmusik` | 1–4, 5–6, 7–8, 9–10 |
| `bach-cello-suite-prelude` | two measures per system |

Regenerate the committed SVGs with:

```sh
scripts/render-showcase-lilypond.sh --update
```

Verify that the committed SVGs still match LilyPond 2.26.0 with:

```sh
scripts/render-showcase-lilypond.sh
```

LilyPond is needed only to regenerate or verify these references. The SVGs in
`reference/` can be viewed directly without it.
