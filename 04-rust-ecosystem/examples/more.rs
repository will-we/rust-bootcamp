use derive_more::{Display, Constructor, From};

#[derive(Display, Constructor, From)]
#[display(fmt = "用户 序号 : {}, 姓名 : {}", id, name)]  // 自定义输出格式
struct User {
    id: i32,
    name: String,
}
fn main() {
    // Constructor trait 使用
    let user1 = User::new(1, "张三".to_string());
    println!("{}", user1);
    // From trait 匹配元组
    let user2:User = (2, "王五".to_string()).into();
    println!("{}", user2);
}