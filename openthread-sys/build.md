# How to build

`cd ~`

## Install dependencies
`sudo apt update && sudo apt-get install -y autoconf automake autotools-dev curl python3 python3-pip python3-toml libmpc-dev libmpfr-dev libgmp-dev gawk build-essential bison flex texinfo gperf libtool patchutils bc zlib1g-dev libexpat-dev ninja-build git cmake libglib2.0-dev libslirp-dev clang`

## Install RISCV-Toolchain

`git clone https://github.com/riscv/riscv-gnu-toolchain`

`cd cd riscv-gnu-toolchain/`

`./configure --prefix=/opt/riscv --with-arch=rv32gc --with-abi=ilp32d`

`sudo make -j16`

## Install ARM-Toolchain

`wget https://developer.arm.com/-/media/Files/downloads/gnu/14.2.rel1/binrel/arm-gnu-toolchain-14.2.rel1-x86_64-arm-none-eabi.tar.xz`

`tar -xf arm-gnu-toolchain-14.2.rel1-x86_64-arm-none-eabi.tar.xz`

`sudo mv arm-gnu-toolchain-14.2.rel1-x86_64-arm-none-eabi /opt/`

`echo 'export PATH="/opt/riscv/bin:/opt/arm-gnu-toolchain-14.2.rel1-x86_64-arm-none-eabi/bin:$PATH"' >> ~/.bashrc`

## Add Rust-Targets

`rustup target add riscv32imac-unknown-none-elf`

`rustup target add thumbv7em-none-eabi`

`rustup target add thumbv6m-none-eabi`

## Install and run xtask

`cd /workspaces/openthread/`


`cargo install cargo-xtask`

`cargo xtast gen riscv32imac-unknown-none-elf`

`cargo xtast gen thumbv7em-none-eabi`

`cargo xtast gen thumbv6m-none-eabi`
