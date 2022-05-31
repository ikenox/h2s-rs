#![feature(generic_associated_types)]

#[test]
fn derive_from_html() {
    use h2s::FromHtml;

    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct Page {
        #[h2s(attr = "lang")]
        lang: String,
        #[h2s(select = "h1.blog-title")]
        blog_title: String,
        #[h2s(select = ".articles > div", attr = "data-author")]
        article_authors: Vec<String>,
        #[h2s(select = ".articles > div > h2")]
        article_titles: Vec<String>,
        #[h2s(select = ".articles > div")]
        articles: Vec<ArticleElem>,
        #[h2s(select = "footer")]
        footer: Footer,
    }

    #[derive(FromHtml, Debug, Eq, PartialEq)]
    pub struct ArticleElem {
        #[h2s(select = "h2")]
        title: String,
        #[h2s(select = "p.modified-date")]
        modified_date: Option<String>,
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
<footer>this is footer</footer>
</body>
</html>
    "#;

    let res = h2s::utils::parse::<Page>(html);
    assert_eq!(
        res,
        Ok(Page {
            lang: "en".to_string(),
            blog_title: "My tech blog".to_string(),
            article_authors: vec!["Alice", "Bob", "Ikeno"]
                .iter()
                .map(|a| a.to_string())
                .collect(),
            article_titles: vec!["article1", "article2", "article3"]
                .iter()
                .map(|a| a.to_string())
                .collect(),
            articles: vec![
                ArticleElem {
                    title: "article1".to_string(),
                    modified_date: None,
                },
                ArticleElem {
                    title: "article2".to_string(),
                    modified_date: Some("2020-05-01".to_string()),
                },
                ArticleElem {
                    title: "article3".to_string(),
                    modified_date: None,
                }
            ],
            footer: Footer {
                txt: "this is footer".to_string()
            }
        })
    )
}
