
extern crate rand;

use std::io;
use rand::Rng;

fn main(){
    println!("Guess tha number!");
    println!("Please input your guess.");

    let secret = rand::thread_rng().gen_range(0, 101);

    println!("The secret number is: {}", secret);

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
//
// ----------------------------------------------------------------------------
//
// クレート（crate）は、いわゆるモジュールで Cargl.toml の dependencies にバージョンと共に記載する
// ことでバンドルされ、extern によってインポートされる。
// クレートが npm で言うところのパッケージであり、Create.io というパブリックなドメインがある。
//
// use rand::Rng は一見不要に思えるが、これはトレイトと呼ばれる型定義のようなものをインポートするた
// めに必要な記述。Rng というトレイト（定義）を参照するために、これを行う必要がある。
//
// rand::thread_rng() は現在のスレッドに乱数生成器をインスタンス化する。
// インスタンスは Rng トレイトがスコープに入っているため gen_range をコールすることができる。
// このメソッドは引数をふたつ持ち、第一引数に下限を、第二引数に上限を取る。（以上、未満）