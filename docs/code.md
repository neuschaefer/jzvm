# Code structure

- libjzvm
- `jzvm` tool
  - `info` subcommand
  - loading and running class files?
- tests


## Code buffer

- state:
  - empty
    - allocate -> writable
  - writable
    - get pointer, write bytes
    - make_executable -> executable
  - executable


## Execution context

- multiple per context, this way we can have threads and all that


## VM run

- actually running some code and handling the results
- multiple backends on different hardware implementations:
  - emulation (works everywhere)
  - ARM
    - ARM9
    - ARM11
    - trivial (only the BXJ entry works, none of the instructions do)


## class loading

- parsing
  - available crates (to be evaluated):
    - **cafebabe**: favorite
    - **classreader**: ok
    - **java_class_parser**: looks lacking
    - **classfile-parser**: ok
    - **classfmt**: "not even remotely production-ready"
    - **class2json**: not that i'd want JSON...
    - **class_file**: not sure about that one, and it's unmaintained
    - **jvm-assembler**: no readme, lol
- method resolution
- JNI stuff
