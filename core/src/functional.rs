pub struct Identity<T>(pub T);

pub trait IsSelf {}

impl<F> IsSelf for F where F: Functor<This<<Self as Functor>::Inner> = Self> {}

pub trait Functor: IsSelf {
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

pub trait Monad: Applicative {
    fn bind<B, F>(self, f: F) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> Self::This<B>;
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

impl<T> Monad for Identity<T> {
    fn bind<B, F>(self, f: F) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> Self::This<B>,
    {
        f(self.0)
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

impl<T> Monad for Option<T> {
    fn bind<B, F>(self, f: F) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> Self::This<B>,
    {
        self.and_then(f)
    }
}

impl<T, E> Functor for Result<T, E> {
    type Inner = T;
    type This<A> = Result<A, E>;

    fn fmap<B, F>(self, f: F) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> B,
    {
        self.bind(|a| <Self::This<B>>::pure(f(a)))
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

impl<T, E> Monad for Result<T, E> {
    fn bind<B, F>(self, f: F) -> Self::This<B>
    where
        F: FnOnce(Self::Inner) -> Self::This<B>,
    {
        self.and_then(f)
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

impl<T> Monad for Vec<T>
where
    T: Clone,
{
    fn bind<B, F>(self, f: F) -> Self::This<B>
    where
        F: Fn(Self::Inner) -> Self::This<B>,
    {
        self.into_iter().flat_map(f).collect()
    }
}

pub trait Foldable {}

pub trait Traversable: Functor + Foldable {
    fn traverse<A, F>(self, f: F) -> A::This<Self::This<A::Inner>>
    where
        A: Applicative,
        F: Fn(Self::Inner) -> A;
}
