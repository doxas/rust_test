
# rust_test

hello rust.

## install

run windows binary.

## vim

rust-lang/rust.vim

```
let g:quickrun_config={'*': {'split': ''}}
set splitbelow

autocmd BufNewFile,BufRead *.crs setf rust
autocmd BufNewFile,BufRead *.rs  let g:quickrun_config.rust = {'exec' : 'cargo run'}
autocmd BufNewFile,BufRead *.crs let g:quickrun_config.rust = {'exec' : 'cargo script %s -- %a'}
```

## command

* rustc source-file -> build only
* cargo build (need Cargo.toml) -> project debug build
* cargo build --release -> project release build and optimize
* cargo run -> debug build and run

Cargo.toml が Makefile と package.json みたいな感じ。

Cargo.lock には、package-lock.json と同じように依存関係などが出力される。

Cargo はまさに npm と同じような存在であり、Mozilla が作ってるだけあって JS のエコシステムに似ている。

* cargo new hello_world --bin

上記のコマンドを叩くと、ディレクトリ構成や Git の init なども含めて全部やってくれる。

npm の init とも似ているが、どちらかというとプロジェクトを生成するコマンドという感じ。








