
// struct
struct VectorThree {
    x: f64,
    y: f64,
    z: f64,
}

// method
impl VectorThree {
    fn new(x: f64, y: f64, z: f64) -> Self {
        VectorThree {x: x, y: y, z: z}
    }
    fn length(&self) -> f64 {
        let l: f64 = self.x * self.x + self.y * self.y + self.z * self.z;
        l.sqrt()
    }
    fn normalize(&mut self) -> VectorThree {
        let l: f64 = self.length();
        self.x /= l;
        self.y /= l;
        self.z /= l;
        VectorThree {x: self.x, y: self.y, z: self.z}
    }
}

fn main(){
    let mut v = VectorThree::new(1.0, 1.0, 1.0);
    let l: f64 = v.length();
    println!("initial: {} {} {}: {}", v.x, v.y, v.z, l);

    let w: VectorThree = v.normalize();
    let l: f64 = w.length();
    println!("normalize: {} {} {}: {}", w.x, w.y, w.z, l);
}

// 上記のようなベクトル管理クラスを考えるとき、この VectorThree は自身のプロパティを持ち、
// normalize のようなメソッドを呼ぶと自身の内部プロパティを変更する可能性があり得る
//
// このときのポイントは、28 行目で v を mut として宣言していることがひとつ。
// そして、自身のプロパティを書き換える可能性のある normalize では、引数に &mut とミュータブルな
// 参照を使っていることである
//
// また、構造体そのものがミュータブルであるかどうかの情報を持つことはできない
// これは仕様だし、まあ考えてみれば当たり前である




