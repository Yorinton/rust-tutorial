use std::io;

fn main() {
    println!("Let's guess the number!");

    println!("Please input your guess");

    // ユーザー入力を保持するための領域を確保
    // String::new()はString型のオブジェクトを返す
    // String型は サイズ可変、UTF-8エンコードされたテキスト破片
    let mut guess = String::new();

    // 特定のテキストからString型のオブジェクトを生成する
    let _ = String::from("テキスト破片");
    
    // 標準入力からの入力値を読み込んでguessに
    // &mutの&は参照であることを表す
    // mutをつけることで可変な参照であることを表す
    // read_lineの返り値であるio::Result型はOk型とErr型の列挙子(Variant)を持つenum
    // io::Result型に対して.expectを実行することでErrの場合はpanicを起こし、成功した場合はResultが保持するOk値を返す
    // 今回の場合のOk値は標準入力から入力されたテキストのバイト数になる
    // 改行区切り文字（0xAバイト）またはEOFが見つかるまでバイトを読み込むため、攻撃者が改行やEOFを送信せずにバイトを送り続ける可能性もある
    let byte_num = io::stdin().read_line(&mut guess).expect("failed to read line");

    // :{}の中に指定した変数に入った値が入る
    println!("入力文字列 :{}バイト数:{}", guess, byte_num);
}
