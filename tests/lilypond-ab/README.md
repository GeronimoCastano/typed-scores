# LilyPond A/B fixtures

These fixtures render the same supported notation in LilyPond 2.26.0 and
`typed-scores`. The committed SVGs are reference engravings embedded beside
live `typed-scores` output at the end of `tests/test.typ`.

Every comparison stays within the public `typed-scores` API. The sources are
adapted only to make the meter and page crop explicit; unsupported LilyPond
features are deliberately excluded.

| Fixture | LilyPond source |
| --- | --- |
| `grace-notes` | [Grace notes](https://lilypond.org/doc/v2.26/Documentation/notation/special-rhythmic-concerns), including the documented single acciaccatura, appoggiatura, multi-note acciaccatura, and an ordinary slur for bow-weight comparison |
| `grace-ledgers` | The focused grace/ledger-line case from the visual regression suite, expressed with LilyPond's documented grace commands |
| `nested-tuplets` | [Nested tuplets](https://lilypond.org/doc/v2.26/Documentation/notation/writing-rhythms) |
| `arpeggio-directions` | [Arpeggio directions](https://lilypond.org/doc/v2.26/Documentation/notation/expressive-marks-as-lines) |
| `single-tremolos` | [Single-note tremolo values](https://lilypond.org/doc/v2.26/Documentation/notation/short-repeats) |

Regenerate and verify the references with an installed LilyPond 2.26.0:

```sh
scripts/test-lilypond-ab.sh --update
scripts/test-lilypond-ab.sh
```

The normal visual-regression gate does not need LilyPond installed. It uses the
committed reference SVGs and compares the complete `tests/test.pdf` raster.
