#[cfg(feature = "std")]
pub use amico_std::*;

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    #[test]
    fn visit_std_api() {
        assert_eq!(crate::std::add(2, 2), 4);
    }
}
