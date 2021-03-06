//! Typical label for a user interface.
//!
//! Example: "alpha label" is a typical label.

use crate::types::{self, text as supertype, text::Text as Supertype};

pub type Label = Supertype;
type T = Label;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

pub fn fab() -> T {
    T::from(format!(
        "{} label",
        types::alpha_word::fab()
        ))
}

#[cfg(test)]
mod tests {
    use crate::types::{label as t, label::Label as T};

    #[test]
    fn test_from_str() {
        let s = "alpha";
        let x: T = T::from(s);
        assert_eq!(x, s);
    }

    #[test]
    fn test_from_option_str() {
        let s = "alpha";
        let x: T = t::from_option_str(Some(s)).unwrap();
        assert_eq!(x, s);
    }

    #[test]
    fn test_fab() {
        let x: T = t::fab();
        assert!(!x.is_empty());
    }

}
