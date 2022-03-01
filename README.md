# YAREGB 🦀🎮

## About
___
YAREGB is **Y**et **A**nother **R**ust **E**mulator (for) **G**ame **B**oy.

This is a study project and HEAVILY borrows code and observation from [rylev's book](https://github.com/rylev/DMG-01), [mooneyegb](https://github.com/Gekkio/mooneye-gb) and [rusty-boy](https://github.com/kbernst30/rusty-boy). Most of the documentation and code below to these and many other open source projects.

## Goals
___
The main goals of this project are:

* Getting started in Rust and mess with pointers and bitwise operations
* Learn about and document my findings regarding DMG internals, Z80 and ASM
* Implement build [blargg test roms](https://gbdev.gg8.se/wiki/articles/Test_ROMs)
* Implement a debugger  
* Mess around with Reverse Engineering GB ROMs and hardware


## To do
__
- [ ] Documentation
    - [ ] Add schematics and file layout
    - [ ] Add commentaries regarding RAM addr
- [ ] CPU
    - [x] Registers
    - [ ] Flags
    - [ ] Instruction Set 
    - [x] Timer and Divider Registers
    - [ ] Interrupts
- [ ] GPU
    - [ ] Rendering (Tile RAM)
- [ ] I/O
    - [x] Joypad Input
    - [ ] Sound Controller
    - [ ] Serial Data Transfer (Game Link)
- [ ] Memory Maps
- [ ] Tests
    - [ ] Add blargg test roms
- [ ] Debugger
    - [ ] Implement a debugger