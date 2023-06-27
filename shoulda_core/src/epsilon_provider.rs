use const_env::from_env;

pub trait EpsilonProvider {
    fn diff() -> f64;
}

#[from_env]
const SHOULDA_EPSILON: f64 = 0.0001;

pub struct EnvEpsilonProvider;

impl EpsilonProvider for EnvEpsilonProvider {
    fn diff() -> f64 {
        SHOULDA_EPSILON
    }
}

/// since only integers are stable as const generics, this accepts an integer that will be std::mem::transmuted to an f64 before being used
/// once const generic floats are stabilized this will change to const N: f64
pub struct ConstEpsilonProvider<const N: u64>;

impl<const N: u64> EpsilonProvider for ConstEpsilonProvider<N> {
    fn diff() -> f64 {
        f64::from_bits(N)
    }
}
