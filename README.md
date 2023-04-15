*介绍
这是一个frame aware scheduling调度的框架

*使用
下载最新release，用magisk安装

*编译
1. clone仓库
```
git clone https://github.com/shadow3aaa/fas-framework
cd fas-framework
```
2. 安装rust(如果没有)
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | s
```
3. 安装静态编译用target(如果没有)
```
rustup target add aarch64-unknown-linux-musl
```
4. 编译
```
# debug
cargo build --target aarch64-unknown-linux-musl
# release
cargo build --target aarch64-unknown-linux-musl --release
# Or
bash ./build.sh
# 编译生成的二进制文件在fas-framework/target/aarch64-unknown-linux-musl/release/下
```