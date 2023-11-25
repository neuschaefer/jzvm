## References

- <https://web.archive.org/web/20211120172822/https://en.wikipedia.org/wiki/Jazelle>
- <https://hackspire.org/index.php/Jazelle>
- <https://github.com/SonoSooS/libjz/wiki/BXJ-and-Registers>



## Implementations

- Inside ARM926EJ-S (0x64100004): fully featured
- Inside Cortex-A8: trivial
- Inside QEMU: trivial but there might be slight differences

## Tools

- jzinfo
  - Decode Jazelle identity register, possibly with some commentary
  - e.g. `ARM926EJ-S (0x64100004)`
  - e.g. `Unknown (0x6410dead)`
  - e.g. `Emulation`
- jztest
  - test suite that should fully characterize the jazelle implementation it runs on
  - see below
  - different known outcomes for different implementations

### Publication

- License? LGPL?


## Documentation

- Documentation of the project
- Documentation of jz
  - throughout the tests
  - but also elsewhere?
- good license for documentation


## Implementation strategy

- Write tests to explore everything
- each test:
  - precondtions
  - expected outcome
  - execute jz code
  - compare
- low-level jz entry function
  - save register state
  - load precondition state
  - execute
  - save outcome state
  - restore old state
- handlers
  - calling C directly is probably a bad idea because of register clashes
  - low-level handlers
    - determine handler identity
    - save registers
    - call high-level handler (written in C)
    - depending on high-level handler's decision,
      - reenter jz mode
      - break from jz mode and return from entry function
  - high-level handlers
    - get handler identity (e.g. undefined opcode 0xac)
    - get current jz state (read/write)
    - determine jz resumption/exit
- time-out mechanisms, necessary for endless-loop tests
  - something with signals, i guess?
    - also useful for other signals such as SIGSEGV
  - make entry function return somehow


## Possible tests

I want tests for every detail of jazelle's behavior.


- permit no unknown hardware implementations
- Alignment of r5 (handler table and other bits)
  - 10 bits or 12 bits?
  - check all claims made by SonoSooS and Hackspire
- precise stack behavior
  - Distance between handler table and stack (max. 0xfffc?)
    - are all bits of r6/r7 relevant?
      - do 31:12 always need to be the same as in r5?
        - i.e. tables located in the same 4k page
    - distance vs. alignment
      - how are the boundaries defined exactly?
    - what if the stack is overflowed?
      - endless-loop behavior?
        - does it matter if the memory after the max distance is accessible?
  - stack spills
    - vs. end
    - at handler entry
    - at other events?
- can jazelle be exited without handlers?
- every opcode's bahavior
  - what about undefined opcodes?
  - jsr/ret format
  - array format
    - settings documented by hackspire
- handlers
  - executed in the "trivial implementation"?
  - state at entry/exit?
  - stack spilling
    - only for undef handlers?
  - how is instruction length determined?
    - in hardware or in software?
    - i.e. can we set our own?
  - can i override instructions that the hardware supports?
  - div-by-zero
- jazelle configuration registers versus context switches
  - does linux (need to) save/restore state?
  - can i run two jz-enabled programs alongside each other?
    - will this create a side channel?
- undefined opcodes (0xf0) vs. unimplemented opcodes (ireturn/0xac)
- how does it crash when certain tables are not set?
  - handler table
  - stack
  - locals
- effect of JE bit on BXJ
- demand paging triggered by Jazelle should complete instead of stalling the
  process at 100% CPU
