fn static_method() {
    let s = String::from("Hello, world!");
    println!("{} {}", s, s.len());
}

// 構造体って関数外で定義しても使えるんだね
struct Animal {
    name: String, // 文字列
    age: i32,     // 整数
    weight: f32,  // 浮動小数点
}

// 空の構造体
struct Marker;

enum Species {
    Dog,
    Cat,
    Fish,
}

struct Animal2 {
    species: Species,
    name: String,
    age: i32,
    weight: f32,
}

fn main() {
    static_method();

    // 構造体データをメモリ上で隣り合うように作成する
    let dog = Animal {
        name: String::from("dog"), // データメモリ上に格納する
        age: 2,
        weight: 3.0,
    };
    println!("{} {} {}", dog.name, dog.age, dog.weight);

    let _m = Marker; //

    let mike = Animal2 {
        species: Species::Cat,
        name: String::from("mike"),
        age: 2,
        weight: 3.0,
    };

    match mike.species {
        // mikeのspeciesに対してmatchを実行
        Species::Dog => println!("dog"),
        Species::Cat => println!("cat"),
        Species::Fish => println!("fish"),
    }
}

/*
    データメモリ
    - 固定長 or Staticなデータを格納する
        Staticなデータとはプログラムのライフサイクルで常に存在するデータである
        （ライフサイクルで破棄される変数もあるってことだよね）
    - メモリの位置が知られていて固定されているため、高速にアクセスできる


    スタックメモリ
    - 関数ないで宣言された変数
    - メモリの上の位置は変更されることがないため、コンパイラからするとチューニングができるため、高速にデータにアクセスできる

    ヒープメモリ
    - プログラムの実行時に作られるデータ
    - このメモリにあるデータは追加、移動、削除、サイズの変更が許されている
    - 動的であるため遅いと思われるが、これにより柔軟性が高くなる
    - allocation : deallocation


*/
