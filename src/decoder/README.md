# decoder

Just a decoder for the Gameboy's LR35902.

What makes this module interesting is that I managed to implement
the primary decoding functionality in a const function. That means
you can potentially decode games at compile time and ship them that way...

### sources

* https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html,
  for decoder pattern matching and bit masks
* http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf