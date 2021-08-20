cargo kbuild
cargo run --package boot
qemu-system-x86_64 -drive format=raw,file=target/x86_64-cherrykernel/debug/boot-bios-cherrykernel.img -serial stdio
