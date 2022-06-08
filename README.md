🚧 UNDER CONSTRUCTION 🚧

# h2s

[![Check](https://github.com/ikenox/h2s/actions/workflows/check.yml/badge.svg?branch=main)](https://github.com/ikenox/h2s/actions/workflows/check.yml) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A declarative HTML parser library in Rust, that works like a deserializer from HTML to struct.

## Example

```rust
#![feature(generic_associated_types)]

use h2s::FromHtml;

#[derive(FromHtml, Debug, Eq, PartialEq)]
pub struct Page {
    #[h2s(select = "div > h1.blog-title")]
    blog_title: String,
    #[h2s(select = ".articles > div")]
    articles: Vec<Article>,
}

#[derive(FromHtml, Debug, Eq, PartialEq)]
pub struct Article {
    #[h2s(select = "h2 > a")]
    title: String,
    #[h2s(select = "h2 > a", attr = "href")]
    url: String,
    #[h2s(select = "p.modified-date")]
    modified_date: Option<String>,
    #[h2s(select = "ul > li")]
    tags: Vec<String>,
}

fn main() {
    let html = r#"
<html>
<body>
<div>
    <h1 class="blog-title">My tech blog</h1>
    <div class="articles">
        <div>
            <h2><a href="https://example.com/1">article1</a></h2>
            <p class="modified-date">2020-05-01</p>
            <ul><li>Tag1</li><li>Tag2</li></ul>
        </div>
        <div>
            <h2><a href="https://example.com/2">article2</a></h2>
            <p class="modified-date">2020-03-30</p>
            <ul></ul>
        </div>
        <div>
            <h2><a href="https://example.com/3">article3</a></h2>
            <ul><li>Tag3</li></ul>
        </div>
    </div>
</div>
</body>
</html>
    "#;
    
    let page: Page = h2s::util::parse(html).unwrap();
    assert_eq!(page, Page {
        blog_title: "My tech blog".into(),
        articles: vec![
            Article {
                title: "article1".into(),
                url: "https://example.com/1".into(),
                modified_date: Some("2020-05-01".into()),
                tags: vec!["Tag1".into(), "Tag2".into()]
            },
            Article {
                title: "article2".into(),
                url: "https://example.com/2".into(),
                modified_date: Some("2020-03-30".into()),
                tags: vec![]
            },
            Article {
                title: "article3".into(),
                url: "https://example.com/3".into(),
                modified_date: None,
                tags: vec!["Tag3".into()]
            },
        ]
    });
}
```

You can see more examples at [examples/](./examples/).
