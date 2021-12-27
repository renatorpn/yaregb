## Architecture

* CPU
* RAM
* ROM
* I/O
    * Screen
    * Sound
    * Gamepad

### CPU

Model: 8-bit 8080-like Sharp CPU (speculated to be a SM83 core) across all arch

CPU Freq: 4.194304MHz for DMG, check doc for expanded architectures

(source: https://gbdev.io/pandocs/Specifications.html)


| 16 Bit  |  Hi |  Lo  | Name / Function |
|:------:|:---:|-----:|:---------------:|
| AF     |  A  |  -  | Accumulator & Flags |
| BC     |  B  |  C  | BC |
| DE     |  D  |  E  | DE |
| HL     |  H  |  L  | HL |
| SP     |  -  |  -  | Stack Pointer |
| PC     |  -  |  -  | Program Counter/Pointer |

#### Flags

Bit 7: "zero"
Bit 6: "subtraction"
Bit 5: "half carry"
Bit 4: "carry"

```
   ┌-> Carry
 ┌-+> Subtraction
 | |
1111 0000
| |
└-+> Zero
  └-> Half Carry

-> the lower 4 bits (a.k.a the lower "nibble") must always be zeros.
```



Zero Flag (Z) -> This bit is set if and only if the result of an operation is zero. Used by conditional jumps.

