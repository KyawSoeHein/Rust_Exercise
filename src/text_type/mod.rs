pub fn print_text(text: &str) -> &str {
    return text;
}

#[cfg(test)]
mod tests {
    use crate::text_type::print_text;

    #[test]
    fn test_if_correct_str() {
        assert_eq!(print_text("Hello"), "Hello");
    }

    #[test]
    fn test_if_correct_string() {
        assert_eq!(print_text(&String::from("Hello")), "Hello");
    }
}
