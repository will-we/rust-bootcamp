use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    birthday: DateTime<Utc>,
    skills: Vec<String>,
}

fn main() {
    let user = User {
        name: "张三".to_string(),
        birthday: Utc::now(),
        skills: vec!["running".to_string(), "music".to_string()],
    };

    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json);
    
    let user1:User= serde_json::from_str(&json).unwrap();
    println!("user1: {:?}",user1);
}
