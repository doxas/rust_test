
struct Philosopher {
    name: String
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string()
        }
    }
    fn eat(&self) {
        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let phis = vec![
        Philosopher::new("zeros"),
        Philosopher::new("ones"),
        Philosopher::new("twos"),
        Philosopher::new("threes"),
        Philosopher::new("fours")
    ];

    for p in &phis {
        p.eat();
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
