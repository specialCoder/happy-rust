# happy-rust
rust 学习笔记

## 安装
### 在Linux 或 macOS 中安装 rustup
在终端输入下面命令：
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
安装成功之后会显示：
```bash
Rust is installed now. Great!
```
查看安装的版本：
```bash
$ rustc --version
```

### 快速开始
#### 创建 `.rs` 文件
首先创建文件目录：
```bash
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```
创建 main.rs 文件，文件内容：
```rs
fn main() {
    println!("Hello, world!");
}
```
#### 编译文件
```bash
$ rustc main.rs
```
当前文件夹下会生成一个可执行文件`main文件`
#### 执行文件
```bash
$ ./main
```
控制台输出：
```
Hello, world!
```

### 使用 Cargo 工具
#### 安装
在安装 Rust的时候会默认安装 Caro,检查 Cargo 是否已经安装：
```bash
$ cargo --version
```
#### 创建工程
```bash
$ cargo new hello_cargo
$ cd hello_cargo
```
文件结构：
```
hello_cargo
|
├── src
│   |
│   └── main.rs                 - application entry
├── target                      - target files
│   ├── debug/                   - debug files
│   └── release/               - release files
├── Cargo.toml                  - configuration
├── Cargo.lock
```
#### 编译 & 执行
使用 `cargo run` 对 `rs` 文件进行编译执行：
```
$ cargo run
```
对 `src/main.rs`  进行编译，控制台输出：
```
Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
Running `target/debug/hello_cargo`
```
成功之后自动执行，控制台输出：
```
Hello, world!
```
#### 项目构建
使用 Cargo 命令构建：
```bash
$ cargo build
```
控制台输出：
```
Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```
编译成功之后，在target/debug 文件夹里生成可执行文件 `hello_cargo文件`。
执行构建出来的文件：

```bash
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
```
控制台输出：
```
Hello, world!
```

## 参考资料
- [Rust 中文官网](https://www.rust-lang.org/zh-CN/)
- [Rust程序设计语言](https://kaisery.github.io/trpl-zh-cn/title-page.html)
- [The Rust Programming Language](https://doc.rust-lang.org/book/index.html)
- [通过例子学 Rust](https://rustwiki.org/zh-CN/rust-by-example/)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
- [Rust small exercises](https://github.com/rust-lang/rustlings/)
- [在线运行 Rust](https://play.rust-lang.org/)
