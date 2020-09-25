// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64 / 31557600.0)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! orbital {
    ($struct_name:ident, $orbital:expr) => {
        pub struct $struct_name;
        impl Planet for $struct_name {
            fn years_during(d: &Duration) -> f64 {
                d.0 / $orbital
            }
        }
    }
}

orbital!(Mercury, 0.2408467);
orbital!(Venus, 0.61519726);
orbital!(Earth, 1.0);
orbital!(Mars, 1.8808158);
orbital!(Jupiter, 11.862615);
orbital!(Saturn, 29.447498);
orbital!(Uranus, 84.016846);
orbital!(Neptune, 164.79132);