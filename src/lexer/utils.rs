impl std::fmt::Debug for crate::token::Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\nToken type: {:?}, Token value: {:?}",
            self.token_type, self.token_value
        )
    }
}
