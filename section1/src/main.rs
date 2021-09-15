fn main() {
    /* 変数 */
    let x = 5;
    println!("number is: {}", x);
    // デフォで不変なので↓はエラー
    // x = 6

    let mut y = 5;
    println!("number is: {}", y);
    // mutをつけてればおk
    y = 6;
    println!("number is: {}", y);

    /* コレクション */
    let source = vec![1, 2, 3, 4, 5];
    let result = source
        .into_iter()
        .filter(|n| n % 2 == 0)
        .map(|n| n.to_string())
        .collect::<Vec<String>>();
    println!("vec is: {:?}", result);

    /* 代数的データ型 */
    let objective: Option<i32> = Some(1);
    match objective {
        Some(x) if x % 2 == 0 => println!("偶数です: {}", x),
        Some(x) => println!("奇数です: {}", x),
        // まだ呼ばれる予定がない場合は、todo!()が使えるっぽい
        // Some(_x) => todo!(),
        None => println!("値がありません"),
    }

    // こうすると Vec<i32> と推論されるが…
    let mut v = vec![];
    v.push(1);
    // v.push(1.0);

    // 先に1.0を持ってくると、Vec<f64> と推論される
    let mut v = vec![];
    v.push(1.0);
    // v.push(1);

    /* トレイト */
    show_animal_data(Dog {});
    show_animal_data(Cat {});
}

trait Animal {
    // 動物の寿命を返す
    fn lifespan(&self) -> u32;
    // 動物の学術名を返す
    fn scientific_name(&self) -> String;
}

// 犬の構造体を用意
struct Dog;

// 犬の構造体に対するAnimalで定義した関数の定義
impl Animal for Dog {
    fn lifespan(&self) -> u32 {
        13
    }

    fn scientific_name(&self) -> String {
        "Canis lupus familiaris".to_string()
    }
}

// 猫の構造体を用意
struct Cat;

// 猫の構造体に対するAnimalで定義した関数の定義
impl Animal for Cat {
    fn lifespan(&self) -> u32 {
        16
    }

    fn scientific_name(&self) -> String {
        "Felis catus".to_string()
    }
}

// 動物の寿命と学術名を標準出力する関数
// T: Animal -> Animal トレイトを境界として持つものという意味
// [Q] ジェネリクスとはまた違うのか…？
fn show_animal_data<T: Animal>(animal: T) {
    println!("Lifespan: {} years", animal.lifespan());
    println!("Scientific name: {}", animal.scientific_name());
}
