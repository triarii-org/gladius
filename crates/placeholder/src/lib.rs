fn hello() -> &'static str {
    "repo!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn says_hello() {
        assert_eq!("repo!", hello())
    }
}
