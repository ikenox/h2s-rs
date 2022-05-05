use h2s::H2s;

#[derive(H2s, Debug, Eq, PartialEq)]
pub struct Page {
    #[h2s(select = "h1.blog-title")]
    blog_title: String,
    #[h2s(select = ".articles > div > .detail > .author")]
    authors: Vec<String>,
    #[h2s(select = ".articles > div")]
    articles: Vec<ArticleElem>,
}

#[derive(H2s, Debug, Eq, PartialEq)]
pub struct ArticleElem {
    #[h2s(select = "h2")]
    title: String,
    #[h2s(select = ".detail > .author")]
    author: String,
}

fn main() {
    let html = r#"
<html>
<body>
<h1 class="blog-title">My tech blog</h1>
<div class="articles">
    <div>
        <h2>article1</h2>
        <div class="detail">
            <span class="author">ikeno</span>
            <span class="date">2020-01-01</span>
        </div>
    </div>
    <div>
        <h2>article2</h2>
        <div class="detail">
            <span class="author">alice</span>
            <span class="date">2020-02-01</span>
        </div>
    </div>
    <div>
        <h2>article3</h2>
        <div class="detail">
            <span class="author">bob</span>
            <span class="date">2020-03-01</span>
        </div>
    </div>
</div>
</body>
</html>
    "#;

    let res: Page = h2s::parse(html).unwrap();
    println!("{:#?}", res);
}
