# oxvm

A virtual machine written in Rust

## binary format

```Rust
const MAGIC_NUMBER: u16 = 0xBADFAD;
struct Executable {
    magic_number: u16
    load_addr: u32
}
```
