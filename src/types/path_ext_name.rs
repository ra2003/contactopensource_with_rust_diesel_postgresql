//! File name extension.
//!
//! Examples: ".txt", ".jpg", ".html".

use ::rand::{thread_rng, seq::SliceRandom};
use crate::types::{text as supertype, text::Text as Supertype};

pub type PathExtName = Supertype;
type T = PathExtName;

pub fn from_option_str(s: Option<&str>) -> Option<T> {
    supertype::from_option_str(s)
}

lazy_static! {
    static ref EXAMPLE_VEC: Vec<T> = {
        vec![
            T::from("txt"), // Text
            T::from("jpg"), // JPG image
            T::from("html"), // Hyper-Text Markup Language
            T::from("json"), // JavaScript Object Notation
            T::from("csv"), // Comma Separated Value
            T::from("tsv"), // Tab Separated Value
        ]
    };
}

pub fn fab() -> T {
    EXAMPLE_VEC.choose(&mut thread_rng()).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use crate::types::{path_ext_name as t, path_ext_name::PathExtName as T};

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
