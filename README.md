# vcd_rust
![build](https://img.shields.io/github/workflow/status/SeanMcLoughlin/vcd_rust/Rust%20checks)
![license](https://img.shields.io/github/license/SeanMcLoughlin/vcd_rust?logo=MIT)
[![](https://tokei.rs/b1/github/SeanMcloughlin/vcd_rust?category=lines)](https://github.com/SeanMcLoughlin/vcd_rust)

A value change dump parser for the Rust programming language.

The goal for this project is to parse VCD files into a similar data structure as 
[yaml-cpp](https://github.com/jbeder/yaml-cpp) does for YAML files, create
understandable data structure representations of a VCD file, and provide clear error 
messages if parsing fails.

### What is value change dump?
Value change dump is a file format specifying how a digital waveform appears. It's mainly used for 
digital waveform viewers, among other electronic design automation (EDA) tools.
You can read more about it on it's [Wikipedia article](https://en.wikipedia.org/wiki/Value_change_dump).

### Example
The below file example is a simplified version of the file on the 
[VCD Wikipedia article](https://en.wikipedia.org/wiki/Value_change_dump).
Given this file named `example.vcd` with the contents:
```
$date August 9th, 2020 $end
$version 1.0 $end
$comment This is an example $end
$timescale 1 ps $end
$scope module top $end
$var wire 8 # data $end
$upscope $end
$enddefinitions $end
$dumpvars
bxxxxxxxx #
$end
#0
b10000001 #
#5
b10101010 #
```

This file can be parsed with the `load_from_file()` method:

```rust
extern crate vcd_rust;
use vcd_rust::{load_from_file, types::timescale::{TimeScale, TimeUnit}};

fn main() {
    let vcd = load_from_file("example.vcd").unwrap();
    assert_eq!(vcd.date, "August 9th, 2020"); // Date
    assert_eq!(vcd.version, "1.0"); // Version
    assert_eq!(vcd.comments, vec!["This is an example"]); // Comments as a vector
    assert_eq!(vcd.timescale, TimeScale::init(1, TimeUnit::PS)); // Custom type for timescale
    // ...among other data structures
}
```

Likewise, the string representation of a VCD file can be parsed with the `load_from_string()` method:

```rust
extern crate vcd_rust;
use vcd_rust::{load_from_string, vcd::VCD};

fn parse_vcd_string() -> VCD {
    let vcd_string = "$date August 9th, 2020 $end...";  // etc.
    return load_from_string(vcd_string).unwrap();
}
```

### References
* [Explanation of VCD format](https://web.archive.org/web/20120323132708/http://www.beyondttl.com/vcd.php)
