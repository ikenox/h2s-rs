#![feature(generic_associated_types)]

use h2s::FromHtml;

fn main() {
    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct Page {
        #[h2s(attr = "lang")]
        lang: String,
        #[h2s(select = "h1.blog-title")]
        blog_title: String,
        #[h2s(select = ".articles > div", attr = "data-author")]
        article_authors: Vec<String>,
        #[h2s(select = ".articles > div")]
        articles_as_vec: Vec<ArticleElem>,
        #[h2s(select = ".articles > div")]
        articles_as_array: [ArticleElem; 3],
        #[h2s(select = "footer")]
        footer: Footer,
    }
    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct ArticleElem {
        #[h2s(attr = "data-author")]
        author: String,
        #[h2s(select = "h2")]
        title: String,
        #[h2s(select = "p.modified-date")]
        modified_date: Option<String>,
        #[h2s(select = ".foo > div", attr = "data-foobar")]
        foobar: Option<String>,
    }

    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct Footer {
        #[h2s]
        txt: String,
    }

    let html = r#"
<!DOCTYPE html>
<html lang="en">
<body>
<h1 class="blog-title">My tech blog</h1>
<div class="articles">
    <div data-author="Alice">
        <h2>article1</h2>
    </div>
    <div data-author="Bob">
        <h2>article2</h2>
        <p class="modified-date">2020-05-01</p>
    </div>
    <div data-author="Ikeno">
        <h2>article3</h2>
    </div>
</div>
<footer>foo</footer>
</body>
</html>
    "#;

    let res = h2s::utils::parse::<Page>(html);
    match res {
        Ok(p) => {
            println!("{:#?}", p);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
