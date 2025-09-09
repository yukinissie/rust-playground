# Rust Playground

Rust のマクロ機能を学習するためのプロジェクト。宣言型マクロと手続き型マクロの実装例を含んでいます。

## 機能

### 宣言型マクロ

- `vec!` マクロの再実装
- 複数の式を受け取り、`Vec`を作成する

### 手続き型マクロ

- `HelloMacro` カスタム derive マクロ
- 構造体に自動的に `hello_macro()` メソッドを実装する

## プロジェクト構成

```
rust-playground/
├── src/
│   └── main.rs           # メインプログラム（マクロの使用例）
├── hello_macro/
│   ├── src/
│   │   └── lib.rs        # HelloMacro トレイト定義
│   └── Cargo.toml
└── hello_macro_derive/
    ├── src/
    │   └── lib.rs        # 手続き型マクロの実装
    └── Cargo.toml
```

## 実行方法

```bash
cargo run
```

## 学習内容

このプロジェクトは [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-05-macros.html) のマクロの章を参考に作成されています。

- 宣言型マクロの基本的な記法
- 手続き型マクロの実装方法
- `syn` と `quote` クレートの使用方法
