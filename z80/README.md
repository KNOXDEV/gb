# z80

This crate simulates the processor in a Gameboy,
built using the amazing
[Gameboy GPU Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)
as a reference.

This crate is technically a misnomer since the Gameboy doesn't
contain a Zilog Z80 processor, but it's pretty close.

> The GameBoy uses a computer chip similar to an Intel 8080.
> It contains all of the instructions of an 8080 
> except there are no exchange instructions. In many 
> ways the processor is more similar to the Zilog Z80 
> processor. Compared to the Z80, some instructions
> have been added and some have been taken away.

See the [Gameboy CPU manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf) for more information.

## on memory maps

I have no idea if this is a novel structure, but on decoding of a rom, 