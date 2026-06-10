//! # Ogum
//!
//! Named after Ogum, the Yoruba orisha of iron and technology —
//! the one who cleared the path through the forest so all others could pass.
//!
//! Ogum is a meta-framework for building web servers in Rust,
//! where any frontend architecture is welcome.
//!
//! **This crate is under active development. The API is not stable yet.**

/// Returns the current version of the crate, as defined in `Cargo.toml`.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
