pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).expect("could not write to the buffer");
        }
    }
}

#[test]
fn find_matches_test() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndoctor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
