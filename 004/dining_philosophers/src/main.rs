
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Table {
    forks: Vec<Mutex<()>>
}

struct Philosopher {
    name: String,
    left: usize,
    right: usize
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right
        }
    }
    fn eat(&self, table: &Table) {
        println!("{} begin eating.", self.name);

        let _left = table.forks[self.left].lock().unwrap();

        thread::sleep(Duration::from_millis(150));

        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating now.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let table = Arc::new(Table{forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(())
    ]});
    let phis = vec![
        Philosopher::new("zeros", 0, 1),
        Philosopher::new("ones", 1, 2),
        Philosopher::new("twos", 2, 3),
        Philosopher::new("threes", 3, 4),
        Philosopher::new("fours", 0, 4)
    ];

    let handles: Vec<_> = phis.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}

// Rust では、関数がなにかしらの値を返すときに return を書かない。
// ただし書くことができないわけではなく、早期にリターンしたい場合などに使うことはできる。
// そして注意すべきなのが、その「return を書かなくても戻り値が返る」という仕組みの本質。
//
// まず、式、そして文、というふたつの概念がある。
// 式とは要するに「a + b」と言うような、文字通りの式である。つまり式は結果を「返す」存在と言える。
// 一方の文とは、文字通り文章なので、これが結果を返すかというと、返さない。
//
// この前提を踏まえて考えるとき、Rust におけるセミコロンの役割とは何かを考えてみると……
// Rust におけるセミコロンとは、式と式をつないで文としていくための区切り文字、である。
// 文末、とか、式の終わりの印とか、そういう意味は持ち合わせていない。
//
// なので、たとえば以下のように……
// fn hoge(fuga: i32) -> i32 {
//     fuga * 2;
// }
// return を使わずにセミコロンを用いた場合、式にさらに何か式を連結して文にしようとしている、と解釈
// され実際には何も返すものがなくなってしまう。これを検出したコンパイラがエラーを吐く。
//
// 要約すると、Rust においては関数の戻り値は「式の結果」が返ることになっている。
// Rust は式ベースの言語であり、ほとんどのものが式として記述されることから、関数はデフォルトで最後
// に評価した式の結果を暗黙に return するようになっている、というわけ。
//
// Vec<_> という表記では、これがベクトルであることを宣言しつつ、その型についてはアンダースコア _ で
// 表すことによって「明示的に」アノテーション（つまり型推論）を示している。
//
// Rust では宣言しているのに使われなかった変数に対してコンパイラが自動的に警告を出す。
// これは、今回の用にスレッドをロックするための代入のみが必要な場合（ようわからん）などに、過剰に無
// 駄な警告を発生させることになる。このような場合は、変数束縛にアンダースコアから始まる変数名を使う
// ことで、コンパイラに対して警告が不要であることを伝えることができる。
//
