# ddlc_helper

[![crates.io](https://img.shields.io/crates/v/ddlc_helper.svg)](https://crates.io/crates/ddlc_helper)

DDLC_Helper 是心跳文学部（Plus）的选词工具。

可以在 [Github](https://github.com/poly000/ddlc_helper/releases) 上下载windows的构建。

## 编译

```
cargo +nightly build --release
```

本项目在版本 `2.0.0` 以后使用了不稳定的特性 [once_cell](https://github.com/rust-lang/rust/issues/74465) 。

## 用法

0. 选择您喜欢的角色 (例如 纱世里（Sayori）)
1. 选词时，您需要将 DDLC 的语言设置为 __English__。
2. 使用OCR工具识别游戏中的单词（比如，ShareX）。
3. 将刚刚得到的单词输入到本工具。这些单词应该以 “空白” 分隔开。“空白” 可以是换行符，空格，制表符等等。
4. 选择本工具以 “Result:” 前缀输出的单词。

## 注意

本工具 **不区分** 字母大小写，即使输出总是小写。
