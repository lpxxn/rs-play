use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;
    // 默认实现允许调用相同特征中的其他方法，哪怕这些方法没有默认实现。如此，特征可以提供很多有用的功能而只需要实现指定的一小部分内容
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!(" author: {}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{} by {} ", self.title, self.author)
    }
}
pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("username: {}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// impl Summary 比较好理解，但实际上他是一个语法糖，等价于下面的写法：
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
/*
真正的完整书写形式如上所述，形如 T: Summary 被称为特征约束。
多重约束
*/

pub fn notify3(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify4<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn random_article(random_number: f64) -> Box<dyn Summary> {
    if random_number < 0.5 {
        Box::new(Weibo {
            username: "li".to_string(),
            content: "haha rust".to_string(),
        })
    } else {
        Box::new(Post {
            title: "title".to_string(),
            author: "".to_string(),
            content: "".to_string(),
        })
    }
}

#[cfg(test)]
mod test_summary {
    use super::*;

    #[test]
    fn test_summary() {
        let post = Post {
            title: "title".to_string(),
            author: "author".to_string(),
            content: "content".to_string(),
        };
        println!("post: {}", post.summarize());

        let weibo = Weibo {
            username: "li".to_string(),
            content: "haha rust".to_string(),
        };
        println!("weibo: {}", weibo.summarize());

        println!("notify post: ---------------");
        notify(&post);
        notify(&weibo);
        notify2(&post);
        notify2(&weibo);
    }

    #[test]
    fn test_random_article() {
        let random_number = 0.3;
        let article = random_article(random_number);
        println!("article: {}", article.summarize());
    }
}