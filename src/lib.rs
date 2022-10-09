//! A declarative HTML parser library in Rust, that works like a deserializer from HTML to struct.
//!
//! # Example
//!
//! ```
//! use h2s::FromHtml;
//!
//! #[derive(FromHtml, Debug, Eq, PartialEq)]
//! pub struct Page {
//!     #[h2s(select = "div > h1.blog-title")]
//!     blog_title: String,
//!     #[h2s(select = ".articles > div")]
//!     articles: Vec<Article>,
//! }
//!
//! #[derive(FromHtml, Debug, Eq, PartialEq)]
//! pub struct Article {
//!     #[h2s(select = "h2 > a")]
//!     title: String,
//!     #[h2s(select = "div > span")]
//!     view_count: usize,
//!     #[h2s(select = "h2 > a", attr = "href")]
//!     url: String,
//!     #[h2s(select = "p.modified-date")]
//!     modified_date: Option<String>,
//!     #[h2s(select = "ul > li")]
//!     tags: Vec<String>,
//! }
//!
//! fn main() {
//!     let html = r#"
//! <html>
//! <body>
//! <div>
//!     <h1 class="blog-title">My tech blog</h1>
//!     <div class="articles">
//!         <div>
//!             <h2><a href="https://example.com/1">article1</a></h2>
//!             <div><span>901</span> Views</div>
//!             <ul><li>Tag1</li><li>Tag2</li></ul>
//!             <p class="modified-date">2020-05-01</p>
//!         </div>
//!         <div>
//!             <h2><a href="https://example.com/2">article2</a></h2>
//!             <div><span>849</span> Views</div>
//!             <ul></ul>
//!             <p class="modified-date">2020-03-30</p>
//!         </div>
//!         <div>
//!             <h2><a href="https://example.com/3">article3</a></h2>
//!             <div><span>103</span> Views</div>
//!             <ul><li>Tag3</li></ul>
//!         </div>
//!     </div>
//! </div>
//! </body>
//! </html>
//!     "#;
//!
//!     let page: Page = h2s::util::parse(html).unwrap();
//!     assert_eq!(page, Page {
//!         blog_title: "My tech blog".into(),
//!         articles: vec![
//!             Article {
//!                 title: "article1".into(),
//!                 url: "https://example.com/1".into(),
//!                 view_count: 901,
//!                 modified_date: Some("2020-05-01".into()),
//!                 tags: vec!["Tag1".into(), "Tag2".into()]
//!             },
//!             Article {
//!                 title: "article2".into(),
//!                 url: "https://example.com/2".into(),
//!                 view_count: 849,
//!                 modified_date: Some("2020-03-30".into()),
//!                 tags: vec![]
//!             },
//!             Article {
//!                 title: "article3".into(),
//!                 url: "https://example.com/3".into(),
//!                 view_count: 103,
//!                 modified_date: None,
//!                 tags: vec!["Tag3".into()]
//!             },
//!         ]
//!     });
//! }
//! ```
//!
//! # Built-in supported types
//!
//! By default, you can use the following types as a field value of `FromHtml`-derived structs.
//!
//! - Basic types
//!   - `String`
//!   - Numeric types ( `usize`, `i64`, `NonZeroU32`, ... )
//!   - [And more](./core/src/from_text.rs)
//! - Container types (where `T` is another supported type)
//!   - `[T;N]`
//!   - `Option<T>`
//!   - `Vec<T>`
//! - Any struct that is derived/implemented `FromHtml` trait
//! - Any struct that is implemented `FromText` trait
//!
//! ## Use custom types
//!
//! You can also use custom types by implementing `FromHtml` or `FromText` trait yourself.
//! In many cases for a simple value, [implementing `FromText`](./examples/from_text_custom.rs) can be enough.
//! Please see also [examples](./examples).

pub use h2s_core::*;
pub use h2s_macro::*;
