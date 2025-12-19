pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 【实现Trait】 为 NewsArticle 实现 Summary Trait
impl Summary for NewsArticle {
    // 实现这个方法
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 【实现Trait】 为 Tweet 实现 Summary Trait
impl Summary for Tweet {
    // 实现这个方法
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 【定义】Trait
pub trait Summary {
    // 不写方法体 （没有默认实现 也可以有，后续实现 Trait可以保留或者覆盖）
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        // format!("(Read more from ...)")
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}
