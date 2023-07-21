use crate::functor::{ExactlyOne, Functor};
use crate::html::{CssSelector, HtmlElement};
use std::fmt::{Debug, Display};

pub trait TargetElementSelector: Debug + Display {
    type Output<E>: Functor<Inner = E>;
    fn select<E>(&self, n: &E) -> Self::Output<E>
    where
        E: HtmlElement;
}

#[derive(Debug)]
pub struct Select {
    pub selector: String,
}

impl TargetElementSelector for Select {
    type Output<E> = Vec<E>;

    fn select<E>(&self, n: &E) -> Self::Output<E>
    where
        E: HtmlElement,
    {
        // TODO cache selector
        let selector = E::Selector::parse(&self.selector).unwrap();
        n.select(&selector)
    }
}

#[derive(Debug)]
pub struct Root;

impl TargetElementSelector for Root {
    type Output<E> = ExactlyOne<E>;

    fn select<E>(&self, n: &E) -> Self::Output<E>
    where
        E: HtmlElement,
    {
        ExactlyOne(n.clone())
    }
}
