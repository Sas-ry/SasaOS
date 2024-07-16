# SasaOS
Homemade OS created on the basis of [writing_os_1000lines](https://operationg-system-in-1000-lines.vercel.app/ja/welcom).
  
# Required
* Must use rust's nightly to run  
```
rustup install nightly
rustup override set nightly
rustup toolchain list
```
* Run the following command to add targets against the Rust toolchain
```
rustup target add riscv32i-unknown-none-elf
```

# Debugging tips
* Check register value
```
(qemu) info registers
CPU#0
 V      =   0
 pc       80200004
 mhartid  00000000
 mstatus  80006080
 mstatush 00000000
 hstatus  00000000
 vsstatus 00000000
 mip      00000000
 ・・・
```
* disassembly
```
// rustfilt requires cargo install rustfilt
llvm-objdump [project_root]/target/riscv32i-unknown-none-elf/debug/SasaOS | rustfilt
```