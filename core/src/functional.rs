use crate::FromHtml;

pub struct Identity<T>(pub T);

pub trait ThisConstraint {}

impl<F> ThisConstraint for F where F: Functor<This<<Self as Functor>::Inner> = Self> {}

pub trait Functor: ThisConstraint + Sized {
    type Inner;
    type This<A>: Functor<Inner = A>;
    fn fmap<B, F>(self, f: F) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> B;
}

pub trait Applicative: Functor {
    fn pure(inner: Self::Inner) -> Self;
    fn ap<B, F>(self, f: Self::This<F>) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> B;
}

impl<T> Functor for Identity<T> {
    type Inner = T;
    type This<A> = Identity<A>;

    fn fmap<B, F>(self, f: F) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> B,
    {
        Identity(f(self.0))
    }
}

impl<T> Applicative for Identity<T> {
    fn pure(inner: Self::Inner) -> Self {
        Identity(inner)
    }

    fn ap<B, F>(self, f: Self::This<F>) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> B,
    {
        todo!()
    }
}

impl<T> Functor for Option<T> {
    type Inner = T;
    type This<A> = Option<A>;

    fn fmap<B, F>(self, f: F) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> B,
    {
        self.map(f)
    }
}

impl<T> Applicative for Option<T> {
    fn pure(inner: Self::Inner) -> Self {
        Some(inner)
    }

    fn ap<B, F>(self, f: Self::This<F>) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> B,
    {
        self.and_then(|a| f.map(|f| f(a)))
    }
}

impl<T, E> Functor for Result<T, E> {
    type Inner = T;
    type This<A> = Result<A, E>;

    fn fmap<B, F>(self, f: F) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> B,
    {
        self.map(f)
    }
}

impl<T, E> Applicative for Result<T, E> {
    fn pure(inner: Self::Inner) -> Self {
        Ok(inner)
    }

    fn ap<B, F>(self, f: Self::This<F>) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> B,
    {
        self.and_then(|a| f.map(|f| f(a)))
    }
}

impl<T> Functor for Vec<T> {
    type Inner = T;
    type This<A> = Vec<A>;

    fn fmap<B, F>(self, f: F) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<T, const N: usize> Functor for [T; N] {
    type Inner = T;
    type This<A> = [A; N];

    fn fmap<B, F>(self, f: F) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> B,
    {
        self.map(f)
    }
}

impl<T> Applicative for Vec<T>
where
    T: Clone,
{
    fn pure(inner: Self::Inner) -> Self {
        vec![inner]
    }

    fn ap<B, F>(self, f: Self::This<F>) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> B,
    {
        self.into_iter()
            .flat_map(|a| f.iter().map(move |f| f(a.clone())))
            .collect()
    }
}

pub trait Traversable: Functor {
    fn traverse<A, F>(self, f: F) -> A::This<Self::This<A::Inner>>
    where
        A: Applicative,
        F: Fn(Self::Inner) -> A;
}

impl<T> Traversable for Vec<T> {
    fn traverse<A, F>(self, f: F) -> A::This<Self::This<A::Inner>>
    where
        A: Applicative,
        F: Fn(Self::Inner) -> A,
    {
        todo!()
    }
}

pub trait FieldValue {
    type Type: FromHtml;
    type Wrapped: Functor<Inner = Self::Type>;

    fn unwrap(wrapped: Self::Wrapped) -> Self;
}

impl<T: FromHtml> FieldValue for T {
    type Type = T;
    // todo note why use identity
    type Wrapped = Identity<T>;
    fn unwrap(wrapped: Self::Wrapped) -> Self {
        wrapped.0
    }
}

impl<T: FromHtml> FieldValue for Vec<T> {
    type Type = T;
    type Wrapped = Self;
    fn unwrap(wrapped: Self::Wrapped) -> Self {
        wrapped
    }
}

impl<T: FromHtml> FieldValue for Option<T> {
    type Type = T;
    type Wrapped = Self;
    fn unwrap(wrapped: Self::Wrapped) -> Self {
        wrapped
    }
}

impl<T: FromHtml, const N: usize> FieldValue for [T; N] {
    type Type = T;
    type Wrapped = Self;
    fn unwrap(wrapped: Self::Wrapped) -> Self {
        wrapped
    }
}
