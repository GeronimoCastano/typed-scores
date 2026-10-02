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
| `cross-staff-beams` | [Changing staff manually](https://lilypond.org/doc/v2.26/Documentation/notation/common-notation-for-keyboards), with automatic beams across the staves |
| `ottava-brackets` | [Ottava brackets](https://lilypond.org/doc/v2.26/Documentation/notation/displaying-pitches), with the ordinal `8va` and `15ma` markups that match Bravura's glyphs |
| `figured-bass` | [Figured bass](https://lilypond.org/doc/v2.26/Documentation/notation/figured-bass), including stacks, accidentals before a numeral, and an accidental alone |
| `percussion-clef` | [Percussion staves](https://lilypond.org/doc/v2.26/Documentation/notation/common-notation-for-percussion), in LilyPond's default drum style: bass drum, snare, X-headed hi-hat on E5, and a circled-X crash cymbal on G5 |

Regenerate and verify the references with an installed LilyPond 2.26.0:

```sh
scripts/test-lilypond-ab.sh --update
scripts/test-lilypond-ab.sh
```

The normal visual-regression gate does not need LilyPond installed. It uses the
committed reference SVGs and compares the complete `tests/test.pdf` raster.
