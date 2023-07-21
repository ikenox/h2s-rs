//! Implementations of `FromHtml` trait

use crate::extraction_method::{ExtractInnerText, ExtractionMethod, NoOp};
use crate::html::HtmlElement;
use crate::Error;
use crate::FromHtml;

pub trait Parseable: Sized {
    type Input<N: HtmlElement>: ExtractedValue;
    type Error: Error;

    fn parse<N: HtmlElement>(input: Self::Input<N>) -> Result<Self, Self::Error>;
}

impl<T: FromHtml> Parseable for T {
    type Input<N: HtmlElement> = N;
    type Error = T::Error;

    fn parse<N: HtmlElement>(input: Self::Input<N>) -> Result<Self, Self::Error> {
        Self::from_html(input)
    }
}

macro_rules! impl_parseable {
        ($($t:ty),*) => {
            $(
                impl Parseable for $t {
                    type Input<N: HtmlElement> = String;
                    type Error = <$t as ::std::str::FromStr>::Err;

                    fn parse<N: HtmlElement>(input: String) -> Result<Self, Self::Error> {
                        input.parse()
                    }
                }
            )*
        };
    }

impl_parseable!(
    String,
    // primitives
    bool,
    char,
    usize,
    u8,
    u16,
    u32,
    u64,
    u128,
    isize,
    i8,
    i16,
    i32,
    i64,
    i128,
    f32,
    f64,
    // non-zero numerics
    std::num::NonZeroU8,
    std::num::NonZeroU16,
    std::num::NonZeroU32,
    std::num::NonZeroU64,
    std::num::NonZeroU128,
    std::num::NonZeroUsize,
    std::num::NonZeroI8,
    std::num::NonZeroI16,
    std::num::NonZeroI32,
    std::num::NonZeroI64,
    std::num::NonZeroI128,
    std::num::NonZeroIsize,
    // misc that implements FromStr in the standard library
    std::path::PathBuf,
    std::net::IpAddr,
    std::net::Ipv4Addr,
    std::net::Ipv6Addr,
    std::net::SocketAddr,
    std::net::SocketAddrV4,
    std::net::SocketAddrV6,
    std::ffi::OsString
);

pub trait ExtractedValue {
    type Default: ExtractionMethod;
    fn default_method() -> Self::Default;
}

impl<N: HtmlElement> ExtractedValue for N {
    type Default = NoOp;
    fn default_method() -> Self::Default {
        NoOp
    }
}

impl ExtractedValue for String {
    type Default = ExtractInnerText;

    fn default_method() -> Self::Default {
        ExtractInnerText
    }
}
