use once_cell::sync::Lazy;
use std::ops::Deref;

pub trait EpsilonProvider {
    fn diff() -> f64;
}

static SHOULDA_EPSILON: Lazy<f64> = Lazy::new(|| {
    option_env!("SHOULDA_EPSILON")
        .map(|x| x.parse().unwrap())
        .unwrap_or(0.0001)
});

pub struct EnvEpsilonProvider;

impl EpsilonProvider for EnvEpsilonProvider {
    fn diff() -> f64 {
        *SHOULDA_EPSILON.deref()
    }
}

/// since only integers are stable as const generics, this accepts an integer that will be std::mem::transmuted to an f64 before being used
/// once const generic floats are stabilized this will change to const N: f64
pub struct ConstEpsilonProvider<const N: u64>;

impl<const N: u64> EpsilonProvider for ConstEpsilonProvider<N> {
    fn diff() -> f64 {
        unsafe { std::mem::transmute(N) }
    }
}
