[![Check](https://github.com/ikenox/h2s/actions/workflows/check.yml/badge.svg?branch=main)](https://github.com/ikenox/h2s/actions/workflows/check.yml) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) ![Rustc Version 1.65+](https://img.shields.io/badge/rustc-1.65+-bc71d0.svg)

# h2s

A declarative HTML parser library in Rust, which works like a deserializer from HTML to struct.

## Example

```rust
use h2s::FromHtml;

#[derive(FromHtml, Debug, Eq, PartialEq)]
pub struct Page {
    #[h2s(attr = "lang")]
    lang: String,
    #[h2s(select = "div > h1.blog-title")]
    blog_title: String,
    #[h2s(select = ".articles > div")]
    articles: Vec<Article>,
}

#[derive(FromHtml, Debug, Eq, PartialEq)]
pub struct Article {
    #[h2s(select = "h2 > a")]
    title: String,
    #[h2s(select = "div > span")]
    view_count: usize,
    #[h2s(select = "h2 > a", attr = "href")]
    url: String,
    #[h2s(select = "ul > li")]
    tags: Vec<String>,
    #[h2s(select = "ul > li:nth-child(1)")]
    first_tag: Option<String>,
}

let html = r#"
<html lang="en">
<body>
  <div>
      <h1 class="blog-title">My tech blog</h1>
      <div class="articles">
          <div>
              <h2><a href="https://example.com/1">article1</a></h2>
              <div><span>901</span> Views</div>
              <ul><li>Tag1</li><li>Tag2</li></ul>
          </div>
          <div>
              <h2><a href="https://example.com/2">article2</a></h2>
              <div><span>849</span> Views</div>
              <ul></ul>
          </div>
          <div>
              <h2><a href="https://example.com/3">article3</a></h2>
              <div><span>103</span> Views</div>
              <ul><li>Tag3</li></ul>
          </div>
      </div>
  </div>
</body>
</html>
"#;

let page: Page = h2s::parse(html).unwrap();

assert_eq!(page, Page {
    lang: "en".to_string(),
    blog_title: "My tech blog".to_string(),
    articles: vec![
        Article {
            title: "article1".to_string(),
            url: "https://example.com/1".to_string(),
            view_count: 901,
            tags: vec!["Tag1".to_string(), "Tag2".to_string()],
            first_tag: Some("Tag1".to_string()),
        },
        Article {
            title: "article2".to_string(),
            url: "https://example.com/2".to_string(),
            view_count: 849,
            tags: vec![],
            first_tag: None,
        },
        Article {
            title: "article3".to_string(),
            url: "https://example.com/3".to_string(),
            view_count: 103,
            tags: vec!["Tag3".to_string()],
            first_tag: Some("Tag3".to_string()),
        },
    ]
});
```

## Supported types

You can use the following types as a field value of the struct to parse.

### Basic types

  - `String`
  - Numeric types ( `usize`, `i64`, `NonZeroU32`, ... )
  - And more built-in supported types ([List](./core/src/from_text.rs))
  - Or you can use any types by implementing yourself ([Example](./examples/from_text_custom.rs))

### Container types (where `T` is a basic type)

  - `[T;N]`
  - `Option<T>`
  - `Vec<T>`

## License

MIT
