# Famous-score showcase

Compile the three-page showcase with:

```sh
typst compile --root . examples/showcase.typ examples/showcase.pdf
```

Each page translates a public LilyPond source into typed-scores notation. Each
music file also exports one reusable Typst function, and the excerpts stay
within the package's current one-rhythmic-voice-per-staff model.

| Example | Scope | Reference |
|---|---|---|
| `chopin-opening.typ` | Piano; pickup and measures 1–8 of Nocturne Op. 9 No. 2 | [Mutopia transcription](https://www.ibiblio.org/mutopia/cgibin/piece-info.cgi?id=1590), sourced from G. Schirmer (1881) |
| `mozart-eine-kleine-nachtmusik.typ` | Four-staff string score; measures 1–10 of K. 525/I | [Mutopia LilyPond sources](https://www.ibiblio.org/pub/multimedia/mutopia/MozartWA/KV525/eine-kleine-nachtmusik-mvt1/eine-kleine-nachtmusik-mvt1-lys/) |
| `bach-cello-suite-prelude.typ` | Solo cello; measures 1–16 of BWV 1007/I | [Mutopia edition and LilyPond download](https://www.ibiblio.org/mutopia/cgibin/piece-info.cgi?id=517) |

The shorter Beethoven Ode to Joy and Für Elise examples remain available as
standalone source files, but are intentionally omitted from the full-page
showcase.

Mutopia marks the Mozart and Bach typesets as public domain. The Chopin
transcription is distributed by Mutopia under CC BY-SA 3.0; the fixture records
Mutopia and its 1881 Schirmer source accordingly.

## LilyPond engraving references

`lilypond-reference/` contains a capability-matched `.ly` translation and a
committed LilyPond SVG for every showcase page. These compact sources reproduce
the notes, supported marks, instrumentation, and system divisions in the
typed-scores fixtures, making score-level engraving differences easy to inspect
without placing both renderers side by side.
