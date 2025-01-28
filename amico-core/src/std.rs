pub use amico_std::*;

#[cfg(test)]
mod tests {
    #[test]
    fn visit_std_api() {
        assert_eq!(crate::std::add(2, 2), 4);
    }
}
