# decoder

Mainly an iterator that takes in a slice of unsigned 8 bit values
and decodes them.

The opcode set in question is LR35902.

### sources

* https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html,
  for decoder pattern matching and bit masks
* http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf