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
    println!("vec is: {:?}", result)
}
