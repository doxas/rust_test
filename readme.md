
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




