use crate::Tuple;

pub trait SelfConstraint {}

impl<F> SelfConstraint for F where F: Functor<Structure<<Self as Functor>::Inner> = Self> {}

pub trait Functor: SelfConstraint {
    type Inner;
    type Structure<A>: Functor<Inner = A>;

    fn fmap<A, B, F>(a: Self::Structure<A>, f: F) -> Self::Structure<B>
    where
        F: Fn(A) -> B;
}

impl<T> Functor for ExactlyOne<T> {
    type Inner = T;
    type Structure<A> = ExactlyOne<A>;

    fn fmap<A, B, F>(a: Self::Structure<A>, f: F) -> Self::Structure<B>
    where
        F: Fn(A) -> B,
    {
        ExactlyOne(f(a.0))
    }
}

impl<T> Functor for Option<T> {
    type Inner = T;
    type Structure<A> = Option<A>;

    fn fmap<A, B, F>(a: Self::Structure<A>, f: F) -> Self::Structure<B>
    where
        F: Fn(A) -> B,
    {
        a.map(f)
    }
}

impl<T> Functor for Vec<T> {
    type Inner = T;
    type Structure<U> = Vec<U>;

    fn fmap<A, B, F>(a: Self::Structure<A>, f: F) -> Self::Structure<B>
    where
        F: Fn(A) -> B,
    {
        a.into_iter().map(f).collect()
    }
}

impl<T, const M: usize> Functor for [T; M] {
    type Inner = T;
    type Structure<U> = [U; M];

    fn fmap<A, B, F>(a: Self::Structure<A>, f: F) -> Self::Structure<B>
    where
        F: Fn(A) -> B,
    {
        a.map(f)
    }
}

impl<T, U> Functor for Tuple<T, U> {
    type Inner = U;
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
