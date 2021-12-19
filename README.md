# rust-memo

## Installation

```console
$ git clone https://github.com/bouzuya/rust-memo
...
$ cargo install --path crates/application
...
$ rust-memo --version
...
```

## Usage

```console
$ mkdir your-memo-dir
$ cd your-memo-dir/
$ rust-memo server
```

## rust-memo とは何か

rust-memo とは何か。 Rust で書かれたメモ管理ソフトウェアだ。

なぜ Rust か。 Rust で書く理由は bouzuya が Rust を使う練習のためだ。

どのようなメモを管理するのか。

メモはテキストによるものだ。イメージなどのバイナリによるものには対応していない。

メモの文法は Markdown だ。一部特殊な解釈を加えているが文法は変わらない。

どのようにメモを管理するのか。

メモの保存先はファイルシステムだ。 1 ディレクトリにすべての Markdown ファイルを保存する。階層化されたディレクトリには対応しない。

メモのファイル形式はいまのところ Markdown によるテキストファイルのみだ。たとえばイメージファイルなどのバイナリファイルは対応していない。将来的には画像の添付などに対応するかもしれない。

メモのファイル名は YYYYMMDDTHHMISSZ の形式をとる。これはメモの作成日時である。

メモの参照は rust-memo の提供する HTTP サーバー経由で間接的に、あるいは保存されているファイルを直接的に参照できる。

メモの更新は保存されているファイルを任意のツールで編集することでできる。 (TODO: obsolete link による履歴管理の説明)

メモのリンクは特定のパスによってできる。メモ間はネットワーク構造をとる。 (TODO: パスの説明)

用途は個人的には読書記録などである。ただし特定の用途に特化しているわけではないため任意の用途に使用できる。

