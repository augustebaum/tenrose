#{
  let p = plugin("./penrose_wasm.wasm")

  let trio = (
    substance: "
      Set A
      Label A $e=mc^2$
    ",
    style: "
      canvas {
        width = 150
        height = 150
      }
      forall Set A {
        center = (0, 0)
        Circle { 
          center: center
          r: 50
       }
        Equation { 
          center: center
          string: A.label
        }
      }
    ",
    domain: "type Set",
    variation: "test",
  )

  let trio_to_bytes = trio => {
    trio.pairs()
  }
  str(p.run(
    bytes(trio.substance),
    bytes(trio.style),
    bytes(trio.domain),
    bytes(trio.variation),
  ))
}
