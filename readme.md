# iCore

the iCore is a simple operation system core, for learning purposes.

## Learning path

- use the popular project [rCore-Tutorial-v3](https://rcore-os.github.io/rCore-Tutorial-v3/chapter0/0foreword.html) to learn the basic knowledge of operation system. Additionally, use the [rust](https://www.rust-lang.org/) language to implement the iCore. the doc pdf file in on the local [doc](./doc/rcore-tutorial-book-v3.pdf) folder.


## environment

here our system is Moc os X (aarch64) version.

### rcore-tutorial

- qemu-system-riscv64 `brew install qemu`
- gun toolchain for riscv64 `brew tap riscv-software-src/homebrew-riscv && brew install riscv-gnu-toolchain && brew install riscv64-elf-gdb`

rust toolchain install 

```shell
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
curl https://sh.rustup.rs -sSf | sh

rustup install nightly
rustup default nightly

rustup target add riscv64gc-unknown-none-elf
cargo install cargo-binutils
rustup component add llvm-tools-preview
rustup component add rust-src
```

plugin on the vscode, `rust-analyzer` and `RISC-V`.
