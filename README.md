### References
- Writing an OS in Rust [edition1](https://os.phil-opp.com/edition-1/) [edition2](https://os.phil-opp.com/)


### Run

```bash
cargo build

cargo install bootimage
rustup component add llvm-tools-preview

qemu-system-x86_64 -drive format=raw,file=target/x86_64-xm_os/debug/bootimage-xm_os.bin

or

cargo run
```