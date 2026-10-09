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
showcase. `beethoven-moonlight-coda.typ` sets measures 178–185 of the Presto
agitato from Op. 27 No. 2, where the right hand's arpeggios cross onto the bass
staff; it is translated from the
[Mutopia edition](https://www.mutopiaproject.org/ftp/BeethovenLv/O27/moonlight/)
and exercised by the visual regression suite and the user guide.

Mutopia marks the Mozart and Bach typesets as public domain. The Chopin
transcription is distributed by Mutopia under CC BY-SA 3.0; the fixture records
Mutopia and its 1881 Schirmer source accordingly. The Moonlight Sonata edition
by Chris Sawer and Stewart Holmes is distributed by Mutopia under CC BY-SA 2.5,
after Berners (1908); its fixture records that source.

The README gallery in `assets/readme/examples/` adds four cards. The Eine kleine Nachtmusik card imports this folder's Mozart fixture with `bar-count: 4`, so its measures 1–4 match the showcase transcription above. Its Fantaisie-Impromptu excerpt (bars 5–8 of Op. 66) follows Mutopia's public-domain [piece 1693](https://www.mutopiaproject.org/cgibin/piece-info.cgi?id=1693), edition by Herrmann Scholtz; its LilyPond reference is `lilypond-reference/chopin-fantaisie-impromptu-op66.ly`. The Carcassi Prelude in C (Op. 59 No. 1) follows Mutopia's public-domain [piece 2209](https://www.mutopiaproject.org/cgibin/piece-info.cgi?id=2209), the 1906 Ditson edition by G. C. Santisteban, typeset by Jeffrey Olson. The guitar voice applies `\transpose c c'`, so the file records sounding pitches one octave below the printed ones. The Five-Four Vamp is an original 5/4 piece, not a transcription, so it has no external source. Each file header records its source and licence.

## LilyPond engraving references

`lilypond-reference/` contains a capability-matched `.ly` translation and a
committed LilyPond SVG for every showcase page. These compact sources reproduce
the notes, supported marks, instrumentation, and system divisions in the
typed-scores fixtures, making score-level engraving differences easy to inspect
without placing both renderers side by side.
