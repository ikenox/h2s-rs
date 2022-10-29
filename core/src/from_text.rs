use std::ffi::OsString;
use std::fmt::{Debug, Display};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};
use std::path::PathBuf;
use std::str::FromStr;

// TODO cannot implement into third party structs by users
pub trait FromText: Sized {
    type Error: Display + Debug + 'static;
    fn from_text(s: &str) -> Result<Self, Self::Error>;
}

mod impls {
    use super::*;

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
}
