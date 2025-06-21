use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

#[derive(EnumString, EnumIter, Display, AsRefStr, Debug)]
enum Color {
    Red,
    Green,
    #[strum(serialize = "blue")]
    Blue,
}

fn main() {
    // 字符串转为枚举；区分大小写，默认只识别和定义完全相同的字符串
    let c = Color::from_str("Red").unwrap();
    println!("{:?}", c);
    // 遍历
    Color::iter().for_each(|c| {
        println!("Color has : {:?}", c);
    });
    // 自定义字符串映射
    let b = Color::from_str("blue").unwrap();
    println!("{:?}", b);
}