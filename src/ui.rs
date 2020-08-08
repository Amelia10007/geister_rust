//! コンソールを利用したユーザとの基本的な対話機能を提供するクレート．

use std::fmt::Display;
use std::io;
use std::str::FromStr;

/// コンソールから値を入力させる．
/// ユーザが無効な入力をした場合は，有効な入力をさせ直す．
pub fn input_parsable<T, U>(message: U) -> T
where
    T: FromStr,
    T::Err: Display,
    U: Display,
{
    loop {
        println!("{}", message);
        match read_line() {
            Ok(line) => match line.parse() {
                Ok(parse_result) => return parse_result,
                Err(err) => {
                    println!("Invalid input!: {}", err);
                    continue;
                }
            },
            Err(err) => {
                println!("Invalid input!: {}", err);
                continue;
            }
        }
    }
}

/// 標準入力から1行取得する．
fn read_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    // 末尾の改行を取り除いたうえで返す
    Ok(buffer.trim_end().to_owned())
}
