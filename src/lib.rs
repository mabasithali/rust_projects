pub fn compute_status() -> &'static str {
    "Rust workspace initialized."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_status() {
        assert_eq!(compute_status(), "Rust workspace initialized.");
    }
}

