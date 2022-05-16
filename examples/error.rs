#![feature(generic_associated_types)]

use h2s::FromHtml;

fn main() {
    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct Page {
        #[h2s(select = "html", attr = "lang")]
        lang: String,
        #[h2s(select = "h1.blog-title", text)]
        blog_title: String,
        #[h2s(select = ".articles > div", attr = "data-author")]
        article_authors: Vec<String>,
        #[h2s(select = ".articles > div")]
        articles: Vec<ArticleElem>,
    }
    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct ArticleElem {
        #[h2s(attr = "data-author", text)]
        author: String,
        #[h2s(select = "h2", text)]
        title: String,
        #[h2s(select = "p.modified-date", text)]
        modified_date: Option<String>,
        #[h2s(select = ".foo > div", attr = "data-foobar")]
        foobar: String,
    }

    let html = r#"
<!DOCTYPE html>
<html lang="en">
<body>
<h1 class="blog-title">My tech blog</h1>
<div class="articles">
    <div data-author="Alice">
        <h2>article1</h2>
        <div class="foo"><div data-foobar="aiu">aaaaaa</div></div>
    </div>
    <div data-author="Bob">
        <h2>article2</h2>
        <p class="modified-date">2020-05-01</p>
        <div class="foo"><div data-foobar="aiu">aaaaaa</div></div>
    </div>
    <div data-author="Ikeno">
        <h2>article3</h2>
        <div class="foo"><div>aaaaaa</div></div>
    </div>
</div>
</body>
</html>
    "#;

    let res = h2s::utils::extract_from_html::<Page>(html);
    match res {
        Ok(p) => {
            println!("{:#?}", p);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
