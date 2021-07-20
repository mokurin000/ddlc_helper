# ddlc_helper

[![crates.io](https://img.shields.io/crates/v/ddlc_helper.svg)](https://crates.io/crates/ddlc_helper)

[**中文文档**](https://github.com/poly000/ddlc_helper/blob/main/README_cn.md)

DDLC_Helper is a tool to select words in DDLC(Plus).

Releases are avaliable on [Github](https://github.com/poly000/ddlc_helper/releases)

## Build

```
cargo +nightly build --release
```

This project used the unstable feature [once_cell](https://github.com/rust-lang/rust/issues/74465) after `2.0.0`.


## Usage

0. Select a charactor you like. (e.g. Sayori)
1. You should set language as __English__ in DDLC when selecting words.
2. Use OCR tool to recogonise words in the game. (e.g. ShareX)
3. Type the words we just got to this tool. (the words should be split in whitespaces like CRLF, Space, Tab, ...)
4. Select one of words ddlc_helper prints prefixed with "Result:".

## Notice

This tool is **NOT** letter-case sensitive, although the output should always be lowercase.
