# rusty-riscv-ave: A simple riscv64 emulator in rust

inspired by rvemu and Rare.

Just for my personal study.

## TODOS
My implementation works fine in previous 9 steps, but failed to boot xv6's kernel in the last lab, 
so I use the cpu.rs from rare, it just works.

I'm trying to figure out the reason now, I'm so tired.

## Usage
```bash
cargo run --release <path-to-kernel-binary> <path-to-file-system>
```

## Tips
To compile xv6 for the emulator, you should use xv6's 2020 version, because the new version uses VIRTIO_VERSION 2, this is not supported.
```bash
git clone git://g.csail.mit.edu/xv6-labs-2020
cd xv6-labs-2020
```
You need to modify the makefile, add ` -march=rv64imazicsr -mabi=lp64` to CFLAGS and remove `asm volatile ("wfi")` in `kernel/proc.c` since we don't support riscv's D, F and C extension and the `wfi` instruction.

Additionally, you may have to compile the `kernel/*.S` files manually with the flag above.

Then do
```bash
make
llvm-objcopy --strip-all -O binary kernel/kernel ./kernel.bin
```

```bash
cargo run --release <path-to-kernel-binary> <path-to-file-system>
```
And enjoy it.
