use once_cell::sync::Lazy;
use std::ops::Deref;

pub trait FloatDiffProvider {
    fn diff() -> f64;
}

static SHOULDA_FLOAT_DIFF_MODE: Lazy<f64> = Lazy::new(|| {
    option_env!("SHOULDA_FLOAT_DIFF_MODE")
        .map(|x| x.parse().unwrap())
        .unwrap_or(0.0001)
});

pub struct EnvFloatDiffProvider;

impl FloatDiffProvider for EnvFloatDiffProvider {
    fn diff() -> f64 {
        *SHOULDA_FLOAT_DIFF_MODE.deref()
    }
}

/// since only integers are stable as const generics, this accepts an integer that will be std::mem::transmuted to an f64 before being used
/// once const generic floats are stabilized this will change to const N: f64
pub struct ConstFloatDiffProvider<const N: u64>;

impl<const N: u64> FloatDiffProvider for ConstFloatDiffProvider<N> {
    fn diff() -> f64 {
        unsafe { std::mem::transmute(N) }
    }
}
