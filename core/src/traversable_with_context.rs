use crate::functor::{ExactlyOne, Functor};
use std::fmt::{Debug, Display};

pub trait FunctorWithContext<T>: Functor<T> {
    type Context: Context;

    fn fmap_with_context<A, B, F>(a: Self::Structure<A>, f: F) -> Self::Structure<B>
    where
        F: Fn(Self::Context, A) -> B;
}

impl<T> FunctorWithContext<T> for ExactlyOne<T> {
    type Context = NoContext;

    fn fmap_with_context<A, B, F>(a: Self::Structure<A>, f: F) -> Self::Structure<B>
    where
        F: Fn(Self::Context, A) -> B,
    {
        Self::fmap(a, |a| f(NoContext, a))
    }
}

impl<T> FunctorWithContext<T> for Option<T> {
    type Context = NoContext;

    fn fmap_with_context<A, B, F>(a: Self::Structure<A>, f: F) -> Self::Structure<B>
    where
        F: Fn(Self::Context, A) -> B,
    {
        Self::fmap(a, |a| f(NoContext, a))
    }
}

impl<T> FunctorWithContext<T> for Vec<T> {
    type Context = ListIndex;

    fn fmap_with_context<A, B, F>(a: Self::Structure<A>, f: F) -> Self::Structure<B>
    where
        F: Fn(Self::Context, A) -> B,
    {
        Self::fmap(a.into_iter().enumerate().collect(), |(i, v)| {
            f(ListIndex(i), v)
        })
    }
}

impl<T, const M: usize> FunctorWithContext<T> for [T; M] {
    type Context = ListIndex;

    fn fmap_with_context<A, B, F>(a: Self::Structure<A>, f: F) -> Self::Structure<B>
    where
        F: Fn(Self::Context, A) -> B,
    {
        // TODO fix inefficient conversion
        Self::fmap(
            a.into_iter()
                .enumerate()
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| "")
                .unwrap(), // never failed
            |(i, v)| f(ListIndex(i), v),
        )
    }
}

pub trait Context: Debug + Display {}

#[derive(Debug)]
pub struct NoContext;

impl Context for NoContext {}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ListIndex(pub usize);

impl Context for ListIndex {}
