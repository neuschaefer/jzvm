# jzvm - ARM Jazelle DBX experiments

_Project links: [repo] ([mirror]), [issues], CI: [ARM], [x86] ![badge]_

[repo]: https://codeberg.org/neuschaefer/jzvm
[mirror]: https://github.com/neuschaefer/jzvm
[issues]: https://codeberg.org/neuschaefer/jzvm/issues
[ARM]: https://codeberg.org/neuschaefer/jzvm/actions
[x86]: https://github.com/neuschaefer/jzvm/actions
[badge]: https://github.com/neuschaefer/jzvm/actions/workflows/github.yml/badge.svg


## Introduction

ARM Jazelle DBX ("direct bytecode execution")[^dbx] is a ARM32 instruction set
extension that allows running Java bytecode directly on certain CPUs.
Jazelle mode is entered by executing a special branch instruction, `BXJ`.

Jazelle hardware support is rather widespread within a certain timeframe of
manufacturing dates, since it was included in the ARM926EJ-S and ARM1176JZF-S
CPUs at the heart of many devices.

Jazelle software support is rather rare, because it presumably relies on a
proprietary JVM licensed by ARM, and the hardware isn't openly documented like
the rest of the ARM architecture(s). This is where `jzvm` comes in: It is an
attempt to create both an open source software implementation, and hardware
documentation for Jazelle.

[^dbx]: The _Jazelle_ brand has later been reused for an unrelated technology, _Jazelle RCT_. When I say _Jazelle_ it will mean _Jazelle DBX_ most of the time.


## License

The source code in this repository is licensed under the [LGPL 2.1]

[LGPL 2.1]: https://opensource.org/license/lgpl-2-1/


## References

- Wikipedia: <https://en.wikipedia.org/wiki/Jazelle>
- the Hackspire wiki: <https://hackspire.org/index.php/Jazelle>
- a previous attempt at using Jazelle: <https://github.com/SonoSooS/libjz/wiki/About>
- and an Advent-of-Code solution: <https://github.com/aspargas2/advent-of-code-2022/tree/main/day05-jazelle>
- ARMv5 Architecture Reference Manual [DDI0100I]: <https://developer.arm.com/documentation/ddi0100/i/>
