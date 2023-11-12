# solana-example

Solana Developer Hub Onlineで使用するサンプルコードです。
https://lu.ma/52jro4af

**NOTE: このリポジトリは学習用であり意図的にバグを残しています**

## プロジェクトの構成

```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src
    ├── instruction.rs # プログラムに渡される指示の定義
    ├── lib.rs         # プログラムのエントリポイント
    ├── processor.rs   # 指示を処理するプログラムの実装
    └── state.rs       # Solanaネットワーク上に保存される状態の定義
```

## プログラムの概要

このプログラムは、Solanaネットワーク上に保存される状態を更新するプログラムです。
IncrementとDecrementの2つの指示を受け付け、カウントアップとカウントダウンを行います。
