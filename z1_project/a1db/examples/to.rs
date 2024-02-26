use taplo::parser::parse;
use serde::{Deserialize, Serialize};
use taplo::dom::node::DomNode;

fn main() {
    let parse_result = parse(source);
    println!("parse_result: {:?}", parse_result);
    // Check for syntax errors.
    assert!(parse_result.errors.is_empty());


    let node = parse_result.into_dom();
    println!("node: {:?}", node);
    let mut nodeObj = Node {
        value: "".to_string(),
        value2: "".to_string(),
        user: User { name: "".to_string(), age: 0 },
    };
    nodeObj = node.clone().try_into().unwrap();

    let binding = node.get("value");
    let value = binding.as_str().unwrap();
    println!("value: {:?}", value);
    let value2 = node.get("value2");
    println!("value2: {:?}", value2);

    let user = node.get("user");
    println!("user: {:?}", user);
}

#[derive(Debug, Deserialize, Serialize)]
struct Node {
    value: String,
    value2: String,
    user: User,
}


impl From<taplo::dom::Node> for Node {
    fn from(node: taplo::dom::Node) -> Self {
        let mut nodeObj = Node {
            value: "".to_string(),
            value2: "".to_string(),
            user: User { name: "".to_string(), age: 0 },
        };
        nodeObj = node.try_into().unwrap();
        nodeObj
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    age: i32,
}

const source: &str = r#"
value = "hello"
value2 = "world"
[user]
name = "tom"
age = 18
"#;