use crate::Tuple;

pub trait SelfConstraint<T> {}
impl<F, T> SelfConstraint<T> for F where F: Functor<T, Structure<T> = Self> {}

pub trait Functor<T>: SelfConstraint<T> {
    type Structure<A>: Functor<A>;

    fn fmap<A, B, F>(a: Self::Structure<A>, f: F) -> Self::Structure<B>
    where
        F: Fn(A) -> B;
}

impl<T> Functor<T> for ExactlyOne<T> {
    type Structure<A> = ExactlyOne<A>;

    fn fmap<A, B, F>(a: Self::Structure<A>, f: F) -> Self::Structure<B>
    where
        F: Fn(A) -> B,
    {
        ExactlyOne(f(a.0))
    }
}

impl<T> Functor<T> for Option<T> {
    type Structure<A> = Option<A>;

    fn fmap<A, B, F>(a: Self::Structure<A>, f: F) -> Self::Structure<B>
    where
        F: Fn(A) -> B,
    {
        a.map(f)
    }
}

impl<T> Functor<T> for Vec<T> {
    type Structure<U> = Vec<U>;

    fn fmap<A, B, F>(a: Self::Structure<A>, f: F) -> Self::Structure<B>
    where
        F: Fn(A) -> B,
    {
        a.into_iter().map(f).collect()
    }
}

impl<T, const M: usize> Functor<T> for [T; M] {
    type Structure<U> = [U; M];

    fn fmap<A, B, F>(a: Self::Structure<A>, f: F) -> Self::Structure<B>
    where
        F: Fn(A) -> B,
    {
        a.map(f)
    }
}

impl<T, U> Functor<U> for Tuple<T, U> {
    type Structure<A> = Tuple<T, A>;

    fn fmap<A, B, F>(a: Self::Structure<A>, f: F) -> Self::Structure<B>
    where
        F: Fn(A) -> B,
    {
        Tuple(a.0, f(a.1))
    }
}

/// Similar to Identity Monad in functional programming languages
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExactlyOne<T>(pub T);
