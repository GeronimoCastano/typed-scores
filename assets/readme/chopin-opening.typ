// Compile with:
// typst compile --root . --ppi 220 assets/readme/chopin-opening.typ assets/readme/chopin-opening.png

#import "../../examples/chopin-opening.typ": chopin-opening

#set page(width: 13.5in, height: 4.25in, margin: 0.35in, fill: white)
#set text(font: "New Computer Modern")

#align(center + horizon)[
  #chopin-opening(
    scale: 0.82,
    note-spacing: 4.35,
    bar-count: 2,
  )
]
