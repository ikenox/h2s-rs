//! A set of internal utility methods that will be used in the auto-generated code on `FromHtml` derive macro.
//! These methods are shorthands to reduce codes in the `quote!` macro and improve development experience.
//! If you are just a h2s user, you wouldn't call these methods directly.

use crate::element_selector::TargetElementSelector;
use crate::extraction_method::ExtractionMethod;
use crate::field_value::FieldValue;
use crate::html::HtmlElement;
use crate::parseable::{ExtractedValue, Parseable};
use crate::transformable::TransformableFrom;
use crate::traversable::Traversable;
use crate::traversable_with_context::{Context, FunctorWithContext};
use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;

/// Process the source HTML element into the specified field value
#[allow(clippy::type_complexity)]
pub fn process_field<E, S, T, M, V, W, P, I>(
    source_element: &E,
    target_element_selector: S,
    // By surrounding extraction method value with `ExtractionMethodWithType`, a caller of this
    // function can be empowered by type inference for a type of field value
    ExtractionMethodWithType(extraction_method, _): ExtractionMethodWithType<V, M>,
) -> Result<
    V,
    ProcessError<
        TransformError<S, <W::Structure<E> as TransformableFrom<S::Output<E>>>::Error>,
        ExtractionError<W::Context, M>,
        ParseError<W::Context, P::Error>,
    >,
>
where
    E: HtmlElement,
    S: TargetElementSelector<Output<E> = T>,
    W::Structure<E>: TransformableFrom<S::Output<E>>,
    M: ExtractionMethod<ExtractedValue<E> = I>,
    V: FieldValue<Wrapped = W, Inner = P>,
    P: Parseable<Input<E> = I>,
    I: ExtractedValue,
    W: FunctorWithContext<P> + Traversable<P>,
{
    let target_elements = target_element_selector.select(source_element);
    let transformed = <_>::try_transform_from(target_elements)
        .map_err(|error| TransformError {
            selector: target_element_selector,
            error,
        })
        .map_err(ProcessError::TransformError)?;
    let with_context = W::fmap_with_context(transformed, |ctx, a: E| (ctx, a));
    let extracted = W::traverse(with_context, |(ctx, a)| {
        match extraction_method.extract(a) {
            Ok(a) => Ok((ctx, a)),
            Err(e) => Err((ctx, e)),
        }
    })
    .map_err(|(ctx, e)| ExtractionError {
        extraction_method,
        context: ctx,
        error: e,
    })
    .map_err(ProcessError::ExtractionError)?;
    let parsed = W::traverse(extracted, |(ctx, a)| {
        P::parse::<E>(a).map_err(|error| ParseError {
            context: ctx,
            error,
        })
    })
    .map_err(ProcessError::ParseError)?;
    Ok(V::finalize(parsed))
}

pub struct ExtractionMethodWithType<V, E>(E, PhantomData<V>);

pub fn extraction_method<V, E>(e: E) -> ExtractionMethodWithType<V, E> {
    ExtractionMethodWithType(e, PhantomData)
}

pub fn default_extraction_method<N: HtmlElement, V>(
) -> ExtractionMethodWithType<V, <<V::Inner as Parseable>::Input<N> as ExtractedValue>::Default>
where
    V: FieldValue,
{
    ExtractionMethodWithType(
        <<V::Inner as Parseable>::Input<N> as ExtractedValue>::default_method(),
        PhantomData,
    )
}

#[derive(Debug)]
pub enum ProcessError<A, B, C> {
    TransformError(A),
    ExtractionError(B),
    ParseError(C),
}

#[derive(Debug, Clone)]
pub struct TransformError<S, E>
where
    S: TargetElementSelector,
    E: Error,
{
    pub selector: S,
    pub error: E,
}

#[derive(Debug, Clone)]
pub struct ExtractionError<C, M>
where
    C: Context,
    M: ExtractionMethod,
{
    pub context: C,
    pub extraction_method: M,
    pub error: M::Error,
}

#[derive(Debug, Clone)]
pub struct ParseError<C, E>
where
    C: Context,
    E: Error,
{
    pub context: C,
    pub error: E,
}
