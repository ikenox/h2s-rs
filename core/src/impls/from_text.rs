use crate::impls::from_html::{FromText, FromTextError};
use std::char::ParseCharError;
use std::convert::Infallible;
use std::ffi::OsString;
use std::net::{
    AddrParseError, IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6,
};
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, ParseFloatError, ParseIntError,
};
use std::path::PathBuf;
use std::str::{FromStr, ParseBoolError};

macro_rules! impl_from_text {
    ($($t:ty),*) => {
        $(
            impl FromText for $t {
                type Error = <$t as FromStr>::Err;

                fn from_text(s: &str) -> Result<Self, Self::Error> {
                    s.parse()
                }
            }
        )*
    };
}

impl_from_text!(
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
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64,
    NonZeroU128,
    NonZeroUsize,
    NonZeroI8,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64,
    NonZeroI128,
    NonZeroIsize,
    // misc that implements FromStr in the standard library
    PathBuf,
    IpAddr,
    Ipv4Addr,
    Ipv6Addr,
    SocketAddr,
    SocketAddrV4,
    SocketAddrV6,
    OsString
);

impl FromTextError for Infallible {}
impl FromTextError for AddrParseError {}
impl FromTextError for ParseFloatError {}
impl FromTextError for ParseBoolError {}
impl FromTextError for ParseCharError {}
impl FromTextError for ParseIntError {}
