
use std::io;

fn main(){
    println!("Guess tha number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line.");

    println!("You guessed: {}", guess);
}

// let で変数束縛（変数束縛の意味は、パターンなどを使えるという意味合い？
// mut キーワードによってミュータブル（可変）な変数束縛となる
//
// String は標準ライブラリによって提供される文字列型
// 可変長文字列を扱うことができr UTF-8 でエンコードされている
// コロンふたつ（::）は static な参照であり、上記の場合インスタンスを生成しているわけではない
//
// std::io::stdin() は標準入力へのハンドル（インスタンス）を返す
// このインスタンスの持つ read_line を使うことで入力を受付けている
//
// expect は read_line が返す io::Result を処理する
// Result の中身が正しくない時、expect は引数に受け取った文字列と共に panic! 、つまり意図的に落ちる。
//
// println の中でプレースホルダ {} を使うと、そこに続く引数を与えて出力が行える。
