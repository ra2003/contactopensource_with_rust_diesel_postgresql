//! Typical geography latitude, using decimal degrees (DD).
//!
//! Example: TODO

use crate::types::{metrics as supertype, metrics::Metrics as Supertype};

pub type Latitude = Supertype;
type T = Latitude;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    supertype::fab() //TODO
}

#[cfg(test)]
mod tests {
    use crate::types::{latitude as t, latitude::Latitude as T};
    use ::bigdecimal::Zero;
    use ::std::str::FromStr;

    #[test]
    fn test_from_str() {
        let s = "0";
        let e = T::zero();
        let x: T = T::from_str(&s).unwrap();
        assert_eq!(x, e);
    }

    #[test]
    fn test_from_option_str() {
        let s = "0";
        let e = T::zero();
        let x: T = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, e);
    }

    #[test]
    fn test_fab() {
        let _x: T = t::fab();
    }

}
