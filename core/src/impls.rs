use crate::{HtmlNodeRef, Selector};

impl Selector for scraper::Selector {
    fn parse<S: AsRef<str>>(s: S) -> Result<Self, String> {
        scraper::Selector::parse(s.as_ref())
            .map_err(|_| format!("failed to parse css selector `{}`", s.as_ref()))
    }
}

impl<'a> HtmlNodeRef for scraper::ElementRef<'a> {
    type Selector = scraper::Selector;

    fn select(&self, sel: &Self::Selector) -> Vec<Self> {
        scraper::ElementRef::<'a>::select(self, sel).collect()
    }

    fn text_contents(&self) -> String {
        self.text().collect::<String>()
    }

    fn get_attribute<S: AsRef<str>>(&self, attr: S) -> Option<&str> {
        self.value().attr(attr.as_ref())
    }
}
