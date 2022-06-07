🚧 UNDER CONSTRUCTION 🚧

# h2s

[![Check](https://github.com/ikenox/h2s/actions/workflows/check.yml/badge.svg?branch=main)](https://github.com/ikenox/h2s/actions/workflows/check.yml) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A declarative HTML parser library in Rust, that works like a deserializer from HTML to struct.

## Example

```rust
#![feature(generic_associated_types)]

use h2s::FromHtml;

#[derive(FromHtml, Debug)]
pub struct Page {
    #[h2s(select = "div > h1.blog-title")]
    blog_title: String,
    #[h2s(select = ".articles > div")]
    articles: Vec<ArticleElem>,
}

#[derive(FromHtml, Debug, Eq, PartialEq)]
pub struct ArticleElem {
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
<!DOCTYPE html>
<html lang="en">
<body>
<div>
<h1 class="blog-title">My tech blog</h1>
<div class="articles">
    <div>
        <h2><a href="https://example.com/1">article1</a></h2>
        <ul>
            <li>Tag1</li>
            <li>Tag2</li>
        </ul>
    </div>
    <div>
        <h2><a href="https://example.com/2">article2</a></h2>
        <ul></ul>
    </div>
    <div>
        <h2><a href="https://example.com/3">article3</a></h2>
        <ul>
            <li>Tag3</li>
        </ul>
        <p class="modified-date">2020-05-01</p>
    </div>
</div>
</div>
</body>
</html>
    "#;

    let page: Page = h2s::util::parse(html).unwrap();
    assert_eq!(page.blog_title.as_str(), "My tech blog");
    assert_eq!(
        page.articles.get(0).unwrap().tags,
        vec!["Tag1".to_string(), "Tag2".to_string()]
    );
}
```

You can see more example  [examples/](./examples/)
