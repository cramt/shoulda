use crate::epsilon_provider::EpsilonProvider;

pub trait ShouldaEqual {
    fn should_eq<Epsilon: EpsilonProvider>(&self, other: &Self) -> bool;
}

macro_rules! eq_assertable_impl {
    ($x:ty) => {
        impl ShouldaEqual for $x {
            fn should_eq<Epsilon: EpsilonProvider>(&self, other: &Self) -> bool {
                self.eq(other)
            }
        }
    };
}

macro_rules! float_assertable_impl {
    ($x:ty) => {
        impl ShouldaEqual for $x {
            fn should_eq<Epsilon: EpsilonProvider>(&self, other: &Self) -> bool {
                (self - other).abs() < (Epsilon::diff() as $x)
            }
        }
    };
}

eq_assertable_impl!(String);
eq_assertable_impl!(str);
eq_assertable_impl!(std::ffi::CString);
eq_assertable_impl!(std::ffi::CStr);
eq_assertable_impl!(std::ffi::OsString);
eq_assertable_impl!(std::ffi::OsStr);
eq_assertable_impl!(std::fs::FileType);
eq_assertable_impl!(std::fs::Permissions);
eq_assertable_impl!(std::net::Ipv4Addr);
eq_assertable_impl!(std::net::Ipv6Addr);
eq_assertable_impl!(std::net::SocketAddrV4);
eq_assertable_impl!(std::net::SocketAddrV6);
eq_assertable_impl!(std::path::Path);
eq_assertable_impl!(std::path::PathBuf);
eq_assertable_impl!(std::thread::ThreadId);
eq_assertable_impl!(std::time::Duration);
eq_assertable_impl!(std::time::Instant);
eq_assertable_impl!(std::time::SystemTime);

eq_assertable_impl!(bool);
eq_assertable_impl!(u8);
eq_assertable_impl!(i8);
eq_assertable_impl!(u16);
eq_assertable_impl!(i16);
eq_assertable_impl!(u32);
eq_assertable_impl!(i32);
eq_assertable_impl!(u64);
eq_assertable_impl!(i64);
eq_assertable_impl!(u128);
eq_assertable_impl!(i128);
eq_assertable_impl!(usize);
eq_assertable_impl!(isize);

eq_assertable_impl!(std::num::NonZeroU8);
eq_assertable_impl!(std::num::NonZeroI8);
eq_assertable_impl!(std::num::NonZeroU16);
eq_assertable_impl!(std::num::NonZeroI16);
eq_assertable_impl!(std::num::NonZeroU32);
eq_assertable_impl!(std::num::NonZeroI32);
eq_assertable_impl!(std::num::NonZeroU64);
eq_assertable_impl!(std::num::NonZeroI64);
eq_assertable_impl!(std::num::NonZeroU128);
eq_assertable_impl!(std::num::NonZeroI128);
eq_assertable_impl!(std::num::NonZeroUsize);
eq_assertable_impl!(std::num::NonZeroIsize);

float_assertable_impl!(f32);
float_assertable_impl!(f64);

impl<T> ShouldaEqual for &T
where
    T: ShouldaEqual,
{
    fn should_eq<Epsilon: EpsilonProvider>(&self, other: &Self) -> bool {
        T::should_eq::<Epsilon>(self, other)
    }
}
