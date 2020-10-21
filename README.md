# vcd-rust
![build](https://img.shields.io/github/workflow/status/SeanMcLoughlin/vcd-rust/Rust%20checks)
![license](https://img.shields.io/github/license/SeanMcLoughlin/vcd-rust?logo=MIT)
[![](https://tokei.rs/b1/github/SeanMcloughlin/vcd-rust?category=lines)](https://github.com/SeanMcLoughlin/vcd-rust)

A value change dump parser for the Rust programming language.

The goal for this project is to parse VCD files into a similar data structure as 
[yaml-cpp](https://github.com/jbeder/yaml-cpp) does for YAML files.

### What is value change dump?
Value change dump is a file format specifying how a digital waveform appears. It's mainly used for 
digital waveform viewers.
You can read more about it on it's [Wikipedia article](https://en.wikipedia.org/wiki/Value_change_dump).
