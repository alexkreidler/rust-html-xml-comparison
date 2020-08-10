use minidom::Element;

const DATA: &'static str = r#"<articles xmlns="article">
    <article>
        <title>10 Terrible Bugs You Would NEVER Believe Happened</title>
        <body>
            Rust fixed them all. &lt;3
        </body>
    </article>
    <article>
        <title>BREAKING NEWS: Physical Bug Jumps Out Of Programmer's Screen</title>
        <body>
            Just kidding!
        </body>
    </article>
</articles>"#;

const ARTICLE_NS: &'static str = "article";

#[derive(Debug)]
pub struct Article {
    title: String,
    body: String,
}

fn main() {
    let root: Element = DATA.parse().unwrap();

    let mut articles: Vec<Article> = Vec::new();

    for child in root.children() {
        if child.is("article", ARTICLE_NS) {
            let title = child.get_child("title", ARTICLE_NS).unwrap().text();
            let body = child.get_child("body", ARTICLE_NS).unwrap().text();
            articles.push(Article {
                title: title,
                body: body.trim().to_owned(),
            });
        }
    }

    println!("{:?}", articles);
}
