use std::io; // 標準入力出力ライブラリをインポート
use rand::Rng; // 乱数生成ライブラリをインポート
use std::cmp::Ordering; // 比較結果を表すOrdering列挙型をインポート

fn main() {
    println!("Guess the number!"); // ゲームの開始メッセージを表示

    let secret_number = rand::thread_rng().gen_range(1..=100); // 1から100までの乱数を生成し、秘密の数として設定

    loop {
        println!("Please input your guess."); // ユーザーに予想を入力するように促すメッセージを表示

        let mut guess = String::new(); // ユーザーの入力を格納するための可変の空の文字列を作成

        io::stdin()
            .read_line(&mut guess) // 標準入力からユーザーの入力を読み込み、guessに格納
            .expect("Failed to read line"); // 入力の読み込みに失敗した場合のエラーメッセージ

        let guess: u32 = match guess.trim().parse() { // 入力された文字列を整数に変換
            Ok(num) => num, // 変換が成功した場合、その数値を返す
            Err(_) => continue, // 変換が失敗した場合、ループの先頭に戻る
        };

        println!("You guessed: {}", guess); // ユーザーが入力した予想を表示

        match guess.cmp(&secret_number) { // ユーザーの予想と秘密の数を比較
            Ordering::Less => println!("Too small!"), // 予想が小さすぎる場合のメッセージを表示
            Ordering::Greater => println!("Too big!"), // 予想が大きすぎる場合のメッセージを表示
            Ordering::Equal => { // 予想が正解の場合
                println!("You win!"); // 勝利メッセージを表示
                break; // ループを終了
            }
        }
    }
}