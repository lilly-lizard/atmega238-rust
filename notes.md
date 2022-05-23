# tutorial:

- https://snowgoons.ro/posts/2020-11-11-compiling-rust-for-arduino-nano-every-part-one/
- https://book.avr-rust.com/003-building-a-crate-for-avr.html

# avr-delay 0.3.7 llvm_asm! depreciation issue

- https://github.com/avr-rust/delay/issues/10
- https://github.com/avr-rust/delay/issues/19 (fix release issue)

# avr-delay no reg_iw class

- https://doc.rust-lang.org/stable/unstable-book/language-features/asm-experimental-arch.html

# "ran out of registers during register allocation" during std building for avr

- https://github.com/avr-llvm/llvm/issues/1 avr-llvm fix 2014/11/26
- https://github.com/avr-rust/rust-legacy-fork/issues/37 legacy avr-rust issue 2017/04/28
- https://lists.llvm.org/pipermail/llvm-bugs/2018-November/069379.html llvm issue 2018/11/04
- https://github.com/rust-lang/rust/issues/88252 rust issue 2021/08/23
- https://github.com/avr-rust/ruduino/issues/42 ruduino "out of registers" identified 2022/03/03
- https://github.com/avr-rust/delay/issues/21 avr-delay release build block 2022/05/23

## TODO:
- try older nightly toolchains
- submit patch to replace llvm_asm! with asm! for ruduino

# language item required eh_personality

- atmega328p-testing project
```sh
cargo build -Z build-std=core --target avr-unknown-gnu-atmega328 --release
```
- because we're building an executable without the standard library (libcore)
- https://docs.rust-embedded.org/book/intro/no-std.html
- https://doc.rust-lang.org/unstable-book/language-features/lang-items.html
- https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html
